# Tasks

## 1. Baseline

- [ ] 1.1 Record the current line ranges, `cargo test --workspace --no-default-features -- --list` and `cargo test --lib -- --list` (sorted), the 48 kHz render hashes (`print_48k_exciter_render_hashes`) and a 5-run benchmark baseline from a clean worktree; verify all four are saved in the change folder and summarized in design.md

## 2. Move, one file per commit

- [ ] 2.1 Move `lib.rs`'s nine test modules to `src/tests/` with `#[path]` declarations; verify the test lists are identical and `cargo test --workspace --no-default-features` passes
- [ ] 2.2 Move `voice/mod.rs`'s five test modules to sibling files and split envelope, expression and render into `src/voice/`; verify identical test lists, identical render hashes and `cargo test --no-default-features --test no_alloc`
- [ ] 2.3 Move `core.rs`'s tests to `src/dsp/resonators/core_tests.rs`; verify identical test lists and `mode_banks_match_golden_fixture` passes
- [ ] 2.4 Split `params.rs` into `src/params/enums.rs`, `constructors.rs` and `tests.rs`; verify identical test lists and `guide_data_equals_the_files_on_disk` and the preset tests pass
- [ ] 2.5 Split `editor.rs` into the `src/gui/editor/` children listed in design.md; verify identical `cargo test --lib -- --list`, `cargo test --lib` passes (GUI lane) and `web/scripts/build.sh` builds

## 3. Proof and documentation

- [ ] 3.1 Compare render hashes and run the benchmarks against the baseline (5 interleaved runs, medians, output hashes), recording the results in design.md; verify hashes are identical and every median ratio is within noise (≤ 1.05) or explained
- [ ] 3.2 Update every Internals page `grep -rl` finds for the five files and the moved functions, and name each new non-test file in a `## Source files` list; verify `cargo test --no-default-features --test internals_wiki`
- [ ] 3.3 Update `docs/ARCHITECTURE.md` and CLAUDE.md where they describe file layout; mark O6 FIXED in `FABLE-CODE-REVIEW-2026-11-14.md` and remove it from `docs/BACKLOG.md`; verify neither lists O6 as open

## 4. Close out

- [ ] 4.1 Run `cargo fmt --check`, clippy with `-D warnings` (CI lane), `cargo test --workspace --no-default-features`, `cargo test --lib`, `no_alloc`, `internals_wiki`, `cargo doc --no-deps --no-default-features`; verify all pass with no new warnings
- [ ] 4.2 Update CLAUDE.md "Current state" and push the commits to `dev`; verify the push

## Workflow follow-up

- Archive with `/opsx:archive split-largest-source-files`.
