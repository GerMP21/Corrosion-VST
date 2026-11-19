# Editor fonts

The plugin's editor and the web app embed these fonts (`src/gui/fonts.rs`,
`replace-editor-fonts`). They replace egui's default fonts, which the build
leaves out (`vendor/egui-baseview/PATCHED.md`).

| File | Font | Source | License |
|---|---|---|---|
| `Inter-Regular.ttf` | Inter 4.1, Regular: the whole interface | `extras/ttf/` of `Inter-4.1.zip`, <https://github.com/rsms/inter/releases/tag/v4.1> | SIL OFL 1.1, `Inter-LICENSE.txt` |
| `IBMPlexSans-SemiBold.ttf` | IBM Plex Sans 1.1.0, SemiBold: the header's brand block | `fonts/complete/ttf/` of `ibm-plex-sans.zip`, <https://github.com/IBM/plex/releases/tag/%40ibm%2Fplex-sans%401.1.0> | SIL OFL 1.1 with the Reserved Font Name "Plex", `IBMPlexSans-LICENSE.txt` |

Both files are unmodified. Don't subset or edit IBM Plex Sans: a modified
version may not keep the name "Plex". To change a font or weight, replace the
file, update this table, the notes in `src/gui/fonts.rs` (the cap heights
behind `UI_EM_SCALE`) and section 1 of `THIRD-PARTY-NOTICES.md`.

The editor's third font, Corrosion Icons, holds the triangles egui draws on
submenus. It is built in code by `icon_font()` in `src/gui/fonts.rs`, so it
has no file here.
