# Proposal: default-patch-decay

## Why

The first sound a new instance makes, Hand Strike on Pipe at the default controls, rings at a held, loud plateau for about 3 seconds before it starts to fade (about 0.31 RMS at the voice, measured by `reorganize-factory-presets`, held or released; Pipe, I-Beam and Taut Cable strikes do the same, even at Damping 1). It isn't self-oscillation (it decays exponentially once below the plateau), but it is loud and static, so the default patch reads as a drone rather than a struck pipe, and every guide clip that starts from the default patch inherits that plateau. The percussion presets avoid it with heavy Rust and Sludge. The maintainer left it to decide by ear.

## What Changes

Recommended option (decided by ear first; design, Decision 2):
- Change one or two **default values** (for example Rust or Sludge, or the default object) so the default note decays from its strike instead of holding a plateau. Factory presets and saved state store their own values, so they don't move.
- State saved without a parameter whose default changes (old preset files, old projects) loads with the previous default, through both the preset loader and the host's state restore, so it keeps its sound.
- Every guide clip that renders from the default patch is regenerated, the guide data is re-exported (the reference lists defaults), and the user Wiki's pages that describe the default patch (Getting started, the sound-design walkthroughs that start from it, Troubleshooting) are updated in English and Spanish.

Alternatives: an engine change so long-ringing objects don't hold the plateau (moves the whole bank and needs a reseed), or keep it and document it.

Whichever is chosen, task 1 first re-measures the plateau on current `dev` and finds what holds it (the voice's drive governor, repeated hand contacts, or Pipe's own decay), since the claim predates `fix-inert-controls`' Hand Strike change.

## Capabilities

### New Capabilities
- `default-patch`: what the patch a new instance starts with sounds like — its note decays from the strike — and how state saved under older defaults keeps its sound.

### Modified Capabilities
- (none)

## Impact

- Code (recommended option): the default values in `src/params.rs` (`CorrosionParams::default`, `:2815-2947`; object Pipe at `:2816`, Rust `:2823`, Sludge `:2947`), the migrations in `src/presets/mod.rs` (`migrate_preset_json`) and `src/lib.rs` (`Plugin::filter_state`), and the randomizer if it starts from defaults.
- Guide: `src/guide/clip_plan.rs` renders from the default patch, so the clips regenerate (`scripts/build-guide-clips.sh`); `export_guide_data`; screenshots retaken if the editor shows the changed default (the knob positions move).
- Tests that assume today's default patch (render hashes, `default_notes_do_not_overdrive_the_voice`, the default-patch identity tests of other features) are re-checked; a test that pinned the old default as a reference is changed to set the old value explicitly.
- Ordering: independent; if `bow-stick-slip` or `env-amount-flat-envelope` regenerate the clips in the same period, regenerate once after all of them.
