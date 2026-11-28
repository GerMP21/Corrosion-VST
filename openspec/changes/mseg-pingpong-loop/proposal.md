# Proposal

## Why

Loop Mode offers Ping-Pong, but the MSEG ignores Loop Start and Loop End in that mode and always repeats Decay → Sustain, exactly like Forward 3 → 4 (`src/dsp/envelopes/mod.rs:374-377`). The help text, the README and the Wiki all have to apologise for it ("for now the same as Forward"). A mode that doesn't do what its name says is a trap; either it works or it goes.

## What Changes

Recommended (the maintainer confirms it at the gating task, see design.md):

- **Ping-Pong plays the loop region forward, then backward, then forward again**, between Loop Start and Loop End, while the gate is held. Played backward, each stage runs from its end level back to its start level over its own time and curve. Forward → backward and backward → forward turn where the levels already meet, so a Ping-Pong loop never jumps (Forward still jumps back to the start stage's start level, as today).
- Degenerate regions (start ≥ end), the stage clamping (0 acts as 1, 5 as 4) and release behave as in Forward.
- Help text ("loop", "Ping-Pong"), the `loop_mode` doc, README, the Internals and user Wiki pages lose their "partial" caveats.
- **Sound change, not BREAKING.** Patches with Loop Mode Ping-Pong sound different on friction exciters (a triangle instead of a sawtooth with the default 3 → 4). The parameter, its range and automation are unchanged. No migration: the mode was documented as unfinished.
- Factory bank: 33 presets store Loop Mode 2, all in `experimental/`; 9 of them use a friction exciter (the only family the MSEG drives) and can move. Bank tests and a reseed.

Alternative (if the maintainer prefers removing it): Loop Mode becomes Off / Forward; presets and DAW projects with Ping-Pong migrate to Forward 3 → 4, which is bit-identical to today's sound; BREAKING for automation of Loop Mode (the normalised range changes). Design.md describes the deltas for that branch.

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `mseg-envelope`: the loop-region requirement is scoped to Forward, and a new requirement defines Ping-Pong as a forward-then-backward loop between the chosen stages, without jumps.

## Impact

- Code: `src/dsp/envelopes/mod.rs` (`MSEG` loop state and `process_sample`), `src/params.rs` (`loop_mode` doc), `src/gui/controls.rs` (help text), `src/presets/factory/` (reseed only).
- Tests: new envelope tests; `loop_output_matches_golden` re-pinned for the `pingpong` row (intended change); `every_loop_configuration_is_bounded` keeps covering mode 2.
- Docs: `README.md`, `docs/ARCHITECTURE.md`, `docs/internals/Envelopes-and-Modulation.md`, `docs/internals/Parameters.md`, Wiki `exciters/envelopes.mdx` (EN + ES), guide data and the `loop_mode` clips.
- Related: `decide-drone-loop-step` (R6) could use Ping-Pong as its "smooth loop" answer for Drone mode; this change doesn't touch Drone mode's forced Forward loop.
