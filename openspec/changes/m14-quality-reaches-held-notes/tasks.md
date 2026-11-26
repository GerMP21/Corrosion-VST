# Tasks

## 1. Demonstrate and decide

- [ ] 1.1 Add an ignored test that prints, per object (C2/C4/C6, Damage 0 and 1), whether the Eco bank's frequencies are a prefix of the Render bank's, and the RMS change of a held note whose top modes are dropped; record the table in design.md Measurements
- [ ] 1.2 Render two clips for the maintainer (a held Render drone switched to Eco: today's behaviour, and a prototype of option A) with CPU figures from `print_` timing or the benchmark; record the choice (A or B) in design.md

## 2. Tests first

- [ ] 2.1 Under A: write the modal-bank scenarios (mode count within 50 ms, no click, upward switch bit-identical, fixed quality bit-identical) as tests and verify the first two fail on current code. Under B: write `held_notes_keep_their_mode_count_on_a_quality_change` and verify it passes, then switch the spec delta to B's requirement and run `openspec validate m14-quality-reaches-held-notes`

## 3. Trim held notes on a downward switch (A only)

- [ ] 3.1 Pass the live quality scale to the voice manager per block and notify sounding voices on a downward change; verify a unit test sees each voice receive it once
- [ ] 3.2 Add the active-mode limit and the 20 ms fade to `ModalResonator`, cold path out of line; verify the group 2 tests pass and `cargo test --no-default-features --test no_alloc` passes with a Quality switch over held voices added
- [ ] 3.3 Prove fixed-quality renders bit-identical (render hash against a clean worktree, gnu) and measure `voice_manager_sixteen_voices` with `scripts/bench_ratios.py 5`; record both in design.md
- [ ] 3.4 Confirm `mode_banks_match_golden_fixture` is unchanged (note-on banks don't change) and the bank tests pass

## 4. Docs

- [ ] 4.1 Update README (Quality), `docs/ARCHITECTURE.md` (Quality modes), Internals `Modal-Resonator.md` ("Mode count and quality") and `Voices-and-Polyphony.md`, and the Wiki's `quality.mdx` in English and Spanish, to say what a downward and an upward switch do to held notes; verify `npm run check` in `web/guide/` and `cargo test --no-default-features --test internals_wiki` pass

## 5. Close-out

- [ ] 5.1 Update M14's note in `docs/archive/FABLE-CODE-REVIEW-2026-07-07.md` (date, change, what proves it) and remove M14 from `docs/BACKLOG.md`; verify by reading both
- [ ] 5.2 Run `cargo fmt --check`, clippy with `-D warnings`, `cargo test --workspace --no-default-features`, `cargo test --lib`, `no_alloc` and the internals_wiki test; verify all pass
- [ ] 5.3 Update CLAUDE.md "Current state" and commit and push to `dev`; verify the push

## Workflow follow-up

- Archive with `/opsx:archive`; the MODIFIED requirement syncs into `openspec/specs/modal-bank/spec.md` (A only).
