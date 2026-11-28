# Tasks

## 1. Tests first

- [ ] 1.1 Add `voice` tests from the review's table: Pneumatic Jet, Electromagnetic Hum and Particle Rain on Pipe C4, Env Attack 0.05 s, Decay 0.3 s, Sustain 0, Release 0.5 s, released at 0.1 s; assert the force envelope is 0 within Release + 10 ms and the voice-manager output RMS at 19–20 s is 0. Verify they fail on current `dev` (the RMS is about 0.07).
- [ ] 1.2 Add a test for Sustain 0.1 released at 0.1 s: the envelope reaches 0 between 0.45 and 0.55 s after note-off. Verify it fails today (about 5 s).
- [ ] 1.3 Add a bit-identity test: a specialty note with Sustain 0.7 held past Decay and released renders bit-identical to a stored render made on current `dev` (or compare against a test-only switch that uses the old slope). Verify it passes before and after.
- [ ] 1.4 Add a test that a note released in Sustain is in the Release phase on the sample after `note_off` (guards the N6 removal). Verify it passes on current `dev`.

## 2. Fix

- [ ] 2.1 Add `adsr_release_from` to `Voice`, set it in `release_force_envelope` when the ADSR moves to Release, clear it in `configure_envelope` and `reset`; use it for the release slope in `process_force_envelope`. Verify 1.1–1.4 pass.
- [ ] 2.2 Remove the unreachable `!self.active` branch of the ADSR's Sustain arm (N6). Verify 1.4 and `cargo test --workspace --no-default-features` pass.
- [ ] 2.3 Run `cargo test --no-default-features --test no_alloc` and verify it passes (the new field adds no allocation).

## 3. Presets

- [ ] 3.1 Run the bank tests (`presets::factory::tests`) and `curated_measurements_table` (ignored) before and after; record the specialty presets' rows that moved, and `percussion/chain_snap`, in design.md Measurements. Verify the bank tests pass.
- [ ] 3.2 Rerun `cargo run --release --bin seed_presets`; if `levels.rs` or files change, commit them and say what moved; verify `generated_bank_equals_the_files_on_disk` passes.

## 4. Docs

- [ ] 4.1 Update `docs/ARCHITECTURE.md`'s envelope description and Internals `docs/internals/Envelopes-and-Modulation.md` (the ADSR section: Release ramps from the note-off level; remove the "Trap: Sustain 0 plus an early note-off" paragraph; name the new test). Verify `cargo test --no-default-features --test internals_wiki` passes.
- [ ] 4.2 Update the user Wiki's ADSR section (`web/guide/src/content/docs/exciters/envelopes.mdx` and `es/`) to say Release is the time to silence from wherever the note was, in the guide's voice; add to Troubleshooting (EN + ES) only if a specialty note can still surprise. If `env_release`'s ⓘ text changes, run `cargo run --bin export_guide_data` and update `parameters.es.json`. Verify `npm run check -- --complete` in `web/guide/` passes.
- [ ] 4.3 Mark C1 and N6 FIXED in `FABLE-CODE-REVIEW-2026-11-14.md` (date, change, the tests that prove it) and remove C1 and N6 from `docs/BACKLOG.md`. Verify by reading both files.

## 5. Close-out

- [ ] 5.1 Run `cargo fmt --check`, `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`, `cargo test --workspace --no-default-features`, `no_alloc` and `internals_wiki`; verify all pass.
- [ ] 5.2 Update CLAUDE.md "Current state" (date, a row for this change, what's next), commit on `dev` and push; verify `git status` is clean.

## Workflow follow-up

- Archive with `/opsx:archive` once the maintainer has listened to the specialty presets' releases.
