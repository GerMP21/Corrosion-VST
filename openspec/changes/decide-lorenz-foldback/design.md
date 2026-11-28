# Design

## Context

See proposal.md for why. Current code (`dev`, 2026-11-26):

- `src/dsp/post_processing/lorenz_drive.rs:193-201`, `tube_saturate`: linear below 0.5, then `sign · (0.5 + u/(1 + u²))` with `u = |x| − 0.5`. `u/(1+u²)` peaks at `u = 1`, so the curve peaks at 1.0 for `|x| = 1.5` and falls to 0.845 at 3 and 0.712 at 5.
- `:123` `driven = input·(1 + 0.5·amount)`: ×3.5 at Drive Amount 5. `:137` the chaos multiplies the gain by up to 1.9. `:147` the product is clamped to ±5 before the curve.
- `:180-191` `mix`: the curve's output is clamped to ±1, then crossfaded with the dry input by `amount/5`. The crossfade and its tests (`mix_is_a_crossfade`, the `post-chain-drive` spec) are not touched.
- The drive has no oversampling (`docs/internals/Filter-and-Drive.md`: "All three run at the host rate"). The fold's extra harmonics alias.
- `docs/internals/Filter-and-Drive.md:147` already describes the fold and says its intent is undocumented.

The review's numbers hold on current code (checked by evaluating the formula; 0.845 at 3, 0.712 at 5).

## Goals / Non-Goals

**Goals:**
- Let the maintainer choose by ear between a documented foldback (A) and a monotonic curve (B), with clips that make the difference audible.
- Implement the choice with a test that pins the curve's shape, and close R8.

**Non-Goals:**
- Oversampling the drive. It would cut the aliasing of either curve, but costs CPU on both channels and is a separate decision.
- Changing the drive's gain, the chaos, the starvation or the dry/wet crossfade.
- The master drive on the voice mix (`src/lib.rs`), which has its own curve.

## Decisions

### 1. Render first, decide, then implement

Task group 1 renders the A/B material with a test-only switch on `LorenzDrive` (`#[cfg(test)] fold_back: bool`, the same pattern as `starvation_stage_enabled`), so both curves run from the same build:
- a 100 Hz and a 1 kHz sine at input peaks 0.5, 1, 2 and 4, through the drive at Drive Amount 5, chaos 0;
- the factory presets with the highest Drive Amount (found from the bank; the experimental grid varies Drive Amount, so several sit near 5), at velocity 100 and 127;
- the default patch with Drive Amount 5 and Chaos Depth 1.

Clips go to the change's folder as 48 kbps Opus (the guide clips' settings), with a table of each clip's peak, RMS and the energy above 10 kHz for A and B. Implementation (group 2) waits for the maintainer's choice, recorded in this file.

### 2. Recommended curve (B): `0.5 + 0.5·tanh(2·(|x| − 0.5))`

- Slope 1 at the knee, like today's curve (`d/du [u/(1+u²)] = 1` at `u = 0`), so the knee stays smooth: no new kink.
- Within 0.028 of today's curve up to 1.5 (largest gap at 1.25: 0.953 vs 0.980). Moderate drive keeps its tone; only hot peaks change.
- Monotonic, asymptotic to 1: hot peaks sit near full scale instead of folding. Fewer high harmonics than the fold, so less aliasing.
- Alternatives considered:
  - `tanh` with gain 2.2: closer to today's curve (0.017) but slope 1.1 at the knee, a small kink.
  - Hold at 1.0 above 1.5 (today's curve, clamped after its peak): monotonic, but the derivative jumps to 0 at 1.5 and the ±1 clamp then acts as a hard clip, which adds harmonics.
  - A table lookup (as the diode stage does): not needed for one `tanh` per sample per channel, but measured in task 2.3 if the CPU cost shows.

### 3. What the spec becomes under each choice

- **B (as written in `specs/post-chain-drive/spec.md`):** "The saturation curve never folds back".
- **A:** the delta is replaced by a requirement "Hot inputs fold back", pinning today's curve: linear below 0.5, output 1.0 at an input of 1.5, falling to 0.845 ± 0.001 at 3 and 0.712 ± 0.001 at 5, odd-symmetric. The docs then call the fold a feature, and say it aliases.

### 4. Proving moderate settings barely move

Under B, the change is not bit-identical anywhere above an input of 0.5. The proof is the bank: `curated_measurements_table` before and after, and the per-preset RMS change, recorded in Measurements. Presets that never drive the curve past 0.5 are bit-identical; a render-hash comparison against a clean worktree lists which ones are.

## Risks / Trade-offs

- [B makes some presets louder: a fold that pulled peaks down to 0.71 now holds them near 1] → the seeder re-levels every preset; the bank's floor and character tests must pass; Measurements lists every preset that moves more than 0.5 dB.
- [B removes a sound someone liked] → that is why group 1 renders the clips before any code changes; A remains a complete outcome.
- [`tanh` cost per sample] → measured with the `post_processing_chain` benchmark (task 2.3); if it costs more than noise, use the diode stage's tanh table.

## Migration Plan

No migration: no parameter changes meaning, and presets and projects store parameter values, not curves. Under B, existing user presets with a high Drive Amount sound different above an input of 1.5; the Wiki's Overdrive page says so.

## Measurements

To fill in during the change: the A/B clip table (group 1), the maintainer's choice, the bank before/after table and the list of moved presets, and the benchmark medians.
