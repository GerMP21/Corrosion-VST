# Proposal

## Why

Three checks the project relies on are manual today (`docs/BACKLOG.md`, "Testing and release infrastructure"): loading the plugin in a DAW (REAPER on Linux, by hand, following `docs/REAPER_TESTING.md`), noticing that the editor looks wrong (the GUI tests drive the editor headlessly but compare no rendered images), and noticing a CPU regression (`scripts/bench_ratios.py` compares twins inside one run; nothing remembers yesterday's numbers). Each has caught or missed real bugs in recent changes; automating them where it is honest to do so, and saying plainly where it isn't, removes guesswork from close-outs.

## What Changes

Three phases, each landable on its own:

1. **REAPER smoke test, local and cloud-session only.** `tests/daw/reaper-smoke.sh` scripts the recipe in `docs/REAPER_TESTING.md`: a virtual display, REAPER with a Lua startup script that inserts the CLAP plugin on a track with a MIDI item, renders it to WAV, saves and reopens the project, then quits. The script checks the WAV (`scripts/check_wav.py`: not silent, finite, under the ceiling) and that the reopened project restores a non-default parameter. It exits non-zero on any failure and says "skipped" (exit code 77) when REAPER isn't installed, never "passed". It does not run in CI (REAPER's licence and download aren't fit for CI runners); other DAWs (Bitwig, Ardour, Live, FL) are out of scope and listed as later work.
2. **Visual regression for the editor, in CI.** The web app draws the same egui editor (`src/gui/editor.rs`) the plugin does, and CI's `web` job already serves it in Chromium. A Playwright script captures fixed views (default patch, Kit mode, a friction exciter's panel, the preset browser open) at a fixed window size and compares them with reference PNGs in the repository, failing above a small pixel-difference threshold and writing a diff image as a CI artifact. References are updated by an explicit command, never automatically.
3. **Benchmark baselines, local.** `scripts/bench_baseline.py save|compare` stores Criterion medians of 5 runs per machine (outside the repository, keyed by host name and CPU) and compares a later run against them, flagging any benchmark slower by more than a threshold above the measured noise (±10–15 % in the cloud container). Within-run ratio bounds (the H12 bounds of `bench_ratios.py`) stay the portable check; absolute baselines are never committed, because they don't transfer between machines.

- No change to the plugin, its sound, presets or parameters.

## Capabilities

### New Capabilities

- `daw-smoke-tests`: what the scripted REAPER smoke test proves, where it runs, and how it reports skips.
- `gui-visual-regression`: reference images of the editor checked in CI, and how they are updated.
- `benchmark-baselines`: saving and comparing per-machine benchmark baselines.

### Modified Capabilities

(none)

## Impact

- New files: `tests/daw/reaper-smoke.sh` (+ its Lua script), `web/scripts/visual.cjs`, reference PNGs (a few hundred KB) under `web/scripts/visual-references/` or `tests/fixtures/visual/`, `scripts/bench_baseline.py`.
- CI: the `web` job runs the visual check and uploads diffs on failure.
- Docs: `docs/REAPER_TESTING.md`, `docs/ARCHITECTURE.md`, `CLAUDE.md` (Build and test, Benchmarks), Internals pages `Testing.md`, `Benchmarks-and-Performance.md`, `CI-and-Releases.md`, `Web-App.md`, `Repository-Layout.md` (each new top-level item of `tests/`, `scripts/`, `web/` must be named, or `internals_wiki` fails).
- Depends on `fix-test-and-ci-scripts` (it removes the `tests/daw/` placeholders first). `fix-web-canvas-dpr` should land first too, so the reference images aren't taken from a cut-off canvas; `split-largest-source-files` doesn't affect it.
