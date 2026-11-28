# Tasks

## 1. Decide by ear

- [ ] 1.1 Add a test-only switch on `MSEG` for the restart behaviour (current, glide of N ms, Drone-only) and a switch on `note_setup` to skip the Drone override; verify that with the switches at "current" every existing envelope test and `loop_output_matches_golden` pass unchanged.
- [ ] 1.2 Render the clips of design decision 1 (an ignored test or `render_presets` run writing WAV/Opus into the session scratchpad) with a table of peak-to-sustain distance, pulse period and largest force step per clip; hand them to the maintainer and record the choice (option and glide time) with its date in design.md Measurements. Verify the clips play and the table matches the renders.
- [ ] 1.3 Rewrite `specs/mseg-envelope/spec.md` to the chosen option (design decision 4; under D set `skip_specs: true`), and adjust groups 2–4 below to it; verify `openspec validate decide-drone-loop-step` passes.

## 2. Implement the chosen option (A shown)

- [ ] 2.1 Add tests for "A loop restart has no step" (Drone defaults: glide length and largest step; release during a glide; restart with no level change unchanged) and for the modified default-loop scenario; verify they fail on current `dev`.
- [ ] 2.2 Implement the restart glide in `advance_stage` and the per-sample level, with the glide length from the sample-rate rule; verify 2.1 passes, `every_loop_configuration_is_bounded` and `default_and_drone_loops_never_hit_transition_cap` pass, and `cargo test --no-default-features --test no_alloc` passes.
- [ ] 2.3 Regenerate `loop_output_matches_golden`'s fixture with the reason in its doc comment; verify it passes on gnu (and musl if the fixture is checked there).
- [ ] 2.4 Remove the test-only switches that only served the clips, or keep them `#[cfg(test)]`; verify `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` passes.

## 3. Presets and clips

- [ ] 3.1 Run the bank tests and `curated_measurements_table` before and after; rerun `cargo run --release --bin seed_presets`; record what moved (Drone and looping presets) in Measurements; verify the bank tests and `generated_bank_equals_the_files_on_disk` pass.
- [ ] 3.2 Regenerate the guide clips with `scripts/build-guide-clips.sh` (the loop sound moves for every looped preset and the Drone demos); verify `npm run check -- --complete` passes and say in the close-out that clips were regenerated.

## 4. Docs

- [ ] 4.1 Update Internals `docs/internals/Envelopes-and-Modulation.md` (the loop section's "A loop jumps, it doesn't glide", the Drone mode section) and `docs/ARCHITECTURE.md`; verify `cargo test --no-default-features --test internals_wiki` passes.
- [ ] 4.2 Update the user Wiki `note-modes/drone.mdx`, `exciters/envelopes.mdx` (Loop modes) and Troubleshooting if the old click was mentioned, in English and Spanish, in the guide's voice; if `loop_mode`'s ⓘ text changes, run `cargo run --bin export_guide_data` and update `parameters.es.json`. Verify `npm run check -- --complete` passes.
- [ ] 4.3 Mark R6 FIXED (or "decided: kept, documented" under D) in `FABLE-CODE-REVIEW-2026-11-14.md` with the date, the maintainer's choice and the proving tests; remove it from `docs/BACKLOG.md`. Verify by reading both.

## 5. Close-out

- [ ] 5.1 Run `cargo fmt --check`, clippy with `-D warnings`, `cargo test --workspace --no-default-features`, `no_alloc` and `internals_wiki`; verify all pass.
- [ ] 5.2 Update CLAUDE.md "Current state" (the choice, what moved, clips regenerated), commit on `dev` and push; verify `git status` is clean.

## Workflow follow-up

- Archive with `/opsx:archive`; tell `mseg-pingpong-loop` which restart rule PingPong's turnarounds must follow.
