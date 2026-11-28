# Tasks

## 1. render_presets

- [ ] 1.1 Add a test (in `src/bin/render_presets.rs` or `tests/`) that the default preset collection finds all 214 factory presets with unique output names; verify it fails on current code (0 found)
- [ ] 1.2 Walk subfolders and name outputs by relative path; verify 1.1 passes and `cargo run --release --bin render_presets -- --out-dir /tmp/rp` writes 214 WAVs (record the count in design.md)
- [ ] 1.3 Update the documented command in `docs/ARCHITECTURE.md`, `CLAUDE.md` and `docs/internals/Factory-Bank.md` / `Testing.md`; verify `internals_wiki` passes

## 2. nightly

- [ ] 2.1 Set `nightly`'s `needs` to `[bundle, web, guide]` in `.github/workflows/ci.yml` and keep its comment true; verify the file parses (`python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"`, or `actionlint` if installed) and, after the push to `dev`, that the workflow graph in the run shows the new edges
- [ ] 2.2 Update `docs/internals/CI-and-Releases.md` and `docs/ARCHITECTURE.md`'s CI section; verify `internals_wiki` passes

## 3. DAW placeholders and tests/AGENTS.md

- [ ] 3.1 Delete `tests/daw/gate-2-smoke.sh` and `tests/daw/run-reaper.sh`; remove `tests/daw` from every Internals page (`grep -rl 'tests/daw' docs/internals/`) and from `docs/ARCHITECTURE.md`/`README.md`; verify `internals_wiki` passes (no dangling path)
- [ ] 3.2 Rewrite `tests/AGENTS.md` without `.sisyphus/`, pointing at `tests/fixtures/` and `docs/REAPER_TESTING.md`; verify `grep -rn sisyphus tests docs` finds nothing outside archives

## 4. verify-local.sh

- [ ] 4.1 Add the docs (warnings denied), `no_alloc` and benchmark-compile steps and a header listing what isn't covered; introduce a `cargo doc` warning in a scratch edit and verify the script fails at the docs step, then revert
- [ ] 4.2 Run the script end to end and record its time before and after in design.md; update `docs/internals/Testing.md` and `CLAUDE.md`'s Build and test table; verify `internals_wiki` passes

## 5. Close-out

- [ ] 5.1 `cargo fmt --check`, clippy `-D warnings`, `cargo test --workspace --no-default-features` pass
- [ ] 5.2 Remove the three items from `docs/BACKLOG.md`, update CLAUDE.md "Current state", commit and push to `dev`

## Workflow follow-up

- The `nightly` edge is only exercised on `main`: check the first run after the next merge.
- Archive with `/opsx:archive`.
