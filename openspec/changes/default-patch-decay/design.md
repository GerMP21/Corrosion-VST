# Design: default-patch-decay

## Context

See proposal.md for why. Current code (`dev`, 2026-11-28):
- The default patch is Hand Strike (`exciter_param(ExciterType::HandStrike…)`, `src/params.rs:2815`) on Pipe (`object_param(0)`, `:2816`), Size 1, Rust 0 (`:2823`), Sludge 0 (`:2947`).
- The plateau was measured by `reorganize-factory-presets` (design, line 207): "Hand Strike on Pipe, held or released, stays at that level for about 3 s; at velocity 25 it decays from the start". It is not the voice's hard clamp: `default_notes_do_not_overdrive_the_voice` (`resonator-level`) keeps every default note under ±1 before the clamp. The candidates for what holds the level are:
  - the per-voice drive governor (`src/voice/mod.rs:666-681`: threshold 0.7, order 8, 200 ms release), which turns the exciter's force down while the ring bound is high and lets it back up as the ring decays, flattening the envelope while the hand is still in contact;
  - Hand Strike re-contacting the surface (since `fix-inert-controls`, soft flesh keeps pressing on landing: `(1 − stiffness/0.425)` of the incoming speed is kept);
  - Pipe's own long, high-Q low modes, whose decay at default Damping is seconds long.
- The guide's clip plan (`src/guide/clip_plan.rs`) renders most rows from the default patch, so whatever is decided changes many clips.

## Goals / Non-Goals

**Goals:**
- A default patch that decays like a struck object.
- No factory preset, user preset or project changes sound.

**Non-Goals:**
- Changing how Pipe, I-Beam or Taut Cable ring in presets (alternative 2 below would; not recommended).
- A new "Init" preset distinct from the default patch.

## Decisions

### 1. Measure first

Task 1 renders the default note's 100 ms RMS envelope at velocity 25, 64, 100 and 127, with the governor disabled (`Voice::governor_enabled`, test-only), with Flesh Stiffness raised, and with Damping 1, to find which of the three candidates holds the plateau. The answer decides which of the options in Decision 2 is even possible; if the plateau is gone on current `dev`, the change closes with a measurement and no code.

### 2. What to change (decide by ear)

- **Recommended: change defaults.** Candidates, rendered as A/B clips: Rust 0.3, Sludge 0.3, Damping (or High Frequency Damping) raised, Flesh Stiffness raised, or Plate as the default object. Pick the smallest change that meets the spec bounds and still sounds like a struck pipe. Defaults live only in `CorrosionParams::default` (and the shared constructors `*_param(value)`), so presets and saved projects, which store every value, don't move.
- **Alternative 2: engine.** If the governor holds the plateau, lower its threshold or shorten its release for struck exciters. This moves every Hit-family preset and kit pad: bank tests, a reseed and a level pass, with the numbers reported before deciding. Under this alternative the spec's second requirement is replaced by "Factory presets stay within their floor and character" (already bound by `factory-preset-bank`) and the default-patch requirement stays.
- **Alternative 3: keep.** The change becomes documentation: the Wiki's Getting started page explains the long ring and points to Rust and Sludge; the spec delta is dropped and the change sets `skip_specs: true`.

### 3. Migrations for a changed default

nih-plug restores each parameter by ID and leaves a parameter absent from the saved state at its current default, and `migrate_preset_json` fills absent preset fields with defaults. For each parameter whose default changes, both paths insert the old default when the field is absent: `migrate_preset_json` (`src/presets/mod.rs`) and `Plugin::filter_state` (`src/lib.rs`, only when the state has parameters, the pattern of `migrate_host_diode_stage`). The legacy corpus (`tests/fixtures/presets/legacy/`) is the regression check that old presets still load and sound the same.

### 4. Tests and references

Tests that use the default patch as a reference for something else (render hashes in `print_48k_exciter_render_hashes`, default-identity tests such as `exciters_play_at_a_common_level`) are checked one by one; any that should keep measuring the old patch sets the old value explicitly, so it keeps measuring what it measured. `default_notes_do_not_overdrive_the_voice` must still pass.

## Risks / Trade-offs

- [A changed default changes what every guide clip demonstrates] → clips regenerated and the walkthroughs re-read against them; the change says so in its close-out.
- [The randomizer or kit Assign start from defaults] → checked; the randomizer's ranges are explicit, so only its base values move, recorded in Measurements.
- [Users who liked the plateau] → it remains a preset away (Rust 0, Sludge 0); the Wiki says how.

## Migration Plan

Additive migrations (Decision 3). Rollback: revert the default and the migrations together.
