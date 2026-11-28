# Tasks

## 1. Spread proximity (R4)

- [ ] 1.1 Add `hrtf_spread` tests: third-octave levels of noise at Width 0.5 and Proximity 0/0.5/1 at 44.1 and 96 kHz within 0.5 dB of 48 kHz, and a 48 kHz bit-identity test against the literal old filter expressions; verify the rate test fails on current `dev` and the identity test passes.
- [ ] 1.2 Cache the rate-converted proximity coefficients in `set_sample_rate` (design decision 1); verify 1.1 passes.

## 2. Spring dispersion (R4)

- [ ] 2.1 Add `space` tests: the Spring's first-echo spectrum at Wire Stiffness 0/0.5/1 at 96 kHz within 1 dB of 48 kHz per third-octave band up to 16 kHz, plus a 48 kHz bit-identity test; verify the rate test fails on current `dev`.
- [ ] 2.2 Convert the dispersion coefficient with `rate::smoother` outside the per-sample path (design decision 2); verify 2.1 and the existing `spring_delay_does_not_depend_on_the_sample_rate` pass.

## 3. Corrugated Drag ridge phase (R5)

- [ ] 3.1 Add `corrugated_drag` tests for the two `friction-exciters` scenarios (10-minute drone at default speed and spacing; Drag Speed 2.5 with Ridge Spacing 0.01), counting bumps over the first and last second; verify the drone test fails on current `dev`.
- [ ] 3.2 Replace the position with a wrapped phase and a cached `1 / spacing` (design decision 3); verify 3.1 passes and `cargo test --workspace --no-default-features` passes.
- [ ] 3.3 Run the bank tests and `curated_measurements_table` before and after; record the Corrugated Drag presets' level changes in design.md Measurements; if `seed_presets` changes `levels.rs` or files, commit them; verify the bank tests and `generated_bank_equals_the_files_on_disk` pass.

## 4. Docs

- [ ] 4.1 Update Internals `docs/internals/Body-Spread-and-Space.md` (proximity and dispersion are rate-converted), `Sample-Rate-Rule.md` (remove these two from any list of exceptions, add them as examples if listed), `Friction-Exciters.md` (Corrugated Drag's per-sample formula uses a wrapped phase) and `docs/ARCHITECTURE.md`; verify `cargo test --no-default-features --test internals_wiki` passes.
- [ ] 4.2 Mark R4 and R5 FIXED in `FABLE-CODE-REVIEW-2026-11-14.md` with the date, this change and the proving tests; remove them from `docs/BACKLOG.md`. Verify by reading both.

## 5. Close-out

- [ ] 5.1 Run `cargo fmt --check`, clippy with `-D warnings`, `cargo test --workspace --no-default-features`, `no_alloc` and `internals_wiki`; verify all pass, and that `mode_banks_match_golden` still passes (no golden regeneration expected).
- [ ] 5.2 Update CLAUDE.md "Current state", commit on `dev` and push; verify `git status` is clean.

## Workflow follow-up

- Archive with `/opsx:archive`.
