# Proposal

## Why

Preset files and saved projects are untrusted input: a user downloads them, hand-edits them, or a disk corrupts them. The parameter loader clamps every value, but three paths skip it (`FABLE-CODE-REVIEW-2026-11-14.md`):
- **R7.** A kit pad's `level_db` goes straight into `10^(dB/20)`: `"level_db": 800` builds a pad with an infinite gain. Only the downstream clamps stand between that and the output.
- **N9.** Building a pad from its stored values ends in `.expect("pad values parse as a preset")`. A value JSON can carry but the preset parser rejects (for example `1e40`, which becomes infinity in `f32` and then `null`) panics the plugin in `initialize` or on the GUI thread.
- **N11.** Import, and the browser's scan of the user folder, read a whole file into memory before checking it. Picking a multi-gigabyte file by mistake stalls the editor.

## What Changes

- A pad's level is clamped to −60…+72 dB when a kit is read (preset file, project state, web app), and when a pad is built. The factory pads use +0.1…+56.7 dB, so the bank is unchanged.
- Building a pad's parameters returns an error instead of panicking. A kit with a bad pad loads every other pad and leaves that key empty; the plugin logs one line naming the key. The editor's "load pad" and Assign paths skip such a pad the same way.
- Preset files larger than 4 MB are refused before they are read, by Import (counted as skipped, like any file that isn't a preset) and by the user-folder scan (counted as skipped). The largest factory file is 50 KB; a full 128-pad kit is about 0.5 MB.
- Not BREAKING: every valid preset and project loads as before.

## Capabilities

### New Capabilities

(none)

### Modified Capabilities
- `kit-mode`: adds that pad levels from files are bounded and that a pad that can't be built leaves only its own key empty.
- `user-presets`: adds that oversized files are refused without being read.

## Impact

- Code: `src/kit/mod.rs` (`KitPad` level clamp on deserialize and in `build`; `params()` and `build()` return `Result`/`Option`; `KitTable::load` skips failures with a log line), `src/gui/editor.rs` (`load_pad` and the pad build call sites), `src/presets/mod.rs` (`Preset::load` and `import_preset_files` check `metadata().len()` first; a `MAX_PRESET_FILE_BYTES` constant), the web app's kit path if it builds pads (`src/web/`).
- Tests: `src/kit/tests.rs`, `src/presets/mod.rs` tests.
- Docs: Internals `Kit-Mode.md`, `Presets-and-Migrations.md` (and any other page that names `KitPad::params` or Import's read), `docs/ARCHITECTURE.md`; user Wiki Troubleshooting (EN + ES) gains a line for "an imported file was skipped" if the size limit can surprise.
- Review: R7, N9 and N11 marked FIXED; removed from `docs/BACKLOG.md`. The review's Security section lists these three as the exceptions to "untrusted input is clamped"; that line is updated.
