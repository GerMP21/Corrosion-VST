# Design

## Context

See proposal.md for why. What the code does today (`dev`, 2026-11-26); the review's line numbers still hold:

- **O1.** `src/lib.rs:1052`, first line of `apply_post_chain_params`, which `process` calls once per block (`:1179`): `self.post_chain.set_sample_rate(sample_rate)`. `PostProcessingChain::set_sample_rate` (`post_chain.rs:104-118`) forwards to every stage: the filter recomputes its smoothing `exp()`s and calls `update_coefficients` (`tan`), the drive recomputes two `exp()`s, the body rebuilds its frequencies and decays, the reverb recomputes its delays, the clipper two `exp()`s. Every `set_*_params` call then runs too, also per block. `configure_output_limiters` (`lib.rs:1032-1040`) does the same for the limiters (`recompute_release`).
- **The hidden dependency.** `WdfLadderFilter::process` (`wdf_filter.rs:169-190`) refreshes its coefficient only when cutoff or resonance move, or while `component_tolerance > 0`. If Component Tolerance goes from above 0 to 0, nothing in `process` recomputes `g` without the drift; today the next block's `set_sample_rate` does it. Dropping O1's per-block call without handling this would freeze the last drifted cutoff forever.
- **O2.** `fem_body.rs:91-115`: per sample and per mode (8), `omega_norm = 2π·f0 / sample_rate` (a division), its square, `material_gain = 0.8 + 0.4·material`, `1/(i+1)` (a division). `post_chain.rs:212-218` runs the body and multiplies by `body_amount` even at 0.
- **O3.** `wdf_filter.rs:183-190`: `drifting` is true whenever tolerance is above 0, so `update_coefficients` (`tan`, `:162`) runs every sample on both channels. The drift is a one-pole on noise with `DRIFT_TAU_SECONDS = 0.3` (`:76`).
- **O4.** `interaction.rs:186-210`: `update_coefficients` caches on the effective position. With wander (`update_position`, `:96-104`) or the Position Envelope's offset, the position changes every sample, so every mode's `sin(nπp)` (`mode_coefficient_1d`, `:11-15`) is recomputed every sample, per voice. It runs from `ModalResonator::prepare_sample` (`core.rs:1390`).
- The `post_processing_chain` benchmark (`benches/performance.rs:196-219`) builds the chain inside the timed closure, so it times `PostProcessingChain::new()` (which allocates the 192 kHz-sized Space buffers) along with 4 096 samples. It can't show O1–O3 cleanly.

## Goals / Non-Goals

**Goals:**
- Remove the per-block and per-sample work above, with the output bit-identical for O1 and for O2 at BODY above 0, and within the spec bounds for O3 and O4.
- Measure the gain honestly (CLAUDE.md benchmark rules).

**Non-Goals:**
- Skipping `set_*_params` calls whose values didn't change. They are cheap once `set_sample_rate` is gone, and caching every parameter adds state for little gain. Task 1.4 measures what is left; revisit only if it shows.
- Changing any sound on purpose. The body's restart from rest is a side effect of not running it at BODY 0, kept because it's simpler and arguably more natural.

## Decisions

### O1: cache the rate in the plugin, and let the filter handle tolerance 0

- `Corrosion` keeps the rate the post chain and limiters were last configured for (`Option<u32>`, `None` after `initialize` and `reset`) and calls `set_sample_rate` only when it differs. Alternative: guard inside `PostProcessingChain::set_sample_rate`. Rejected: `new()` already holds 48 kHz, so a guard there would skip the first configuration in tests that rely on it; the caller's `Option` makes the first call explicit.
- `WdfLadderFilter::set_parameters` calls `update_coefficients` when the tolerance changes to 0 (and when it changes at all, which is cheap and keeps the code obvious). That replaces the dependency on the per-block call. A test pins it (spec "Tolerance back to 0 restores the cutoff"), written first and shown red against a build with only the O1 guard.
- Any coefficient a later change derives from the rate (for example R4 in `rate-invariant-spread-and-ridge-wrap`, which converts the spread's and spring's constants) lands in `set_sample_rate` and is covered by the same guard. That's why this change runs after it (see Ordering).

### O2: precompute in the same arithmetic order, and idle at BODY 0

- `update_mode_frequencies` and `update_decay` store per mode `a = 2 − ω² − decay` and `b = 1 − decay` (with `ω = 2π·f0 / sample_rate` computed exactly as today), plus `material_gain` and `inv_index = 1/(i+1)` as separate arrays. `process` computes `force + a·y − b·y1` and `y·material_gain·inv_index` in the same order as today, so the result is bit-identical. Precomputing `material_gain·inv_index` as one product would change rounding; it's left as two multiplies.
- In `PostProcessingChain::process`, when `body_amount == 0.0 && body_amount_target == 0.0`, skip `body.process` and add nothing. On the sample the smoothed amount first reaches 0 with a 0 target, reset the body (`fill` on 16 floats, no allocation). So raising BODY again starts from rest.
- Output at BODY 0 differs from today only where `x + 0.0` flips a negative zero to positive; the existing `post-chain-routing` scenario ("within 1e-6") and the new one ("sample for sample", compared as values, not bits) both hold.

### O3: refresh the drift every 16 samples

- While tolerance is above 0 and cutoff and resonance are settled, advance the drift noise every sample as today (so its statistics and the noise sequence are unchanged), but call `update_coefficients` only every 16th sample. When the cutoff or resonance move, refresh every sample as today.
- 16 samples is 0.33 ms at 48 kHz, 1/900 of the drift's time constant. The drifted cutoff can move by at most about `DRIFT_DEPTH × 16/τ` of its spread in that time, far under 0.1 %.
- Alternative: a polynomial `tan` for small changes. Rejected: it changes the coefficient on every sample, so no part of the path stays bit-identical, and the accuracy depends on the cutoff.

### O4: a sine recurrence in double precision

- For `p` the effective position, `θ = π·p`, compute `s₁ = sin θ`, `c = cos θ` once, then `s_{n+1} = 2c·s_n − s_{n−1}` for every mode, in `f64`, storing each as `f32`. One `sin` and one `cos` per sample instead of one `sin` per mode.
- In `f64` the recurrence's error grows roughly linearly with `n`; at the largest bank (`MAX_MODES`) it stays near 1e-13, so the stored `f32` matches `sin(nπp)` to within an ulp in nearly every case, well inside the 1e-5 bound.
- The cache on the effective position stays: a still position recomputes nothing, as today. With the position still the weights are computed the old way (`mode_coefficient_1d`) on the first sample after a change; only moving positions use the recurrence. That keeps still-position notes bit-identical.
- Alternative: refresh every 32 samples and interpolate per mode. Rejected: it needs a second per-mode array of increments and is less accurate than the recurrence for the same cost.
- The fundamental-lock minimum (`fundamental_minimum`) is applied after the recurrence, as today.

### Measuring

- Bit-identity (O1, O2 at BODY above 0): a render-hash comparison against a clean worktree (`git worktree add --detach /tmp/base HEAD`), over the legacy corpus preset `sheet_lightning` and the 16 exciters' default renders, at 48 kHz on gnu (hashes are not portable across libm).
- Speed: new benchmark twins built outside the timed loop (`iter_batched`): `post_processing_chain_steady` (BODY 2, tolerance 0.05, as today's settings), `post_processing_chain_body_zero`, and `voice_manager_sixteen_voices_wander` (Wander 0.5). The base and the change are built in separate target dirs and run alternately, five times each; the median of the per-run ratios (change ÷ base) is recorded, per CLAUDE.md. Code layout can move the plain benchmarks by 30 % with identical output, so a ratio is trusted only with the output hash matching.

### Ordering

Run after `smooth-space-and-width` (R1–R3 rewrite `post_chain.rs`'s Space path and add a smoother), `rate-invariant-spread-and-ridge-wrap` (R4 adds rate-derived coefficients that O1's guard must cover), `glide-delay-length-controls` (O5 changes how the Space delays follow `set_factory_params`/`set_spring_params`) and `tidy-review-nits` (O8 edits `apply_post_chain_params`). Run before `split-largest-source-files` and `merge-mono-stereo-render-paths`, which move the same code mechanically and should move the final version.

## Risks / Trade-offs

- [Another stage silently depends on the per-block `set_sample_rate`, like the filter does] → task 1.1 audits every stage's `process` for state only `set_sample_rate` refreshes; the render-hash comparison with automation (tolerance on and off, quality switches, Space mode switches) catches what the audit misses.
- [The body's restart from rest is audible when BODY is automated up from 0] → judged by ear in task 2.4 with a clip; if it's worse, keep running the body at BODY 0 but with the precomputed constants (still bit-identical).
- [Recurrence error on a pathological position (p near 0 or 1, where `sin θ` is tiny)] → the test sweeps p over 0..1 including both ends; the bound is absolute (1e-5), and at those ends every weight is near 0 anyway.
- [Benchmark noise hides a few-percent gain] → the specs only require "no slower"; the Measurements record the medians whatever they are.

## Migration Plan

None: no parameter or preset changes.

## Measurements

To fill in: the stage audit (1.1), render-hash comparison results, the twins' medians before and after for each of O1–O4, the O3 level-difference table, the O4 worst weight error, and the BODY-from-0 clip verdict.
