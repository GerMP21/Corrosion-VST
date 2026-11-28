# Design

## Context

See proposal.md for why. The two copies differ in exactly these places (checked on `dev`, 2026-11-28):

**Resonator** (`core.rs:1689-1764` vs `:1766-1850`):
- per mode, mono adds `mode_output` to one sum; stereo pans it: `pan = clamp(width · index/mode_count · (±1 alternating))`, gains `0.5·(1 ∓ pan)` into two sums;
- stereo returns `(0, 0)` early, with `prepared = false` and no preparation, when the bank is empty; mono runs the preparation and returns 0;
- mono ends with a `match self.profile` whose every arm is `mode_sum` (dead, N1).

**Voice** (`voice/mod.rs:1832-1889` vs `:1908-1987`):
- NaN/denormal guard per channel;
- the rattle and the high-pass boost run on the mono signal (mono path) or on the mid `(l + r)/2` (stereo path), and the boost is added to each channel;
- the pre-clamp peak, clamp, gain, flush and peak hold run per channel (`max` of both for the peaks);
- the exciter call goes through `process_exciter_stereo(width)`, a pass-through (N4).

Everything else is the same code twice. CLAUDE.md's hot-loop rules apply: the mode loop runs for every mode of 16 voices every sample, so the merged code must not add a branch or data to it.

## Goals / Non-Goals

**Goals:**
- One pipeline per layer, both entry points bit-identical to today.
- The plugin's stereo path covered by the benchmarks.

**Non-Goals:**
- Moving the offline renderer, the tests or the benchmarks from mono to stereo. The mono path stays a supported, identical-sounding API.
- Any change to panning, rattle or boost behaviour.

## Decisions

### 1. A small output trait, monomorphized

- `trait ModeOutput { type Sample; fn add(&mut self, index, mode_count, width, value); fn finish(self) -> Self::Sample; }` with `Mono(f32)` and `Stereo { left, right }` (names indicative). The resonator's loop becomes `fn run_modes<O: ModeOutput>(&mut self, excitation, sample_rate, out: O) -> O::Sample`, `#[inline(always)]`, called from both trait methods. `Mono::add` ignores `index`, `mode_count` and `width`, so after monomorphization the mono loop is today's loop.
- The voice does the same at its level (`Channels` trait with per-channel guard, clamp, gain and flush, and a `mid()` for rattle and boost); mono's mid is the sample itself, so the arithmetic order is today's.
- Alternative: a `const STEREO: bool` generic. Works too, but every difference becomes an `if STEREO` inside the function; the trait keeps each difference in one named place. Either compiles to the same code; pick the trait unless it measurably changes inlining.
- Alternative: compute stereo only and derive mono as `left + right`. Rejected: not bit-identical (0.5 gains and a different summation order), and the stereo loop costs more per mode.

### 2. The empty-bank case

The two paths disagree only for an empty bank (stereo skips preparation; mono runs it). Both return silence. The merged code keeps the stereo behaviour for both and a test shows the mono output for an empty bank is still 0 and the next non-empty note is unaffected; if any render hash changes because of it, keep each path's own behaviour instead.

### 3. Proofs

- **Sound:** render hashes for both paths (the voice manager's `process_sample` and `process_sample_stereo` at widths 0, 0.5 and 1), over the 16 exciters' default notes, the 9 objects, a bent note, a dynamic object and a kit pad, at 48 and 96 kHz, compared with a clean worktree (`git worktree add --detach /tmp/base HEAD`). Plus `print_48k_exciter_render_hashes` and the golden mode-bank fixture. Hashes on gnu only (CLAUDE.md: not portable across libm).
- **CPU:** stereo twins `voice_manager_sixteen_voices_stereo` and `voice_manager_sixteen_voices_bent_stereo` are added to `benches/performance.rs` first (task 1.2), measured on the base and on the change, interleaved, 5 runs, medians; `scripts/bench_ratios.py 5` for the H12 pairs. Output hashes of each benchmark scenario must match, so a code-placement swing (up to 30 %, CLAUDE.md) is recognised as such.

### Ordering

After `tidy-review-nits` (it removes N1's dead `match` and N4's wrapper; if it hasn't run, this change removes them) and after the behaviour changes that edit these two functions (`fix-specialty-adsr-release`, `m17-decorrelate-damage-rattle`, `default-patch-decay`, `trim-post-chain-per-sample-work`'s O4, which edits the resonator's preparation). Before `split-largest-source-files`, which then moves the merged code into `src/voice/render.rs` mechanically.

## Risks / Trade-offs

- [The generic version inlines differently and the hot loop slows] → `#[inline(always)]` on the loop and the trait methods; the stereo twins show it; if the stereo path slows beyond noise, try the `const bool` form before giving up.
- [A subtle reorder breaks bit-identity] → the hash comparison over both paths and several widths catches it; each layer (resonator, then voice) lands in its own commit.
- [Future fixes still go to one path] → there is only one path to fix after this.

## Migration Plan

None.

## Measurements

To fill in: the hash comparison table (all identical), the stereo and mono twins' medians before and after, `bench_ratios.py` medians, and the empty-bank verdict.
