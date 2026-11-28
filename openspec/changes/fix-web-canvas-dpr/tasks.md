# Tasks

## 1. Reproduce

- [ ] 1.1 Add a `#[wasm_bindgen]` getter for the editor's content height in CSS px (`web/editor/src/lib.rs`) and verify `cargo clippy -p corrosion-web-editor --target wasm32-unknown-unknown -- -D warnings` passes in `web/`
- [ ] 1.2 Write `web/scripts/dpr.cjs`: open `/play/` at `deviceScaleFactor` 1, 1.5 and 2 in a 1600 × 1000 viewport, compare the canvas's CSS height with the getter and its drawing buffer with CSS size × ratio, exit non-zero naming the ratio and both numbers; verify it fails on current code at 1.5 and 2 (record the numbers in design.md `## Measurements`)

## 2. Fix

- [ ] 2.1 Find the factor that carries the device pixel ratio (log `pixels_per_point`, `zoom_factor`, `devicePixelRatio`, `screen_rect` at each ratio), note it in design.md as an *As built* line, and size the canvas in CSS px as in design decision 2; verify `dpr.cjs` passes at all three ratios and `smoke.cjs` still passes
- [ ] 2.2 Check by hand in Chromium that Ctrl + / Ctrl − (which changes the ratio at run time) keeps the whole editor visible; record the result

## 3. CI and docs

- [ ] 3.1 Run `dpr.cjs` in CI's `web` job after `smoke.cjs`; verify the job passes on the pushed branch
- [ ] 3.2 Update `docs/internals/Web-App.md` (canvas sizing, the new script; `web/scripts/` items stay named), `docs/ARCHITECTURE.md`'s web app section, and the comment in `web/scripts/screenshots.cjs:37-42`; verify `cargo test --no-default-features --test internals_wiki` passes
- [ ] 3.3 Check whether the Wiki (`web/guide/src/content/docs/`, EN and ES) mentions a cut-off editor or HiDPI; update it if so (none found when this change was written); verify `npm run check -- --complete` in `web/guide/`

## 4. Close-out

- [ ] 4.1 `cargo fmt --check`, clippy (workspace and the web crates), `cargo test --workspace --no-default-features`, `web/scripts/build.sh` all pass
- [ ] 4.2 Remove the web canvas DPR item from `docs/BACKLOG.md`, update CLAUDE.md "Current state", commit and push to `dev`

## Workflow follow-up

- Archive with `/opsx:archive`; the site deploy check runs only on `main`.
