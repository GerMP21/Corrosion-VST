# Proposal

## Why

The post chain and the strike-position bus do work every block or every sample that only needs doing when something changes (review findings O1–O4, `FABLE-CODE-REVIEW-2026-11-14.md`; `docs/BACKLOG.md`). None of it is audible, but it is CPU spent on every voice or every channel for nothing, and the plugin runs 16 voices in a host that has other plugins to run.

## What Changes

- **O1.** The post chain is no longer reconfigured from scratch every block: `set_sample_rate` (every stage's smoothing `exp()`, the body's mode tables, the reverb's delays, the filter's `tan`) runs only when the rate changes. A hidden dependency goes with it: today the per-block call is what restores the filter's exact cutoff when Component Tolerance returns to 0; the filter now does that itself.
- **O2.** The FEM body precomputes its per-mode constants when its controls or the rate change, instead of dividing by the sample rate and recomputing gains per mode per sample. While BODY is 0 (target and smoothed value), the body does no work and holds no state, so raising BODY from 0 starts it from rest.
- **O3.** While Component Tolerance is above 0 and the cutoff is not moving, the filter refreshes its drifted coefficient every 16 samples instead of calling `tan()` every sample. The drift has a 0.3 s time constant, so 16 samples (0.33 ms at 48 kHz) are far inside it.
- **O4.** While the strike position moves (wander or Position Envelope), the per-mode position weights `sin(nπp)` are computed with a recurrence (one `sin` and one `cos` per sample, then two multiplies per mode) instead of one `sin` per mode per sample.
- O1 and O2 at BODY above 0 are bit-identical. O3 and O4 change the output by rounding only (bounds in the specs). O2's restart from rest is the one audible difference, and only when BODY is raised from 0 while sound plays.
- Measured with the post-chain and plugin-path benchmarks, medians of 5 interleaved runs against a clean-worktree baseline.
- Not **BREAKING**.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `realtime-safety`: adds that the post chain is configured when its rate or controls change, not every block, with output unchanged.
- `post-chain-routing`: adds that the body rests while BODY is 0 and starts from rest when raised.
- `post-chain-filter`: adds that Component Tolerance's drift, refreshed at a control rate, keeps the cutoff within a bound of the per-sample drift, and that returning the tolerance to 0 restores the exact cutoff.
- `strike-position`: adds an accuracy bound on the per-mode position weights while the position moves.

## Impact

- Code: `src/lib.rs` (`apply_post_chain_params`, `:1045-1052`), `src/dsp/post_processing/post_chain.rs` (`set_sample_rate` `:104-118`, `process` `:205-237`), `fem_body.rs` (`:50-115`), `wdf_filter.rs` (`:110-190`), `src/dsp/interaction.rs` (`update_coefficients`, `:186-210`).
- Benchmarks: `benches/performance.rs` gains post-chain twins built outside the timed loop (BODY 0, Component Tolerance on), since `post_processing_chain` today times `PostProcessingChain::new()` too.
- Docs: `docs/ARCHITECTURE.md`, Internals pages `Post-Chain.md`, `Body-Spread-and-Space.md`, `Filter-and-Drive.md`, `Interaction-Bus-and-Contact.md`, `Benchmarks-and-Performance.md`.
- Wiki: the BODY text in English and Spanish, only if the restart from rest is worth a line (task 2.4 decides by ear).
- Presets: none move (BODY automation from 0 is not in the bank's renders); bank tests must still pass.
- Review: O1–O4 marked FIXED, removed from `docs/BACKLOG.md`.
