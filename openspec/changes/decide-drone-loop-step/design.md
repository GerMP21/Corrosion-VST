# Design

## Context

See proposal.md for why. Checked on `dev` (2026-11-26):
- `src/lib.rs:713-720` (`note_setup`): in Drone mode, `controls.loop_mode = 1` (Forward), start 3, end 4, overriding the preset. Comment: "so friction voices ring indefinitely" (they would anyway: Sustain holds while the gate is held). The Internals page (`docs/internals/Envelopes-and-Modulation.md`, "Drone mode") gives the real reason: without the loop a held friction note pushes almost constantly and the object sits nearly still.
- `src/dsp/envelopes/mod.rs:344-366` (`advance_stage`): with the gate held, Decay hands over to Sustain, which is the loop end, so the stage becomes Decay again with `stage_time = 0`.
- `src/dsp/envelopes/mod.rs:305-313`: Decay's start level is `l_peak`, Attack's is `l_onset`; the per-sample level is `start + (target − start)·curve(progress)`. So the restart is a one-sample jump from `l_sustain` to `l_peak`.
- `src/dsp/envelopes/mod.rs:372-381`: PingPong currently loops 3 → 4 too, so it steps the same way (`mseg-pingpong-loop` reworks it).
- The user Wiki presents the pulse as a feature (`note-modes/drone.mdx`: "the drone pulses at the Decay's rate"; `exciters/envelopes.mdx`: "a looping grinder is half a groove already").

## Goals / Non-Goals

**Goals:**
- The maintainer hears the options and picks one; the chosen one is implemented with tests and docs.
- Under A/A′: no step in the force envelope at a loop restart; the pulse's rate and depth stay.

**Non-Goals:**
- PingPong's region and direction (`mseg-pingpong-loop`).
- Release Snap (`add-release-snap`), Env Amount at 0 (`env-amount-flat-envelope`).

## Decisions

1. **Decide by ear first (task group 1).** Render 8 s clips, through the full post chain, of `drones/grinding_beam`, `drones/bowed_beam_drone`, `drones/chain_drag_drone` and one Tonal preset with a Forward loop, under: current, A at 2, 5 and 10 ms, and C. Put them in the session's scratchpad with a short table (peak-to-sustain distance, pulse period, the largest force step), and ask the maintainer to choose the option and the glide time. Implementation waits for that answer.
2. **Option A (recommended): a restart glide in the MSEG.** When `advance_stage` restarts the loop, the MSEG enters a `glide` sub-state: for `glide_samples` it ramps linearly from the current level to the restart stage's start level, then runs the stage from `stage_time = 0` as today. `glide_samples` is computed from the chosen time with the sample-rate rule (a duration, so the same in seconds at every rate). The glide adds its length to the loop period (5 ms on a 300 ms Decay is under 2 %).
   - *Alternative:* start the restarted stage from the current level (the review's "smooth sawtooth"). For the default Decay → Sustain loop that makes Decay run from sustain to sustain: a flat line, no pulse at all. Rejected.
   - *Alternative:* restart Attack instead of Decay in Drone (loop 1 → 4). Uses the Attack time as the rise, which couples the pulse's shape to the note's attack. Rendered only if the maintainer asks.
   - Linear glide: simplest and has no step; a raised-cosine glide is a one-line change if the maintainer hears a corner.
3. **Why all loops, not just Drone.** The step comes from `advance_stage`, not from Drone: a user's Forward loop clicks the same way. One rule is easier to explain in the Wiki. A′ (Drone only) is the fallback if the maintainer wants the hard edge kept for user loops as a rhythmic effect.
4. **Spec delta per option.**
   - A: as written in `specs/mseg-envelope/spec.md` (MODIFIED loop requirement, ADDED "A loop restart has no step").
   - A′: no MODIFIED; the ADDED requirement is renamed "Drone's loop restart has no step" and scoped to Drone mode; the loop requirement's "identical output" scenario stays.
   - C: the ADDED requirement becomes "Drone mode keeps the MSEG's own loop settings" (a held Drone friction note with Loop Off holds Sustain); no MODIFIED; `drone_mode_forces_mseg_loop_on` is replaced by its inverse.
   - D: the delta is dropped and `.openspec.yaml` gets `skip_specs: true`; only docs change.
   The delta file is rewritten to the chosen option before implementation (task 1.3).
5. **Golden fixture.** `loop_output_matches_golden` pins the Decay → Sustain loop. Under A it is regenerated once, with the reason in its doc comment, after the new tests pass.

## Risks / Trade-offs

- [The glide softens the attack of fast rhythmic loops] → the clips include 2 ms; A′ keeps the hard edge for user loops if wanted.
- [Many presets move (Drone and looping Tonal)] → bank tests and reseed; record the level and character changes in Measurements.
- [Ordering with `mseg-pingpong-loop`] → whichever lands second applies the glide to PingPong's turnarounds (noted in both).

## Migration Plan

None: no parameter changes meaning, so saved state loads as before; the sound of looped notes changes. Not BREAKING.

## Open Questions

- The glide time within 2–20 ms (picked by ear in task 1.2; the spec allows the whole range).

## Measurements

(Filled in during implementation: the clips' table, the maintainer's choice and date, the largest force step before/after, loop period before/after, bank-test results and which presets moved.)
