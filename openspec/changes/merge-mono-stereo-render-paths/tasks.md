# Tasks

## 1. Baseline

- [ ] 1.1 Add an ignored test that prints render hashes for the voice manager's mono path and stereo path (widths 0, 0.5, 1) over the 16 exciters' default notes, the 9 objects, a bent note, a dynamic object and a kit pad at 48 and 96 kHz; record its output from a clean worktree in the change folder; verify it runs on both trees
- [ ] 1.2 Add the stereo benchmark twins `voice_manager_sixteen_voices_stereo` and `voice_manager_sixteen_voices_bent_stereo` and record a 5-run baseline (with `bench_ratios.py 5`) from the clean worktree in design.md; verify the benchmarks run

## 2. Resonator

- [ ] 2.1 Merge `ModalResonator::process_sample` and `process_sample_stereo` into one `#[inline(always)]` mode loop generic over the output, removing the dead profile `match` if still there; verify the hash test matches the baseline, `mode_banks_match_golden_fixture` and `cargo test --no-default-features resonators` pass
- [ ] 2.2 Settle the empty-bank case per design.md Decision 2 with a test (silent output, next note unaffected); verify the test passes and the hashes still match

## 3. Voice

- [ ] 3.1 Merge `Voice::process_sample` and `process_sample_stereo` into one generic pipeline and drop `process_exciter_stereo` if still there; verify the hash test matches the baseline, `cargo test --no-default-features voice` and `cargo test --no-default-features --test no_alloc` pass

## 4. Proof and documentation

- [ ] 4.1 Run the twins and `bench_ratios.py 5` against the baseline (interleaved, medians, output hashes compared) and record them in design.md; verify every median ratio is within noise (≤ 1.05) or explained
- [ ] 4.2 Update `docs/ARCHITECTURE.md` and the Internals pages `Voices-and-Polyphony.md`, `Modal-Resonator.md`, `Signal-Chain.md` (one pipeline, two entry points) and `Benchmarks-and-Performance.md` (the stereo twins); verify `cargo test --no-default-features --test internals_wiki`
- [ ] 4.3 Mark O7 FIXED in `FABLE-CODE-REVIEW-2026-11-14.md` and remove it from `docs/BACKLOG.md`; verify neither lists it as open

## 5. Close out

- [ ] 5.1 Run `cargo fmt --check`, clippy with `-D warnings`, `cargo test --workspace --no-default-features`, `no_alloc`, `internals_wiki` and the bank tests; verify all pass with no preset moved
- [ ] 5.2 Update CLAUDE.md "Current state" and commit and push to `dev`; verify the push

## Workflow follow-up

- Archive with `/opsx:archive merge-mono-stereo-render-paths`.
