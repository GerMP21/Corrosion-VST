# Design

## Context

- **Why:** see proposal.md.
- **Before this change,** egui 0.31 embedded its default fonts through `epaint_default_fonts`. The editor asked only for `FontFamily::Proportional`, which egui maps to Ubuntu Light, with Noto Emoji and the emoji icon font as fallbacks. Hack was the monospace font.
- **Font sizes** come from `font_size(scale, pct)` in `src/gui/editor.rs`: a share of the editor width plus `FONT_SIZE_BOOST`. They were tuned on Ubuntu Light. Layout tests guard the result: `editor_content_fits_the_window_width`, `header_faders_have_room` and `brand_text_clears_the_preset_bank`.
- **The glyphs egui draws as text** in the widgets the editor uses:
  - `⏵`, on the submenus of the preset browser's folders (`ui.menu_button`);
  - `…`, when it elides text.

  ComboBox, checkbox and scroll arrows are painted, not drawn as glyphs.
- **Where `default_fonts` comes from.** `nih_plug_egui` turns it on twice: through its own default features, and by depending on egui-baseview with egui-baseview's defaults (`default = ["opengl", "default_fonts"]`). Cargo can't switch a transitive default feature off. The web workspace turned it on in `web/editor` (`eframe` features and `egui`'s defaults) and in the root `egui` dependency.

## Goals / Non-Goals

**Goals:**
- Inter for the interface and IBM Plex Sans for the brand block (maintainer, 2026-11-19).
- No egui default font in any build.
- No visible layout change apart from the typeface: same cap height, same row height.

**Non-Goals:**
- Retuning the layout or the font sizes for Inter's look.
- Changing the site's fonts: it uses the system font stack.
- Subsetting the fonts.

## Decisions

### 1. Inter Regular and IBM Plex Sans SemiBold, unmodified, from `assets/fonts/`
- **Inter 4.1 Regular** sets the interface, from the release's static `extras/ttf/`.
- **IBM Plex Sans 1.1.0 SemiBold** sets the brand.

Both are embedded with `include_bytes!` and not subset. Plex's license reserves the name "Plex" for unmodified copies, and there are no font tools in this environment to subset with.

One weight each keeps the size down. SemiBold gives the brand line weight against Inter's Regular, and the subtitles stay legible at the editor's smallest size.

*Alternatives:*
- **Inter Light,** closer to Ubuntu Light's weight. Thinner on dark at small sizes.
- **Plex Regular for the subtitles:** +196 KiB for two short lines.

### 2. `UI_EM_SCALE`: keep the cap height and the row height
- **The factor.** `font_size()` multiplies every size by Ubuntu Light's cap height over Inter's, 0.693 / 0.7275 = 0.9525. Capitals stay as tall as before. Inter's row (ascender + descender, 1.21 em) becomes 1.153 em of the requested size, against Ubuntu Light's 1.149 em (+0.3 %). The ⓘ badge's "i" gets the same factor.
- **Why not `FontTweak::scale`.** It scales the glyphs but not the row height, which egui takes from the unscaled metrics. Rows would grow 5 %, and every panel with them.
- **The brand.** IBM Plex Sans gets a glyph `FontTweak` of 0.693 / (0.698 × 0.9525) = 1.042, so its capitals match too. Its rows are 1.3 em (hhea), taller than before. The brand block's height is set by fixed-height rects and a metal panel, and `brand_text_clears_the_preset_bank` still passes.

### 3. Corrosion Icons, built in code
`⏵` must render and Inter has no glyph for it. The options were:
- (a) keep egui's emoji icon font (MIT, 312 KiB);
- (b) ship another symbol font;
- (c) build a font.

`icon_font()` builds a TrueType file of under 1 KiB:
- `.notdef` plus four filled triangles at U+23F4–U+23F7 (⏴⏵⏶⏷);
- the tables a reader needs: `cmap` format 4, `glyf`, `head`, `hhea`, `hmtx`, `loca`, `maxp` 1.0 and `post` 3.0, with real checksums;
- Inter's em, ascender and descender, so a fallback glyph sits on Inter's baseline.

It is the last fallback of each family. It's Corrosion's own work, so it needs no notice. The triangles are about the size of the old icon font's arrow; they were enlarged once after the first screenshot.

### 4. Taking `default_fonts` out of the builds
- **The patch.** `vendor/egui-baseview` is a copy of the pinned revision `ec70c3f` with one change: `default = ["opengl"]`. It is patched in with `[patch."https://github.com/BillyDM/egui-baseview.git"]`, following the `vendor/nih_log` precedent (`PATCHED.md`).
- **The dependencies.** `nih_plug_egui` is used with `default-features = false, features = ["opengl"]`. The root `egui` and `web/editor`'s `egui` lose their default features, and `eframe` loses `default_fonts`.
- **The guard.** `scripts/third_party_notices.py` gets `FORBIDDEN = {"epaint_default_fonts"}`, so CI's `web` job fails if any change turns the fonts back on.

*Alternative:* vendoring `nih_plug_egui` instead. It would need its `nih_plug` path dependency rewritten to git, which is a bigger diff.

### 5. Installing the fonts
`fonts::install(ctx)` calls `ctx.set_fonts` once:
- in `create_editor`'s build closure, which runs each time the window opens, before the first frame;
- in the web app's eframe creation closure (`corrosion::gui::install_fonts`).

The editor's tests make their contexts with `editor_test_context()`, which does the same. Installing per frame was rejected: `set_fonts` would compare about 600 KiB of font data every frame, and a fresh context's first frame would still lack the fonts.

## Risks / Trade-offs

- **[A new label uses a character Inter lacks]** → `every_drawn_character_has_a_glyph` lists the editor's non-ASCII characters. It has to be extended when a label gains a new symbol. A missing glyph shows as `?`, not as an invisible gap.
- **[NIH-plug moves to another egui-baseview revision]** → The patch then no longer applies: Cargo warns that the patch is unused, and `epaint_default_fonts` would return. The `FORBIDDEN` check fails in CI, and `PATCHED.md` says what to redo.
- **[Inter's look changes the feel]** → The maintainer chose it. Before and after screenshots are below, and the Wiki's screenshots were retaken.

## Measurements

| | Before | After |
|---|---|---|
| Font bytes embedded per build | 1,407,752 B (Ubuntu Light, Noto Emoji, Hack, emoji icons) | 614,272 B (Inter 411,640, Plex SemiBold 202,632) + icon font under 1 KiB |
| Web editor wasm (`corrosion_web_editor_bg.wasm`, release) | 5,016,196 B | 4,239,224 B (−776,972 B, −15.5 %) |
| `epaint_default_fonts` in the dependency tree | Linux, Windows MSVC/GNU, macOS ×2, wasm32 | none (`cargo tree -i`, all six targets) |
| Shipped crates (third-party notices) | 274 | 273 |
| Lockfiles | — | `Cargo.lock` −8 lines, `web/Cargo.lock` −7 lines; no crate added |

- **Tests, red then green:**
  - `the_editor_uses_only_its_own_fonts` and `every_drawn_character_has_a_glyph` fail with Corrosion Icons removed from the interface family ("lacks ⏴⏵⏶⏷").
  - `the_icon_font_draws_the_submenu_arrow` fails the same way: the advance becomes the `?` replacement's, 6.76 against 9.04.
  - The generator fails on a crate added to `FORBIDDEN`.
- **The suites:**
  - GUI lane (`cargo test --lib`): 82 editor tests pass, layout tests included. 40 failed before the tests' contexts got the fonts.
  - CI lane (`cargo test --workspace --no-default-features`): passes.
  - `npm run check -- --complete`: passes.
- **Screenshots.** The Wiki's 10 editor screenshots were retaken with `web/scripts/screenshots.cjs` from a local web build.
  - The header's brand column grew about 11 px at 1280 px, because Plex SemiBold is wider. The preset bank moved right by the same amount.
  - The callouts on the editor and preset-browser images still point at their parts.
- **Not measured:**
  - The native plugin binaries' size. The font bytes predict about −780 KiB, like the wasm.
  - A check in REAPER: REAPER isn't in this container. The web app draws the same editor code with the same fonts.
