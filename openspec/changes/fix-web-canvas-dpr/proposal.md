# Proposal

## Why

On any screen with a device pixel ratio above 1 (HiDPI laptops, most phones, a zoomed browser), the web app's editor is cut off and blurry: at a ratio of 1.5 the 1280 px editor's canvas is 939 CSS px tall instead of 1408, at 2 it is 705 px, so the bottom panels and the keyboard disappear (`docs/BACKLOG.md`; the guide's screenshot script already works around it, `web/scripts/screenshots.cjs:37-42`). That is most of the people who open `/play/` on a laptop.

## What Changes

- The editor sizes its canvas from the editor's content height in CSS pixels at every device pixel ratio, so the whole editor shows; the canvas's backing store follows the ratio, so text and lines are sharp.
- A browser check renders `/play/` at device pixel ratios 1, 1.5 and 2 and fails if the canvas is shorter than the editor's content or if the backing store doesn't match the ratio. It runs in CI's `web` job (Chromium is already installed there for `smoke.cjs`).
- The screenshot script's workaround (a 3200 px window at ratio 1) stays valid; the comment that blames the bug is updated.
- No change to the plugin, the sound, presets or parameters.

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `web-app`: a new requirement that the whole editor shows, sharp, at device pixel ratios 1, 1.5 and 2.

## Impact

- Code: `web/editor/src/lib.rs` (canvas sizing in `CorrosionApp::update`), possibly `web/www/index.html` (canvas CSS) and `src/gui/editor.rs` (`WebEditor::frame` / `content_height`, only if the root cause is in the layout scale).
- Tests: a new Playwright check (`web/scripts/` + CI `web` job).
- Docs: `docs/internals/Web-App.md` (or whichever Internals page names `web/editor/src/lib.rs`), `docs/ARCHITECTURE.md` web section, the Wiki's Troubleshooting page if it mentions the cut-off (EN + ES), the comment in `web/scripts/screenshots.cjs`.
