# Design

## Context

See proposal.md for why. Checked on `dev` (2026-11-26):
- `src/kit/mod.rs:95-104`: `KitPad { key, tune, level_db, values }`, `level_db` defaults to 0 and is deserialized as is. `from_params` (`:138-151`) clamps `tune` to ±`MAX_TUNE` but stores `level_db` unchanged; `build` (`:191-199`) clamps tune again and computes `gain: 10f32.powf(self.level_db / 20.0)`.
- `src/kit/mod.rs:156-188`: `params()` builds a JSON preset from the defaults, writes each stored value (`serde_json::Value::from(value as f64)`; a non-finite `f32` becomes `null`), then `Preset::from_json_str(..).expect("pad values parse as a preset")`. A `null` in a float field fails the typed parse, so the `expect` panics.
- Callers of `params()`/`build()`: `KitTable::load` (`src/kit/mod.rs:340-347`, from `initialize` and state restore), the editor's `load_pad` (`src/gui/editor.rs:4081-4082`) and the pad build at `src/gui/editor.rs:4059`.
- `src/presets/mod.rs:770-773`: `Preset::load` does `fs::read_to_string(path)?`; it serves the user-folder scan (`:929`). `import_preset_files` (`:1114-1137`) reads each picked file whole.
- Sizes: the largest factory file is 50 330 bytes (`drums/junk_percussion_kit`); `PAD_LEVELS` spans +0.1…+56.7 dB.

## Goals / Non-Goals

**Goals:** no file can produce an infinite pad gain, a panic, or an unbounded read.

**Non-Goals:**
- A level control for pads in the editor.
- Bounding host project state (the host hands it over in memory).
- The seqlock's formal data race (F1), which the review left as FYI.

## Decisions

1. **Clamp `level_db` at the boundaries.** A `deserialize_with` on `level_db` clamps to `PAD_LEVEL_DB_RANGE = −60.0..=72.0` (non-finite → the nearer bound, NaN → 0); `from_params` and `build` clamp too, so a pad made in code is bounded the same way. Clamping on deserialize means a saved-again file holds the clamped value.
   - Range: −60 dB is effectively silent, +72 dB leaves 15 dB above the loudest factory pad.
   - *Alternative:* clamp only in `build`. Fixes the gain but keeps an absurd value in the file and the editor; rejected.
2. **`params()` returns `Result<CorrosionParams, KitPadError>`; `build()` returns `Option<Pad>`.** `KitTable::load` writes `None` for a key whose pad fails and logs it once with `nih_log` (key and reason), on the thread that already loads (never the audio thread). The editor's paths treat a failure like an empty key. The non-finite value is also caught early: `params()` skips a stored value that isn't finite, so the pad loads with that one parameter at its default; only a value the parser still rejects fails the whole pad.
   - *Alternative:* skip only the bad value and never fail the pad. Covers the known case, but other parser rejections (an enum out of its type) would still need a path; the `Result` is the general guard and the skip keeps most pads playable.
3. **`MAX_PRESET_FILE_BYTES = 4 * 1024 * 1024`.** `Preset::load` and `import_preset_files` check `fs::metadata(path)?.len()` first and return an `io::ErrorKind::InvalidData` error ("preset file too large") without opening the file for reading. Both callers already count an error as skipped.
   - 1 MB (the review's example) is closer to a full kit (≈ 0.5 MB) than comfortable; 4 MB leaves room and still bounds the read.
   - A race between `metadata` and the read (a file growing in between) is not a concern for a GUI-thread file picker; reading through `take(limit + 1)` would close it at no cost, so do that too.

## Risks / Trade-offs

- [A legitimate user kit somewhere outside −60…+72 dB] → none can be made in the editor; only a hand-edited file can, and it loads at the bound.
- [Changing `params()`'s signature touches tests] → a handful of `.unwrap()`s in tests.

## Migration Plan

None; not BREAKING. Valid files load identically.

## Measurements

(Filled in during implementation: the `level_db: 800` gain before/after, the panic reproduced before and gone after, the factory kits' pad gains unchanged.)
