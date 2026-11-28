# Tasks

## 1. REAPER smoke test (phase 1, local and cloud sessions)

- [ ] 1.1 Write `tests/daw/reaper-smoke.sh` and its Lua startup script following `docs/REAPER_TESTING.md`: build the project, render, save with a non-default parameter, reopen, check the WAV with `scripts/check_wav.py` and the restored parameter; verify on a cloud session with REAPER that it exits 0 on a fresh CLAP bundle and record the run time
- [ ] 1.2 Verify the failure paths: exits 77 with "skipped" without REAPER, and exits non-zero naming the WAV check with a bundle patched to output silence (a scratch build, not committed)
- [ ] 1.3 Update `docs/REAPER_TESTING.md` (one-command section), `docs/internals/Testing.md` and `Repository-Layout.md` (name `tests/daw`), `CLAUDE.md`'s Build and test notes; verify `cargo test --no-default-features --test internals_wiki` passes

## 2. Visual regression (phase 2, CI)

- [ ] 2.1 Write `web/scripts/visual.cjs` (Playwright, fixed viewport, ratio 1, audio not started) capturing the four views, with `--update` writing `tests/fixtures/visual/*.png`; generate the references and verify two consecutive runs pass
- [ ] 2.2 Tune the per-pixel tolerance and the failing share so a 4 px shift of one panel (a scratch edit) fails and the unchanged tree passes in CI's SwiftShader; record both in design.md
- [ ] 2.3 Run it in CI's `web` job after `smoke.cjs`, uploading captured and difference images on failure; verify the job passes on `dev` and fails on a scratch branch with the shifted panel
- [ ] 2.4 Update `docs/internals/Web-App.md`, `CI-and-Releases.md`, `Testing.md`, `docs/ARCHITECTURE.md`, and CLAUDE.md's "After an editor change" note (run `visual.cjs --update` with the screenshots); verify `internals_wiki` passes

## 3. Benchmark baselines (phase 3, local)

- [ ] 3.1 Write `scripts/bench_baseline.py save|compare` (5 runs, median, machine key, threshold, refusal on another machine's baseline or on a noisy save); verify two compares on an unchanged tree flag nothing and a scratch slowdown (an added busy loop in the benchmark) is flagged with exit code 1
- [ ] 3.2 Update `CLAUDE.md`'s Benchmarks section, `docs/internals/Benchmarks-and-Performance.md` (name `scripts/bench_baseline.py`), `docs/ARCHITECTURE.md`; verify `internals_wiki` passes

## 4. Close-out

- [ ] 4.1 `cargo fmt --check`, clippy `-D warnings`, `cargo test --workspace --no-default-features`, `web/scripts/build.sh` pass
- [ ] 4.2 Remove the DAW smoke automation, visual regression and benchmark baseline items from `docs/BACKLOG.md` (keep a line for the other DAWs), update CLAUDE.md "Current state", commit and push to `dev`

## Workflow follow-up

- Phases can be committed separately; archive with `/opsx:archive` after all three.
- Other DAWs (Bitwig, Ardour, Live, FL) stay in the backlog as later work.
