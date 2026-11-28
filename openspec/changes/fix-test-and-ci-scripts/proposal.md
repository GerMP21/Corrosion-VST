# Proposal

## Why

Four pieces of test and release tooling say more than they do (`docs/BACKLOG.md`, "Testing and release infrastructure"): `render_presets` renders nothing with its default arguments, the `nightly` release can publish while the web app or the user Wiki is broken, the DAW scripts in `tests/daw/` print "PASSED" without rendering anything, and `scripts/verify-local.sh` claims to mirror CI while skipping several of its steps. `tests/AGENTS.md` sends agents to a `.sisyphus/evidence/` folder that doesn't exist. Each one misleads whoever trusts it.

## What Changes

- **`render_presets` walks subfolders.** Its default `--preset-dir presets/factory` finds every factory preset (they all sit in subfolders); output files keep the folder path in their names so `experimental/bow/pipe` and `experimental/drumstick/pipe` don't collide.
- **`nightly` waits for every job:** `needs: [bundle, web, guide]`, so a red web app or Wiki blocks the pre-release, as the workflow's comment already says.
- **`tests/daw/` placeholders are removed.** `gate-2-smoke.sh` and `run-reaper.sh` render nothing, and `run-reaper.sh` reports success unconditionally. Until `add-test-infrastructure` adds a real REAPER smoke test, `docs/REAPER_TESTING.md` is the documented manual check.
- **`verify-local.sh` runs what CI's `checks` job runs on Linux** (adds `cargo doc` with warnings denied, the separate `no_alloc` step, the benchmark compile) and its header lists what it still skips (the validators, the web and guide jobs, other platforms), so "passed" means what it says.
- **`tests/AGENTS.md`** drops the `.sisyphus/` lines and points at the real places (repo-root fixtures, `docs/REAPER_TESTING.md`).
- No change to the plugin, its sound or presets.

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `continuous-integration`: the nightly requires the web and guide jobs too; a new requirement for the local verification script.
- `preset-rendering`: the render tool's default command renders the whole factory bank.

## Impact

- Code and tooling: `src/bin/render_presets.rs`, `.github/workflows/ci.yml`, `scripts/verify-local.sh`, `tests/daw/` (deleted), `tests/AGENTS.md`.
- Docs: `docs/ARCHITECTURE.md` (CI and tools sections), `CLAUDE.md` (Build and test table, the `render_presets` note), Internals pages `CI-and-Releases.md`, `Testing.md`, `Repository-Layout.md`, `Building-and-Bundling.md`, `Factory-Bank.md`, `Home.md` (wherever they name these files; `tests/daw` must leave every page or `internals_wiki` fails).
- Order: before `add-test-infrastructure`, which adds the real DAW smoke test into a clean `tests/daw/`.
