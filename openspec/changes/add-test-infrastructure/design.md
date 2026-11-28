# Design

## Context

- `docs/REAPER_TESTING.md` already documents a working manual recipe from a cloud session: Xorg dummy display on `:98`, REAPER with a startup script that loads the CLAP plugin, a screenshot build, `xdotool` and ImageMagick, and Lua for other actions (its sections 2–6). The placeholders in `tests/daw/` are removed by `fix-test-and-ci-scripts`.
- The GUI lane (`cargo test --lib`) drives the editor headlessly (interaction, hover, layout, a few pixel checks); nothing compares whole rendered views.
- CI's `web` job builds the web app, installs Playwright 1.63 with Chromium and runs `web/scripts/smoke.cjs` against a local server (`.github/workflows/ci.yml:194-208`); `web/scripts/screenshots.cjs` already captures editor regions deterministically for the Wiki.
- `scripts/bench_ratios.py 5` reports the H12 bounds as medians of within-run ratios; the container's run-to-run noise is ±10–15 % per benchmark (CLAUDE.md, Benchmarks).

## Goals / Non-Goals

**Goals:** a scripted REAPER check anyone with REAPER (or a cloud session following the recipe) can run in one command; a CI visual check of the editor; a local regression check for CPU.

**Non-Goals:** DAWs other than REAPER (Bitwig, Ardour, Live, FL: later, each its own change); running REAPER in CI; pixel tests of the native baseview window; committed absolute benchmark numbers; a benchmark job in CI (shared runners are too noisy to gate on).

## Decisions

1. **REAPER via Lua, not GUI clicks.** The startup script builds the project (track, CLAP FX, MIDI item), sets a parameter, renders with REAPER's render action, saves, reopens and reads the parameter back, writing a small result file the shell script checks. Clicks with `xdotool` are kept for nothing but an optional editor screenshot. Exit code 77 for "skipped" follows the Automake/meson convention, so wrappers can tell skip from pass.
2. **Visual regression on the web editor.** It is the same `src/gui/editor.rs` the plugin draws, already runs in CI, and renders deterministically in SwiftShader. The native window would need REAPER or a host in CI. Views are captured at device pixel ratio 1 in a fixed viewport, after a fixed wait, with animations that depend on audio (meters, the diode dot) excluded by not starting audio.
3. **Comparison by pixel share with a per-pixel tolerance** (for example a pixel counts as different when any channel differs by more than 16, and the view fails above 0.5 % of pixels), implemented in Node with `pngjs` + `pixelmatch` (npm, MIT/ISC; added to `THIRD-PARTY-NOTICES.md` only if the site ships them, which it doesn't: dev-only). Thresholds are tuned in task 2.2 so two runs of an unchanged tree pass and a 4 px panel shift fails.
4. **References in the repository** (`tests/fixtures/visual/`, a few PNGs at 1280 px wide), updated by `node web/scripts/visual.cjs --update`. CI never writes them.
5. **Benchmark baselines in the user's cache** (`~/.cache/corrosion-bench/<host>-<cpu>.json`), from Criterion's estimates of 5 runs, median per benchmark. The flag threshold defaults to 1.25× (above the container's ±15 %), overridable per run. Within-run ratios (`bench_ratios.py`) remain the CI-portable bound.

## Risks / Trade-offs

- [REAPER versions change their actions or Lua API] → the script records the REAPER version it ran with; failures name the step.
- [SwiftShader output drifts with a Chromium update] → Playwright is pinned (`playwright@1.63.0` in CI); a Playwright bump comes with regenerated references in the same change.
- [Visual references go stale every editor change and become noise] → only four views; the update command is one line; CLAUDE.md's "after an editor change" note gains it next to the screenshots step.
- [Baselines saved on a busy machine] → the save command refuses when 5 runs' spread exceeds the threshold and asks to rerun.

## Measurements

(filled in during apply: REAPER smoke run time, visual check run time in CI and the tuned thresholds, two-run noise of the baseline compare)
