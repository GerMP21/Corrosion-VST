# Tasks

## 1. Listen and decide (maintainer)

- [ ] 1.1 Implement the blend law behind a test-only switch on `MSEG` and render A/B clips (Env Amount 1, 0.5 and 0 on Bow · Plate and Corrugated Drag · Pipe, plus two experimental presets near 0.55) under both laws into the scratchpad; verify the clips exist and the old law's Env Amount 0 clips are silent
- [ ] 1.2 Record the maintainer's choice (blend or keep) in design.md, Decision 1 *As built*; if keep, replace the spec delta with the "keep" requirement from Decision 1, set the tasks below that change the sound to not applicable, and skip to group 4

## 2. Tests first

- [ ] 2.1 Write the modified and added `mseg-envelope` scenarios as tests in `src/dsp/envelopes/mod.rs` (flat share at zero level, full velocity gives `a·level + (1 − a)`, Bow at Env Amount 0 audible and released within MSEG Release) and adjust the existing tests whose expected value multiplied by Env Amount; verify the new ones fail on the current law
- [ ] 2.2 Add the bit-identity test (old law vs new law at Env Amount 1 over the loop-mode, start and end sweep); verify it passes trivially before the implementation, as the reference

## 3. Implement

- [ ] 3.1 Implement Decision 1 in `MSEG::process_sample`, `release` and `velocity_scaled`; verify 2.1 and 2.2 pass, the exhaustive loop sweep stays finite and in [0, 1], and `tests/no_alloc.rs` passes in debug
- [ ] 3.2 Compare `print_48k_exciter_render_hashes` against a clean worktree; verify the five friction exciters and all others match, and record the hashes in design.md `## Measurements`
- [ ] 3.3 Update Env Amount's help text in `src/gui/controls.rs` ("How much the friction envelope shapes the force. At 0 the exciter pushes steadily; at 1 it follows the envelope.") and check `every_sound_shaping_control_changes_its_note` still lists Env Amount as acting; verify the GUI lane and control-response tests pass
- [ ] 3.4 Reseed (`cargo run --release --bin seed_presets`) and run the bank tests; verify they pass and record which experimental presets moved and by how much (`preset_levels_table` before and after)

## 4. Guide, Wiki and docs

- [ ] 4.1 Run `cargo run --bin export_guide_data` and update Env Amount's Spanish entry in `web/guide/src/data/parameters.es.json`; verify `guide_data_equals_the_files_on_disk` and `npm run check -- --complete` pass
- [ ] 4.2 Regenerate the guide clips (`scripts/build-guide-clips.sh`) if the sound changed; verify the Env Amount clip row now has sound at its minimum, and note the regeneration in the close-out
- [ ] 4.3 Update the user Wiki in English and Spanish: Gesture Modifiers and envelopes pages, and Troubleshooting (remove or rewrite "my friction exciter is silent"); verify `npm run check -- --complete` passes
- [ ] 4.4 Update `docs/ARCHITECTURE.md` and the Internals pages that describe the MSEG output law (`grep -rl envelopes/mod.rs docs/internals/`: Envelopes-and-Modulation, Friction-Exciters, Testing); verify `cargo test --no-default-features --test internals_wiki` passes
- [ ] 4.5 Remove the Env Amount item from `docs/BACKLOG.md`; verify it is gone

## 5. Close-out

- [ ] 5.1 Run fmt, clippy `-D warnings`, `cargo test --workspace --no-default-features`, `cargo test --lib`, `no_alloc` and `cargo doc`; verify all pass and record the counts
- [ ] 5.2 Update "Current state" in `CLAUDE.md`, commit on `dev` and push; verify `git log origin/dev` shows it

## Workflow follow-up

- Archive with `/opsx:archive`; the `mseg-envelope` deltas sync into `openspec/specs/`.
