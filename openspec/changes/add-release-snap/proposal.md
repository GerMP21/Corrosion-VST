# Proposal: add-release-snap

## Why

Releasing a friction note (Bow, Stiff Point Scrape, Corrugated Drag) lets the object spring back from the static deflection the exciter held it in, and that "snap" can be far louder than the held note: +30 dB on the default patch (Plate + Corrugated Drag, −48.1 → −17.6 dB) and +37 dB in the bank (Bow · Taut Cable), measured by `reorganize-factory-presets`. The maintainer keeps the snap as a feature, but today nothing bounds it: a player can't have a bowed pad that simply fades, and the factory levelling only measures held notes, so a preset can pass the quality floor and still jump on every note-off.

## What Changes

- New host parameter **Release Snap** (0–1), shown in the friction envelope (MSEG) panel. It sets how suddenly the exciter's static push is withdrawn at note-off: 1 lets go as today (the push follows MSEG Release), 0 eases the object back so the released note is no louder than the held one.
- At the value that reproduces today's snap, every note renders bit-identical to today. The recommended default is that value (1), so the default patch, the bank and old state don't move; the maintainer may choose a bounded default by ear instead (design, Decision 3).
- Old user presets and saved DAW projects without the parameter load with the value that reproduces today's snap (preset and host migrations), whatever the default ends up being.
- The factory bank gains a released-note bound: the seeder renders the note-off too, and each friction preset's Release Snap is set (curated by hand, experimental within a range) so its tail stays within the bound.
- Help text, clip plan entry, exported guide data with its Spanish entry, user Wiki (EN + ES: MSEG page, Bow / Stiff Point Scrape / Corrugated Drag pages, Troubleshooting "my note jumps when I let go"), Internals pages and ARCHITECTURE.
- Not BREAKING: no existing parameter changes meaning.

## Capabilities

### New Capabilities
- `friction-release`: how a friction note lets go of the object at note-off — the Release Snap control, its bound at 0, its identity with today's release at 1, and the loading of state saved before it existed.

### Modified Capabilities
- `factory-preset-bank`: a new requirement bounds every non-kit preset's released note against its held note.

## Impact

- Code: `src/params.rs` (new parameter), `src/voice/mod.rs` (`VoiceControls`, note-off and the friction force path), the friction exciters' release (`src/dsp/exciters/bow.rs`, `stiff_point.rs`, `corrugated_drag.rs`), `src/gui/editor.rs` + `src/gui/controls.rs` (fader, help), `src/presets/mod.rs` (`migrate_preset_json`), `src/lib.rs` (`filter_state`), `src/presets/factory/` (curated values, experimental range, measurement of the released note, levels), `src/guide/clip_plan.rs`.
- Data: factory presets reseeded (every file gains the field; friction presets may get a value below 1), guide data and clips regenerated, screenshots retaken (new fader in the MSEG panel).
- Ordering: after `bow-stick-slip` (the Bow's static deflection changes with real stick-slip, so its snap must be measured on the final model) and after `decide-drone-loop-step` (it reshapes the friction MSEG in Drone mode). Independent of `env-amount-flat-envelope`, but if both land, Release Snap acts on the force after Env Amount's blend.
