# Design

## Context

- `CorrosionApp::update` (`web/editor/src/lib.rs:101-109`) sets the canvas's CSS height to `ceil(editor.content_height() · ctx.zoom_factor())` and leaves the width to CSS (`#editor { width: 80% }`, `web/www/index.html:36`). eframe 0.31 (`glow`, `web/editor/Cargo.toml:16-17`) sizes the drawing buffer from the canvas's CSS size and `devicePixelRatio`, and sets egui's `pixels_per_point`.
- `WebEditor::frame` (`src/gui/editor.rs:684-708`) scales the layout to the page with `page_scale(screen_rect().width())` (`:959`) and records `content_height` in points.
- The measured symptom (backlog, and `web/scripts/screenshots.cjs:37-42`): the CSS height comes out as the correct height divided by the device pixel ratio (1408 → 939 at 1.5, 704 at 2). So one of the factors in that product is in physical pixels, or `zoom_factor()` already divides by the ratio in this eframe version. Not yet pinned to a line: "found while capturing screenshots", not reproduced in a test.
- CI's `web` job already builds the app, serves it and runs `web/scripts/smoke.cjs` in Chromium (`.github/workflows/ci.yml:194-208`).

## Goals / Non-Goals

**Goals:** the whole editor at ratios 1, 1.5 and 2, a sharp drawing buffer, and a CI check that would have caught it.

**Non-Goals:** changing the editor's layout or its 80 %-width rule; the plugin's own (baseview) editor, which has no such sizing code; a mobile-specific layout.

## Decisions

1. **Reproduce first, then fix at the cause.** A Playwright script opens `/play/` at `deviceScaleFactor` 1, 1.5 and 2 in the same CSS viewport, waits for `window.corrosionEditorReady`, and reads the canvas's CSS height and its `width`/`height` attributes plus `devicePixelRatio`. Expected values come from the editor itself: expose the editor's content height in CSS px to the page (a `#[wasm_bindgen]` getter next to `wants_wheel`) so the check compares against the editor's own number, not a constant.
2. **Compute the CSS height in CSS pixels explicitly.** Whatever the root cause turns out to be, the fix converts through egui's `pixels_per_point` and the page's `devicePixelRatio` read once per frame (`web_sys::window().device_pixel_ratio()`): `css = points · pixels_per_point / devicePixelRatio`. This holds whether eframe's zoom factor includes the ratio or not, and stays right if the browser zoom changes the ratio at run time.
3. **Check in CI by extending the existing job,** as a new `web/scripts/dpr.cjs` run after `smoke.cjs` (same server, same Chromium), rather than a separate job: it adds seconds, not a new install.
4. **Leave `screenshots.cjs` at ratio 1** (its 3200 px window already gives sharp 2560 px shots); only correct its comment. Switching it to ratio 2 would change every committed screenshot for no gain.

## Risks / Trade-offs

- [SwiftShader in CI renders differently from a GPU] → the check reads sizes, not pixels.
- [eframe re-reads the canvas size every frame and could fight the explicit height] → the check runs for several frames and after a resize; a failing case shows as a height that oscillates, which the script reports.
- [Browser zoom changes `devicePixelRatio` while the page is open] → covered by decision 2 (read per frame); one manual check with Ctrl + / Ctrl − in Chromium.

## Measurements

(filled in during apply: canvas CSS height and drawing-buffer size at ratios 1, 1.5, 2, before and after)
