# Tasks

## 1. Listen and decide

- [ ] 1.1 Add a test-only `fold_back` switch on `LorenzDrive` that selects today's `tube_saturate` or the proposed curve, and a unit test showing the old-curve position reproduces today's output bit for bit; verify with `cargo test --no-default-features lorenz`
- [ ] 1.2 Add an ignored test (`render_drive_fold_ab`) that renders the A/B material listed in design.md (sines at peaks 0.5/1/2/4, the highest-Drive factory presets at velocity 100 and 127, the default patch at Drive 5 and Chaos 1) as WAV pairs, and prints peak, RMS and energy above 10 kHz per clip; verify the table prints and the WAVs exist
- [ ] 1.3 Encode the clips at 48 kbps Opus into `openspec/changes/decide-lorenz-foldback/clips/` and add the table to design.md `## Measurements`; verify every clip pair plays and the table lists all of them
- [ ] 1.4 Ask the maintainer to choose A (documented foldback) or B (monotonic curve), record the choice and date in design.md, and if A, replace the spec delta with the "Hot inputs fold back" requirement from design.md Decision 3; verify `openspec validate decide-lorenz-foldback` passes

## 2. Implement the chosen curve

- [ ] 2.1 Write the spec scenarios as unit tests in `lorenz_drive.rs` (monotonic sweep 0–5, linear below the knee, within 0.03 of the old curve up to 1.5, ≥ 0.99 at 3 and 5, odd symmetry; or the fold values under A) and show they fail on current code (B) or pass as a reference (A); verify with `cargo test --no-default-features lorenz`
- [ ] 2.2 Under B, replace `tube_saturate` with `0.5 + 0.5·tanh(2·(|x| − 0.5))` above the knee and remove the test-only switch; under A, add a doc comment on `tube_saturate` describing the fold as intended; verify the 2.1 tests pass and the `post-chain-drive` spec tests (`mix_is_a_crossfade`, starvation tests) still pass
- [ ] 2.3 Under B, run `post_processing_chain` and `plugin_output_path_sixteen_voices` against a clean-worktree baseline, interleaved, 5 runs, and record the medians in design.md; verify the cost is within noise or switch to the diode stage's tanh table
- [ ] 2.4 Under B, run the bank tests and `curated_measurements_table` before and after, reseed (`cargo run --release --bin seed_presets`), and record every preset that moved more than 0.5 dB in design.md; verify `cargo test --no-default-features presets::factory` and `generated_bank_equals_the_files_on_disk` pass

## 3. Documentation

- [ ] 3.1 Update `docs/ARCHITECTURE.md` (drive section) and `docs/internals/Filter-and-Drive.md` (the `tube_saturate` paragraph and its diagram) to describe the chosen curve with its numbers; verify `cargo test --no-default-features --test internals_wiki` passes
- [ ] 3.2 Update the Wiki's Overdrive page in English and Spanish (`web/guide/src/content/docs/output-chain/overdrive.mdx`, `es/output-chain/overdrive.mdx`) to say what hot drive does (holds near full scale under B; folds back, with a tip on where it gets gritty, under A), in the guide's voice; verify `npm run check -- --complete` in `web/guide/`
- [ ] 3.3 Under B, regenerate the guide clips (`scripts/build-guide-clips.sh`) and say so in the close-out; verify `npm run check` passes and the Drive Amount clips changed
- [ ] 3.4 Mark R8 FIXED in `FABLE-CODE-REVIEW-2026-11-14.md` (date, change, the test that proves it) and remove it from `docs/BACKLOG.md`; verify both files no longer list R8 as open

## 4. Close out

- [ ] 4.1 Run `cargo fmt --check`, `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`, `cargo test --workspace --no-default-features`, `cargo test --no-default-features --test no_alloc` and the internals_wiki test; verify all pass
- [ ] 4.2 Update CLAUDE.md "Current state" (date, R8's outcome, what is next) and commit and push to `dev`; verify `git log origin/dev -1` shows the close-out

## Workflow follow-up

- Archive with `/opsx:archive decide-lorenz-foldback` once the maintainer has heard the result in a DAW.
