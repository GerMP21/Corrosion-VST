# Proposal

## Why

The voice and the modal resonator each have two copies of their per-sample pipeline, one mono and one stereo (review finding O7, `FABLE-CODE-REVIEW-2026-11-14.md`; `docs/BACKLOG.md`):

- `Voice::process_sample` (`src/voice/mod.rs:1832-1889`) and `Voice::process_sample_stereo` (`:1908-1987`): NaN guard, rattle, velocity high-pass boost, clamp, expression and pad gain, denormal flush, peak tracking and tail shutdown, choke.
- `ModalResonator::process_sample` (`src/dsp/resonators/core.rs:1689-1764`) and `process_sample_stereo` (`:1766-1850`): preparation, coupling blend, the mode loop, ring-bound tracking, dynamics tracking and the contact feedback.

A fix applied to one copy and not the other is the classic drift. It has a concrete cost here: the plugin plays only the stereo path (`src/lib.rs:1218`), while the Criterion benchmarks (`benches/performance.rs:19, 38, 272`), the offline renderer (`src/offline/mod.rs:713`) and most tests drive the mono path. So the H12 cost bounds and many sound tests measure a copy the plugin doesn't run.

## What Changes

- One inner pipeline per layer, generic over the output (mono sum or panned stereo pair), monomorphized so each public entry point compiles to what it is today. The public API (`process_sample`, `process_sample_stereo`, the `ResonatorCore` trait) is unchanged.
- The trivial wrapper `process_exciter_stereo` and the dead `match self.profile` go with it, if `tidy-review-nits` (N1, N4) hasn't removed them already.
- Bit-identical output for both paths, proven with render hashes; CPU within noise.
- Stereo twins of the 16-voice benchmarks, so the plugin's real path is measured from now on.
- Not **BREAKING**: no behaviour change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None: a pure refactor, so `.openspec.yaml` sets `skip_specs: true`.

## Impact

- Code: `src/voice/mod.rs` (`:1832-1987`), `src/dsp/resonators/core.rs` (`:1689-1850`), `benches/performance.rs` (new stereo twins).
- Docs: `docs/ARCHITECTURE.md`, Internals pages `Voices-and-Polyphony.md`, `Modal-Resonator.md`, `Signal-Chain.md`, `Benchmarks-and-Performance.md`.
- Review: O7 marked FIXED, removed from `docs/BACKLOG.md`.
