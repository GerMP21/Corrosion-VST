# Design

## Context

Re-checked on `dev` (2026-11-28) by reading the code; none was run yet:

- `render_presets` (`src/bin/render_presets.rs:18-19`, `:138-153`): `--preset-dir` defaults to `presets/factory` and `collect_preset_paths` reads only that folder's top level with `fs::read_dir`; every factory preset sits in a subfolder (`bass/`, `experimental/<exciter>/`, `drums/`…), so the default command finds nothing. Confirmed by reading. File names repeat across folders (`experimental/*/pipe.corrosion-preset`).
- `nightly` (`.github/workflows/ci.yml:371-374`): `needs: bundle`, and `bundle` needs `checks` (`:248-250`). `web` (`:148`) and `guide` (`:215`) are not upstream of it, though the comment says "after every job passed". Confirmed.
- `tests/daw/run-reaper.sh` starts nothing and prints "✅ REAPER smoke test PASSED" whenever REAPER and a bundle exist; `tests/daw/gate-2-smoke.sh` exits 0 when REAPER is missing and only greps an ini file. Neither renders audio; CI runs neither. Confirmed.
- `scripts/verify-local.sh` runs fmt, notices, clippy, workspace tests, the GUI lib tests, the renderer smoke and WAV check, and a Linux bundle; it skips `cargo doc` (CI denies warnings, `ci.yml:85-89`), the separate `no_alloc` step (`:70-72`) and the benchmark compile (`:91-93`), and says it "mirrors the repository CI lane". Confirmed.
- `tests/AGENTS.md:15-16` mention `.sisyphus/`, which doesn't exist. Confirmed.

## Goals / Non-Goals

**Goals:** each tool does what its name, comment or docs say; nothing reports success it didn't earn.

**Non-Goals:** real DAW automation, visual tests, benchmark baselines (`add-test-infrastructure`); running the validators or the web/guide jobs locally.

## Decisions

1. **Recursive walk with relative names.** `collect_preset_paths` walks subfolders (std only, sorted, deterministic); output names are the path relative to `--preset-dir` with `/` replaced by `__` (`experimental__bow__pipe.wav`), so names are unique and flat in `--out-dir`. Alternative: mirror the folders under `--out-dir`; rejected because existing callers expect a flat directory.
2. **`needs: [bundle, web, guide]`** for `nightly`. Alternative: fix the comment to say it waits only for bundles; rejected, a broken site or Wiki shouldn't ship a "green" nightly. Cost: the nightly waits for the slowest of the three (the guide job), measured from the first run.
3. **Delete the DAW placeholders** rather than rewrite their descriptions: a script whose only output is a false "PASSED" has no honest description. `add-test-infrastructure` adds the real one. The Internals pages and `tests/AGENTS.md` stop naming `tests/daw/`.
4. **`verify-local.sh` adds the cheap CI steps** (docs with `RUSTDOCFLAGS=-D warnings`, `no_alloc`, `cargo bench --no-run`) and lists what it skips in its header. The validators need downloads and the web/guide jobs need Node, wasm and Playwright; they stay out, by name.

## Risks / Trade-offs

- [`verify-local.sh` gets slower] → the added steps reuse the same target dir; the docs build and bench compile are the cost (minutes, recorded in Measurements).
- [A slow guide job delays the nightly] → acceptable; the nightly is not time-critical.

## Measurements

(filled in during apply: `render_presets` count before/after, `verify-local.sh` run time before/after, first nightly run's timing)
