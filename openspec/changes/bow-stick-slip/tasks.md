# Tasks

## 1. Prototype and listen (maintainer)

- [ ] 1.1 Confirm `fix-internals-wiki-findings`' Bow ripple fix and `decide-drone-loop-step` are merged on `dev` (this change starts after them); verify with `git log` and record the base commit in design.md `## Measurements`
- [ ] 1.2 Add an ignored measurement test `print_bow_stick_fraction` that reports, for a held Bow note, the stick fraction, the slip interval against the fundamental's period, the strongest peak in cents and the largest subharmonic peak in dB; run it on the current sliding bow (expected stick fraction 0) and record the table
- [ ] 1.3 Implement Decisions 1–3 behind a test-only switch `BowExciter::stick_slip_enabled`, and run 1.2 on the default patch at C2/C3/C4 on Taut Cable, Pipe, I-Beam and Plate; verify stick fractions between 20 % and 90 % and no subharmonic within 20 dB, or stop and report with the numbers
- [ ] 1.4 Render before/after clips of Bowed Cable Sub, Pipe Bow Lead, Sheet Bow Organ and Bowed Beam Drone into the scratchpad; verify the maintainer's verdict on the model and on Decision 4 is recorded in design.md (*As built*)

## 2. Tests first

- [ ] 2.1 Write the `bow-exciter` spec scenarios as tests (stick-slip once per period, the sliding bow gone, pitched and subharmonic-free at defaults, the Pressure sweep, three rates); verify each fails on the current sliding bow
- [ ] 2.2 Add a bank test that runs the subharmonic check over every factory preset using the Bow; verify it passes on today's (sliding) bank, as a guard for the re-voicing

## 3. Implement

- [ ] 3.1 Make the stick-slip model the only Bow model (remove the switch, or keep it test-only for the comparison) and pass the contact prediction from `src/voice/mod.rs`; verify 2.1 passes and `tests/no_alloc.rs` passes in debug
- [ ] 3.2 Re-tune `EXCITER_DRIVE_LEVEL[1]` with `print_exciter_overdrive`; verify `exciters_play_at_a_common_level` and `default_notes_do_not_overdrive_the_voice` pass, and record the old and new level
- [ ] 3.3 Check Rosin Grip, Slip Curve, Bow Pressure, Bow Speed and the global Pressure/Speed/Roughness still act; verify `every_sound_shaping_control_changes_its_note` and the exciter-controls tests pass, and update any help text whose direction changed (EN in `src/gui/controls.rs`)
- [ ] 3.4 Measure CPU with 16 bowed voices (`python3 scripts/bench_ratios.py 5`); verify the median ratio and record it (report if above 1.05×)

## 4. Re-voice the bank

- [ ] 4.1 Re-voice the 22 curated Bow presets in `src/presets/factory/curated.rs` and adjust the Bow ranges in `experimental.rs`; verify the bank tests pass (folder character, quality floor, 2.2)
- [ ] 4.2 Reseed (`cargo run --release --bin seed_presets`); verify `generated_bank_equals_the_files_on_disk` passes and record what moved (`curated_measurements_table` before and after for the bowed presets) in Measurements
- [ ] 4.3 Render the re-voiced presets for the maintainer and record their verdict; verify no preset they flagged is left unchanged

## 5. Guide, Wiki and docs

- [ ] 5.1 Run `cargo run --bin export_guide_data` (help texts or the bank changed) and fix the Spanish entries in `web/guide/src/data/parameters.es.json`; verify `guide_data_equals_the_files_on_disk` and `npm run check -- --complete` pass
- [ ] 5.2 Regenerate the guide clips (`scripts/build-guide-clips.sh`), since the bowed sound moved; verify the clip count is unchanged and note the regeneration in the close-out
- [ ] 5.3 Update the user Wiki in English and Spanish: the Bow page (stick-slip, the playable region, the raucous edge as a sound to use), any recipe using a bowed preset, and Troubleshooting ("my bow growls or drops an octave"); verify `npm run check -- --complete` passes
- [ ] 5.4 Update `docs/ARCHITECTURE.md` and the Internals pages naming `bow.rs` or the bow (`grep -rl bow docs/internals/`; at least Friction-Exciters, Exciters, Interaction-Bus-and-Contact, Factory-Bank); verify `cargo test --no-default-features --test internals_wiki` passes
- [ ] 5.5 Remove "Real stick-slip for the Bow" from `docs/BACKLOG.md`; verify it is gone

## 6. Close-out

- [ ] 6.1 Run fmt, clippy `-D warnings`, `cargo test --workspace --no-default-features`, `cargo test --lib`, `no_alloc` and `cargo doc`; verify all pass and record the counts
- [ ] 6.2 Update "Current state" in `CLAUDE.md`, commit on `dev` and push; verify `git log origin/dev` shows it

## Workflow follow-up

- Archive with `/opsx:archive` after the maintainer has heard the re-voiced presets.
