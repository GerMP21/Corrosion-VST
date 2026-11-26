# Tasks

## 1. Listen and decide

- [ ] 1.1 Render A/B clips (today, A1, A2) of three damaged presets from the bank (a percussion preset, a bowed plate with Damage, an experimental one with high Damage), with each clip's rattle L/R correlation and RMS; give them to the maintainer and record the choice in design.md

## 2. Tests first

- [ ] 2.1 Add a test-only switch on the voice to disable the rattle and to force the shared rattle, then write the material-transforms scenarios (correlation below 0.1, per-channel RMS within 1 dB, Damage 0 bit-identical) and verify the correlation test fails on current code (under B: write the left = right test instead, verify it passes, and change the spec delta to B's requirement)

## 3. Implement (A1 or A2)

- [ ] 3.1 Add the second, per-note-seeded noise stream and split the gate from the noise in the stereo path, keeping the mono path's single stream; verify the group 2 tests pass, the mono path renders bit-identical (hash against a clean worktree), and `cargo test --no-default-features --test no_alloc` passes
- [ ] 3.2 Under A2: add the rate-converted one-pole high-pass and its make-up gain, with the low/high energy test and a rate test (44.1, 48, 96 kHz); verify both pass
- [ ] 3.3 Run the bank tests (`presets::factory::tests`), reseed with `cargo run --release --bin seed_presets` if levels moved, and record what moved; verify `generated_bank_equals_the_files_on_disk` passes
- [ ] 3.4 Regenerate the guide clips (`scripts/build-guide-clips.sh`) and record it in design.md

## 4. Docs

- [ ] 4.1 Update the comment in `rattle_noise`, `docs/ARCHITECTURE.md`, Internals `Material-Transformations.md` and `Voices-and-Polyphony.md`, and the Damage text in `objects/environmental-exposure.mdx` (EN and ES) to describe the stereo rattle; verify `npm run check` in `web/guide/` and `cargo test --no-default-features --test internals_wiki` pass

## 5. Close-out

- [ ] 5.1 Update M17's note in `docs/archive/FABLE-CODE-REVIEW-2026-07-07.md` (date, change, proof) and remove M17 from `docs/BACKLOG.md`; verify by reading both
- [ ] 5.2 Run `cargo fmt --check`, clippy with `-D warnings`, `cargo test --workspace --no-default-features`, `cargo test --lib`, `no_alloc` and the internals_wiki test; verify all pass
- [ ] 5.3 Update CLAUDE.md "Current state" and commit and push to `dev`; verify the push

## Workflow follow-up

- Archive with `/opsx:archive` after the maintainer has listened to the damaged presets.
