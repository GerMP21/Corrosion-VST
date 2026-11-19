# Tasks

## 1. Fonts and their tests

- [x] 1.1 Add Inter 4.1 Regular and IBM Plex Sans 1.1.0 SemiBold, their licenses and a README to `assets/fonts/`; verify the files match the releases (sha256 in the commit) and the README names version, source and license
- [x] 1.2 Write `src/gui/fonts.rs`: definitions (proportional and monospace: Inter then Corrosion Icons; `brand`: Plex then Inter then icons), `UI_EM_SCALE`, Plex's glyph tweak, `install`, and `icon_font()`; verify the five `gui::fonts` tests pass and three of them fail with the icons removed from the interface family
- [x] 1.3 Use the fonts in the editor: `font_size()` and the badge's "i" times `UI_EM_SCALE`, the brand block in the `brand` family, `install` in `create_editor`'s build closure and in the web app's creation closure, `editor_test_context()` for the tests; verify the GUI lane's 82 editor tests pass

## 2. Take egui's fonts out of the builds

- [x] 2.1 Vendor egui-baseview `ec70c3f` with `default = ["opengl"]` and `PATCHED.md`, patch it in, and turn off the default fonts on `nih_plug_egui`, the root `egui` and `web/editor`'s `egui` and `eframe`; verify `cargo tree -i epaint_default_fonts` finds nothing on all six shipped targets
- [x] 2.2 Add `FORBIDDEN` to `scripts/third_party_notices.py`, drop OFL-1.1 and Ubuntu-font-1.0 from `ALLOWED`, add the fonts and egui-baseview to the notices' section 1, and regenerate; verify `--check` passes and a forbidden crate fails it

## 3. Screenshots, docs and close-out

- [x] 3.1 Rebuild the web app, retake the Wiki's screenshots and check the callouts; record sizes and the before and after look in design.md `## Measurements`
- [x] 3.2 Document the fonts in `docs/ARCHITECTURE.md` (module map, the "Editor fonts" section, the ⓘ note) and `CLAUDE.md` ("Third-party notices")
- [x] 3.3 Close out: `cargo fmt --check` (both workspaces), CI-lane clippy with `-D warnings`, web clippy (native and wasm), no new GUI-lane clippy warnings, the CI and GUI test lanes, `no_alloc`, `cargo doc`, the guide check and `openspec validate replace-editor-fonts`; update "Current state" in `CLAUDE.md`, then commit and push to `dev`

## Workflow follow-up

- The maintainer checks the editor in a DAW: the brand block, the preset browser's submenu arrows, and text at 50 % and 150 % UI scale.
- Archive with `/opsx:archive` once checked.
