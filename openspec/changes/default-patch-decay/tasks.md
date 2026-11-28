# Tasks

## 1. Measure and decide (maintainer)

- [ ] 1.1 Add an ignored measurement test `print_default_patch_envelope` that prints the default note's 100 ms RMS envelope (C4, held 4 s) at velocity 25, 64, 100 and 127, and the same with the governor disabled, with Flesh Stiffness at its maximum and with Damping 1; record the table in design.md `## Measurements` and name what holds the plateau, or record that it is gone (then close the change with the measurement)
- [ ] 1.2 Render A/B clips of the default patch under the Decision 2 candidates (today, Rust 0.3, Sludge 0.3, raised damping, raised Flesh Stiffness, Plate) into the scratchpad, each with its spec measurements; verify the maintainer's choice is recorded in design.md (*As built*), and rewrite the spec and tasks first if they choose alternative 2 or 3

## 2. Tests first

- [ ] 2.1 Write the `default-patch` scenarios as tests (the decay bounds at C2, C3, C4, C5; an old preset and an old project lacking the changed field load with the old default and render bit-identical); verify the decay tests fail on today's default patch
- [ ] 2.2 List the tests that use the default patch as a reference and change those that must keep the old patch to set the old value explicitly; verify the full CI lane still passes before the default changes

## 3. Implement

- [ ] 3.1 Change the chosen default(s) in `src/params.rs`; verify 2.1's decay tests pass, `default_notes_do_not_overdrive_the_voice` passes, and the factory bank tests and `generated_bank_equals_the_files_on_disk` pass unchanged (presets store their values)
- [ ] 3.2 Add the preset and host migrations (Decision 3); verify 2.1's migration tests pass and every file in `tests/fixtures/presets/legacy/` loads with the old value
- [ ] 3.3 Check the randomizer and kit Assign against the new default; verify their tests pass and record any base value that moved

## 4. Guide, Wiki and docs

- [ ] 4.1 Run `cargo run --bin export_guide_data` and update the Spanish entries of any parameter whose default changed in `web/guide/src/data/parameters.es.json`; verify `guide_data_equals_the_files_on_disk` and `npm run check -- --complete` pass
- [ ] 4.2 Regenerate the guide clips (`scripts/build-guide-clips.sh`), since the default patch changed; verify the clip count is unchanged and note the regeneration in the close-out
- [ ] 4.3 Retake the guide screenshots (`node web/scripts/screenshots.cjs`) if a changed default moves a drawn control; verify the callouts still point at the right parts
- [ ] 4.4 Update the user Wiki in English and Spanish: Getting started (what the first note sounds like), every walkthrough that starts from the default patch with values as the plugin now shows them, and Troubleshooting if it mentions the long ring; verify `npm run check -- --complete` passes
- [ ] 4.5 Update `docs/ARCHITECTURE.md` and the Internals pages that list defaults or describe the default patch (`grep -rln "default patch" docs/internals/`, plus Parameters and Presets-and-Migrations for the migrations); verify `cargo test --no-default-features --test internals_wiki` passes
- [ ] 4.6 Remove the default-patch item from `docs/BACKLOG.md`; verify it is gone

## 5. Close-out

- [ ] 5.1 Run fmt, clippy `-D warnings`, `cargo test --workspace --no-default-features`, `cargo test --lib`, `no_alloc` and `cargo doc`; verify all pass and record the counts
- [ ] 5.2 Update "Current state" in `CLAUDE.md`, commit on `dev` and push; verify `git log origin/dev` shows it

## Workflow follow-up

- Archive with `/opsx:archive`; the new `default-patch` capability syncs into `openspec/specs/`.
