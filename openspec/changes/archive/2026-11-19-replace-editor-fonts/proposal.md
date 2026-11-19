# Proposal

## Why

The editor drew with egui's default fonts, which egui embeds in every build:
- Ubuntu Light, under the Ubuntu Font Licence, is uncommon and a poor fit for a GPLv3 binary;
- Noto Emoji, which Corrosion never shows;
- Hack;
- an emoji icon font.

Together they come to 1.38 MB per plugin binary and web app. `add-third-party-notices` surfaced them. The maintainer chose (2026-11-19) to replace them with Inter for the interface and IBM Plex Sans for the header's brand block, and to take egui's fonts out of the builds entirely.

## What Changes

- **Inter 4.1 Regular** (SIL OFL 1.1) sets the whole interface: the plugin's editor and the web app.
- **IBM Plex Sans 1.1.0 SemiBold** (SIL OFL 1.1) sets the brand block: "CORROSION", "INDUSTRIAL RESONATOR" and "MODEL: CR-89X".
- **Corrosion Icons**, a TrueType font built in code, holds the four triangles egui draws as text and Inter lacks. One of them is the `⏵` on the preset browser's folder submenus.
- **Same text metrics.** Every editor font size is scaled so Inter's capitals are as tall as Ubuntu Light's were, and rows keep their height (within 0.3 %). Panels don't grow, and the layout's tuned sizes still hold.
- **egui's default fonts leave every build.** A vendored copy of egui-baseview drops `default_fonts` from its default features: `nih_plug_egui` turns it on through egui-baseview, and a dependent can't switch it off. `nih_plug_egui`, `egui` and `eframe` are used without it.
- **The third-party notices** list the two fonts and the vendored egui-baseview. The generator fails if `epaint_default_fonts` ever ships again. OFL-1.1 and Ubuntu-font-1.0 leave the crate allowlist.
- **The Wiki's editor screenshots** are retaken. No Wiki text describes the fonts, so no page changes.

## Capabilities

### New Capabilities
- `editor-fonts`: which fonts the editor draws with, that every character it draws has a glyph, that sizes keep the layout's metrics, and that no other font ships in the builds.

### Modified Capabilities
(none)

## Impact

- **New:**
  - `src/gui/fonts.rs`;
  - `assets/fonts/`: the two TTFs, their licenses and a README;
  - `vendor/egui-baseview/`: a copy, plus `PATCHED.md`.
- **Changed:**
  - `src/gui/editor.rs`: font sizes, the brand family, font installation, the tests' contexts;
  - `src/gui/mod.rs`;
  - `web/editor/src/lib.rs`;
  - `Cargo.toml`, `web/editor/Cargo.toml` and both lockfiles (only removals);
  - `scripts/third_party_notices.py`, `THIRD-PARTY-NOTICES.md`;
  - `docs/ARCHITECTURE.md`, `CLAUDE.md`;
  - the Wiki's screenshots.
- **Size:** the web editor's wasm shrinks by 759 KiB, and the plugin binaries by about the same amount.
- **Not affected:** sound, parameters, presets, saved projects, the host's parameter names.
