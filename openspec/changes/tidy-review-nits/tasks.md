# Tasks

## 1. Baseline

- [ ] 1.1 Extend `tests/tidy_render_hash.rs` with a Specialty exciter note, a Drone friction note and a post-chain render with Space on; run it and `print_48k_exciter_render_hashes` on a clean worktree of the base commit (`git worktree add --detach /tmp/base HEAD`, own target dir, gnu) and record the hashes in design.md Measurements

## 2. Post-chain enum mapping (O8)

- [ ] 2.1 Write tests for `SpaceMode::from_int` and `PostQualityMode::from_int` (every value 0–3 and an out-of-range value giving Off and Normal) and verify they fail to compile or fail on current code
- [ ] 2.2 Add the constructors beside the enums and use them in `apply_post_chain_params`; verify the tests pass and the hashes match the baseline

## 3. Resonator and interaction tidy-ups (N1, N2, N3)

- [ ] 3.1 Replace the dead `match self.profile` with `mode_sum` (N1); verify the hashes match
- [ ] 3.2 Remove the default writes in `rebuild_in_place` and the fields if only tests read them, rewriting `src/dsp/tests.rs:450`, `:497`, `:576` to check the bank instead (N2); verify `cargo test --workspace --no-default-features` passes and the hashes match
- [ ] 3.3 Make `calculate_mode_coefficients` `#[cfg(test)]` and remove the P3.1 comments (N3); verify `cargo doc --no-deps --no-default-features` adds no warnings and the test at `interaction.rs:268` still runs
- [ ] 3.4 Update Internals `Modal-Resonator.md` and `Interaction-Bus-and-Contact.md` where they mention these fields or the function; verify `cargo test --no-default-features --test internals_wiki` passes

## 4. Voice and envelope tidy-ups (N4, N5, N7)

- [ ] 4.1 Inline `process_exciter_stereo` and rewrite the stale boost comment (N4); verify the hashes match
- [ ] 4.2 Remove the three redundant `set_sample_rate` calls at note-on (N5); verify the hashes match and `cargo test --no-default-features --test no_alloc` passes
- [ ] 4.3 Join the MSEG doc comment, cache `dt` in `set_sample_rate`, and drop the per-sample `mseg.set_sample_rate` calls, keeping the note-on call (`voice/mod.rs:843`) and adding one in `Voice::set_rate`'s rate-changed branch (N7); add a test that an MSEG at 96 kHz still times its stages in seconds; verify the hashes match
- [ ] 4.4 Update Internals `Envelopes-and-Modulation.md` and `Voices-and-Polyphony.md` where they describe these calls; verify the internals_wiki test passes

## 5. Vendor URL and clamp documentation (N8, N10)

- [ ] 5.1 Ask the maintainer whether `EMAIL` stays empty or gets a project address; set `URL` to `https://corrosion-vst.org/` (and `EMAIL` if given); verify with a unit test reading `Corrosion::URL` and by checking `clap-validator` output in CI
- [ ] 5.2 Update Internals `Plugin-Lifecycle.md` (the `URL`/`EMAIL` row and the CLAP line) to match; verify the internals_wiki test passes
- [ ] 5.3 Document every silent safety clamp (HRTF spread ±1.5, body ±10 per mode and ±1 overall, drive ±1, the filter's saturators) in `docs/ARCHITECTURE.md` and Internals `Post-Chain.md`, `Body-Spread-and-Space.md` and `Filter-and-Drive.md`, saying they are guards and not the diode stage (N10); verify each value against the code

## 6. Close-out

- [ ] 6.1 Re-run the hash scenarios on the final tree and record them next to the baseline in design.md Measurements; verify every hash is identical
- [ ] 6.2 Mark O8, N1–N5, N7, N8 and N10 FIXED in `FABLE-CODE-REVIEW-2026-11-14.md` (date, change, proof) and remove them from `docs/BACKLOG.md`; verify by reading both files
- [ ] 6.3 Run `cargo fmt --check`, `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`, `cargo test --workspace --no-default-features`, `cargo test --lib`, `cargo test --no-default-features --test no_alloc` and the internals_wiki test; verify all pass
- [ ] 6.4 Update CLAUDE.md "Current state" (date, row, next) and commit and push to `dev`; verify `git log origin/dev` shows the commit

## Workflow follow-up

- Archive with `/opsx:archive` once the maintainer has reviewed it.
