# Design

## Context

See proposal.md for why. Checked on `dev` (2026-11-26):
- `src/dsp/post_processing/hrtf_spread.rs:89-95`: `lpf = lpf·0.8 + x·0.2`; `hpf = hpf·0.5 + (x − lpf)·0.5`, per channel. `set_sample_rate` (`:54-56`) only stores the rate.
- `src/dsp/post_processing/space.rs:250-255`: `dispersion_coeff = 0.5 + 0.4·s + (s − 0.5)·reach` (reach 0.7 below the default, 0.2 above), so 0.15 at Wire Stiffness 0, 0.7 at 0.5 and 1.0 at 1 (the review's "0.5–0.9" predates `fix-inert-controls`). Not rate-converted. The Spring's delay is already converted (`spring_delay_does_not_depend_on_the_sample_rate`).
- `src/dsp/utils/rate.rs:30-37`: `smoother(k, ratio) = 1 − (1 − k)^ratio`, the identity at 48 kHz.
- `src/dsp/exciters/corrugated_drag.rs:62-71`: `position += v_rel · rate::step(0.01, ratio)`, never wrapped; `bump = sin(2π · position / ridge_spacing)`. Ridge Spacing is set only at note-on (`src/voice/mod.rs:1246`, in `note_on_with_controls`). Bow wraps its position (`src/dsp/exciters/bow.rs:120-124`).
- Numbers at the default patch, velocity 100: speed ≈ 0.82 · 0.85 = 0.70, step ≈ 0.007 per sample, position ≈ 336 per second. After 60 s (≈ 20 000) the `f32` spacing is 0.002, 28 % of a step.

## Goals / Non-Goals

**Goals:**
- Proximity and Spring dispersion corners in hertz independent of the sample rate; bit-identical at 48 kHz.
- Corrugated Drag's ridge rate stable for any note length.

**Non-Goals:**
- The aliasing of Corrugated Drag's bumps at extreme settings (documented as a trap; not this change).
- What the spread does to the image (`widen-hrtf-spread`); Bow's Roughness ripple (`fix-internals-wiki-findings`).

## Decisions

1. **Proximity filters:** keep today's expression form, `lpf = lpf·a + x·k` and `hpf = hpf·b + (x − lpf)·m`, with `k = rate::smoother(0.2, ratio)`, `a = 1 − k`, `m = rate::smoother(0.5, ratio)`, `b = 1 − m`, all cached in `set_sample_rate`. At 48 kHz they are exactly 0.2, 0.8, 0.5 and 0.5 (`1.0f32 − 0.2f32 == 0.8f32`), so the per-sample arithmetic is the same as today and the output is bit-identical; a test compares against the literal old expressions.
   - *Alternative:* the textbook `lpf += (x − lpf)·k`. Same filter, but it rounds differently in `f32`, so 48 kHz would no longer be bit-identical. Rejected.
2. **Spring dispersion:** compute `k = dispersion_coeff` as today from stiffness, then `k_rate = rate::smoother(k, ratio)` once per parameter change (in `set_parameters`/`set_sample_rate`, not per sample), and run `state = state·(1 − k_rate) + x·k_rate`. At 48 kHz `k_rate == k` (bit-identical). `k = 1.0` stays 1.0 at every rate.
3. **Ridge phase:** replace `position` with `phase ∈ [0, 1)`. Per sample: `phase += v_rel · step · inv_spacing`, then wrap with `phase -= phase.floor()` (handles negative `v_rel` too); `bump = sin(2π · phase)`. `inv_spacing = 1 / ridge_spacing` is computed in `set_parameters`, which also removes a per-sample division.
   - *Alternative:* wrap `position` by `ridge_spacing` (`rem_euclid`). Same precision; keeps a division per sample. Rejected for cost, not correctness.
   - *Alternative:* `f64` position. Precise for years but slower in a hot loop and still unbounded. Rejected.
4. **New capability `friction-exciters`** for the ridge-rate requirement; the existing `exciter-controls` is about the global Pressure, Speed and Roughness controls, not one model's behaviour.

## Risks / Trade-offs

- [R5 changes rounding from the first sample, so renders of Corrugated Drag presets change by noise] → compare `curated_measurements_table` before/after; expect level changes well under 0.1 dB. Golden fixtures that include Corrugated Drag (if any) are regenerated only after checking the difference is noise, with the reason noted.
- [`rate::smoother` uses `powf` at non-48 kHz rates] → computed only on rate or parameter changes, never per sample.

## Migration Plan

None; not BREAKING.

## Measurements

(Filled in during implementation: proximity and Spring band levels at 44.1/48/96 kHz before/after, the 48 kHz bit-identity proof, the ridge-rate drift over 10 minutes before/after, Corrugated Drag presets' level changes.)
