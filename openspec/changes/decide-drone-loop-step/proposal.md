# Proposal

## Why

In Drone mode every friction note's MSEG is forced into a Forward loop from Decay (3) to Sustain (4), whatever the preset says (`src/lib.rs:713-720`). Each time Decay finishes, the loop restarts Decay **at the peak level**, so the force envelope steps from the sustain level straight back to the peak: 0.5 → 0.75 every 0.3 s at the defaults (`FABLE-CODE-REVIEW-2026-11-14.md` R6). The Wiki sells the pulse as a feature ("the drone pulses at the Decay's rate"), and the Internals wiki explains why the loop exists (a constant Sustain holds the object almost still), but the hard edge is a click in the force that the object rings on. The same jump happens in every user Forward loop (Attack restarts from Onset Level, Decay from the peak), in Tonal too.

The review marks R6 "decide by ear". This change renders the options for the maintainer, then implements the chosen one.

## What Changes

**Recommended (option A), subject to the maintainer's listening:**
- Every MSEG loop restart glides from the current level to the restart stage's start level over a short time (between 2 and 20 ms, the exact time picked by ear), instead of jumping. The pulse and its rate stay; the step goes. It applies to Drone's forced loop and to user Forward loops alike.

**Alternatives rendered for comparison:**
- **A′, Drone only:** the glide applies only to Drone's forced loop; user loops keep their hard restart.
- **C, no forced loop:** Drone stops overriding the MSEG's Loop settings; a held friction note sits at Sustain unless the preset loops.
- **D, keep:** no code change; ARCHITECTURE and the Wiki say the pulse has a hard edge.

Under A, A′ and C the sound of Drone presets (and under A, of looping Tonal presets) changes: bank tests, reseed, Wiki Drone, Envelopes and Troubleshooting pages (EN + ES), and clips. Not BREAKING: no parameter changes meaning.

## Capabilities

### New Capabilities

(none)

### Modified Capabilities
- `mseg-envelope`: under option A, the loop requirement's "identical to the behavior before this change" scenario changes, and a new requirement says a loop restart has no step. (Design lists the delta for A′, C and D.)

## Impact

- Code (option A): `src/dsp/envelopes/mod.rs` (`advance_stage` and the per-sample level: a glide segment at a loop restart). A′ adds a flag set from `note_setup` in `src/lib.rs`; C removes the override at `src/lib.rs:713-720` and `drone_mode_forces_mseg_loop_on`.
- Tests: `loop_output_matches_golden` (`src/dsp/envelopes/mod.rs:485`) pins the loop bit-for-bit and is regenerated with the reason; `drone_mode_forces_mseg_loop_on` stays under A/A′.
- Presets: 59 factory presets play in Drone, 44 use Forward loops and 33 PingPong (which loops Decay → Sustain today); bank tests and reseed.
- Ordering: `mseg-pingpong-loop` rewrites PingPong's region; both edit `advance_stage`. Land this first so PingPong's turnarounds inherit the glide rule, or apply the glide to its restarts if it lands first. `add-release-snap` and `env-amount-flat-envelope` also touch the MSEG; no conflict in behaviour.
- Docs: `docs/ARCHITECTURE.md`, Internals `Envelopes-and-Modulation.md` ("A loop jumps, it doesn't glide"; the Drone mode section), user Wiki `note-modes/drone.mdx`, `exciters/envelopes.mdx`, Troubleshooting (EN + ES); guide clips for `loop_mode` and Drone pages.
- Review: R6 marked FIXED (or "decided: kept" under D); removed from `docs/BACKLOG.md`.
