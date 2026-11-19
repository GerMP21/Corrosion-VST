# Patched egui-baseview

This is egui-baseview 0.5.0, the egui backend NIH-plug's `nih_plug_egui` uses,
taken from <https://github.com/BillyDM/egui-baseview> at `ec70c3f` (the revision
`nih_plug_egui` pins). The license is in `LICENSE` (MIT).

**The one change:** `default_fonts` is no longer a default feature
(`Cargo.toml`, `[features] default = ["opengl"]`). `nih_plug_egui` depends on
egui-baseview with its default features, so without this every Corrosion
binary embedded egui's default fonts (Ubuntu Light, Noto Emoji, Hack and an
emoji icon font). The editor brings its own fonts instead
(`src/gui/fonts.rs`, `replace-editor-fonts`). The `src/` files are untouched.

The plugin workspace uses this copy through `[patch."https://github.com/BillyDM/egui-baseview.git"]`
in the root `Cargo.toml`, and depends on `nih_plug_egui` without its default
features (`opengl` only). Drop the copy and the patch line when nih_plug_egui
lets a dependent turn the fonts off, or when egui-baseview's default features
leave them out. If NIH-plug moves to another egui-baseview revision, refresh
this copy from that revision and reapply the change.
