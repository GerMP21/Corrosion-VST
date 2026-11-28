# Proposal

## Why

Two precision faults from `FABLE-CODE-REVIEW-2026-11-14.md`:
- **R4.** The project's sample-rate rule says per-sample constants are tuned at 48 kHz and converted with `dsp::utils::rate`. Two post-chain filters break it: the HRTF spread's proximity one-poles (`0.8/0.2`, `0.5/0.5`) and the Spring's dispersion one-pole. At 96 kHz their corners move up an octave, so Listener Proximity and Wire Stiffness sound different at 44.1, 48 and 96 kHz.
- **R5.** Corrugated Drag accumulates its ridge position in `f32` without wrapping. On the default patch it grows by about 340 per second; after a minute the `f32` spacing near 20 000 is about 28 % of a per-sample step, so the ridge rate, which sets the corrugation's pitch, drifts and quantizes on long Drone notes.

## What Changes

- The spread's proximity low-pass and high-pass coefficients and the Spring's dispersion coefficient are converted with `rate::smoother`, exactly their current values at 48 kHz (bit-identical there) and the same corners in hertz at every other rate.
- Corrugated Drag keeps its ridge phase in `[0, 1)` (a phase accumulator advanced by `v_rel · step / spacing`), so the ridge rate is as precise after ten minutes as on the first sample. This changes rounding from the first sample: the sound moves by noise only.
- Not BREAKING. At 48 kHz R4 is bit-identical; R5 moves renders by rounding noise.

## Capabilities

### New Capabilities
- `friction-exciters`: behaviour of the friction-family contact models (Bow, Stiff Point Scrape, Heavy Grinding, Corrugated Drag, Tension Rise) that holds over the life of a note; starts with Corrugated Drag's ridge rate staying stable on long notes.

### Modified Capabilities
- `sample-rate-independence`: adds that the spread's proximity tone and the Spring's dispersion are the same at every sample rate.

## Impact

- Code: `src/dsp/post_processing/hrtf_spread.rs` (proximity coefficients, cached in `set_sample_rate`), `src/dsp/post_processing/space.rs` (Spring dispersion), `src/dsp/exciters/corrugated_drag.rs` (phase accumulator, reciprocal spacing computed in `set_parameters`).
- Presets: R4 none at 48 kHz. R5 moves Corrugated Drag presets by rounding noise; the bank tests and render hashes show how much.
- Ordering: `smooth-space-and-width` and `widen-hrtf-spread` also edit `hrtf_spread.rs`; apply this one after `smooth-space-and-width`, and before or after `widen-hrtf-spread` as long as the proximity filters keep their rate-converted coefficients.
- Docs: `docs/ARCHITECTURE.md`, Internals `Body-Spread-and-Space.md`, `Sample-Rate-Rule.md` and `Friction-Exciters.md` (the position formula). No user-visible behaviour at 48 kHz, so the user Wiki needs no change (Listener Proximity and Wire Stiffness now sound the same at 44.1 and 96 kHz as their pages already describe).
- Review: R4 and R5 marked FIXED; removed from `docs/BACKLOG.md`.
