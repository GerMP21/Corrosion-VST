# Tasks

## 1. Measure today's snap and choose the default (maintainer)

- [ ] 1.1 Add an ignored measurement test `print_release_snap_table` that renders, through the plugin's output path (voice mix DC blocker and post chain), each friction exciter on each object at C4 velocity 100, held 1 s then released, and prints held-second RMS, 0.5 s tail RMS and their ratio; verify it reproduces `reorganize-factory-presets`' order of magnitude (Plate + Corrugated Drag around +30 dB) or record why not, in design.md `## Measurements`
- [ ] 1.2 Run the same table over the factory bank (non-kit presets, velocity 100 and 127) and record the presets whose tail exceeds the held second by more than 6 dB; verify the list is in Measurements
- [ ] 1.3 Prototype Decision 1 behind a test-only switch, tune the cutoff and `T_max` until Release Snap 0 meets the spec bound on every object with at least 1 dB margin, and record the worst case; if no setting meets it, stop and report to the maintainer with the numbers
- [ ] 1.4 Render A/B clips (Release Snap 1, 0.5, 0) of Bowed Cable Sub, Plate + Corrugated Drag and Plate + Stiff Point Scrape into the scratchpad for the maintainer; verify the maintainer's choice of default is recorded in design.md (Decision 3, *As built*)

## 2. Tests first

- [ ] 2.1 Write the `friction-release` spec scenarios as tests (bound at 0, bit-identity at 1 across the 16 exciters via the test-only switch, monotonic sweep on Bow · Taut Cable, Hit/Specialty unaffected, old preset and old project load with 1); verify each fails or does not compile on current code
- [ ] 2.2 Write the `factory-preset-bank` released-note bound as a bank test; verify it fails on the current bank, naming the presets from 1.2

## 3. Implement the control

- [ ] 3.1 Add `release_snap` to `src/params.rs` (constructor shared with `Preset::into_params`), `VoiceControls`, the kit pad values and the randomizer's friction set; verify `cargo test --no-default-features` builds and the host-parameter uniqueness test passes
- [ ] 3.2 Implement Decision 1 in the friction note-off path of `src/voice/mod.rs`, skipping it entirely at 1; verify the tests from 2.1 pass and `tests/no_alloc.rs` passes in debug
- [ ] 3.3 Add the preset and host migrations (Decision 5) with the tests from 2.1; verify an old preset from `tests/fixtures/presets/legacy/` and a project state without the field load with 1
- [ ] 3.4 Prove bit-identity at 1: compare `print_48k_exciter_render_hashes` against a clean worktree and record both in Measurements; verify they match for all 16 exciters
- [ ] 3.5 Check the CPU cost with `python3 scripts/bench_ratios.py 5`; verify within noise and record the medians

## 4. Editor and help

- [ ] 4.1 Add the Release Snap fader to the MSEG panel for the friction family (`src/gui/editor.rs`, `FIXED_CONTROLS` in `src/gui/controls.rs`) with its help text ("How suddenly the object springs back when you let go. Lower eases it back; higher snaps."); verify the GUI lane (`cargo test --lib`) passes, including help coverage and editor layout tests
- [ ] 4.2 Retake the guide screenshots (`node web/scripts/screenshots.cjs`) and check the callouts on the MSEG panel; verify the images changed only where the fader was added

## 5. Factory bank

- [ ] 5.1 Add the held-then-released render to `src/presets/factory/measure.rs`, set Release Snap on the curated presets that fail the bound, and give `experimental.rs` a Release Snap range that passes; verify the bank tests (`presets::factory::tests`) pass, including 2.2
- [ ] 5.2 Reseed (`cargo run --release --bin seed_presets`); verify `generated_bank_equals_the_files_on_disk` passes and record in Measurements which presets got a value below 1 and how their tails moved

## 6. Guide, Wiki and docs

- [ ] 6.1 Add a clip plan entry for Release Snap in `src/guide/clip_plan.rs` (Bow on Taut Cable, released), run `cargo run --bin export_guide_data`, and add the Spanish entry to `web/guide/src/data/parameters.es.json`; verify `guide_clip_plan_covers_every_parameter`, `guide_data_equals_the_files_on_disk` and `npm run check -- --complete` (in `web/guide/`) pass
- [ ] 6.2 Regenerate the guide clips (`scripts/build-guide-clips.sh`) since the plan changed; verify the fingerprint test passes and note the regeneration in the close-out
- [ ] 6.3 Update the user Wiki in English and Spanish: the envelopes (MSEG) page, the Bow, Stiff Point Scrape and Corrugated Drag pages, and Troubleshooting ("my note jumps when I let go"), in the wiki writing style; verify `npm run check -- --complete` passes
- [ ] 6.4 Update `docs/ARCHITECTURE.md` and the Internals pages that describe the friction release (`grep -rl` for `voice/mod.rs`, `envelopes/mod.rs`, the friction exciters and `presets/factory` in `docs/internals/`; at least Friction-Exciters, Envelopes-and-Modulation, Parameters, Presets-and-Migrations, Factory-Bank, Editor); verify `cargo test --no-default-features --test internals_wiki` passes
- [ ] 6.5 Remove the Release Snap item from `docs/BACKLOG.md`; verify the file no longer lists it

## 7. Close-out

- [ ] 7.1 Run `cargo fmt --check`, `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`, `cargo test --workspace --no-default-features`, `cargo test --lib`, `cargo test --no-default-features --test no_alloc`, `cargo doc --no-deps --no-default-features`; verify all pass and record the counts
- [ ] 7.2 Update "Current state" in `CLAUDE.md` (row, next, archive note), commit on `dev` and push; verify `git log origin/dev` shows the commit

## Workflow follow-up

- Archive with `/opsx:archive` once the maintainer has heard the bank's changed presets.
