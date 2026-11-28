# Proposal: env-amount-flat-envelope

## Why

Env Amount multiplies the friction envelope's output (`mseg-envelope`: output = level · velocity gain · env_amount), and the friction exciters' force is that envelope, so at Env Amount 0 Bow, Stiff Point Scrape, Heavy Grinding, Corrugated Drag and Tension Rise make no sound at all. A control called "amount" on an envelope normally sets how much the envelope shapes the sound, with 0 meaning a flat, steady push, not silence. Today it duplicates Pressure as a level control and leaves no way to play a friction exciter without its envelope's shape. Found by `print_velocity_sweep` (`fix-pneumatic-jet-saturation`); the tooltip was corrected to say what it does (`write-user-guide-content`), and the decision was left to the maintainer.

## What Changes

Recommended option (the maintainer decides by ear first; design, Decision 1):
- Env Amount becomes an envelope depth: the friction force blends from a flat gate (full while the key is held, released with the envelope's release) at 0 to the full envelope shape at 1. Env Amount 1 (the default and every curated preset) renders bit-identical.
- The 45 experimental friction presets with Env Amount between 0.51 and 0.97 change: their envelope gets shallower and louder in its quiet stages. The seeder re-levels them; all must pass the floor and their character.
- Old user presets and projects keep their values; their sound changes where Env Amount is below 1 (a sound change, not migratable without keeping the old law). Not BREAKING for automation: the parameter keeps its range and default; its meaning below 1 changes, which is called out.
- Help text (EN, and ES in the guide data), the Gesture Modifiers and envelopes pages of the user Wiki (EN + ES), Troubleshooting (the "friction exciter is silent" entry goes), Internals and ARCHITECTURE.

Alternative (keep): no sound change; the change closes the backlog item with the behaviour documented as intended (the help and Wiki already say it), and a test pins "Env Amount 0 silences friction" so it can't drift.

## Capabilities

### New Capabilities
- (none)

### Modified Capabilities
- `mseg-envelope`: "Velocity scales level multiplicatively" and "Zero stage level is silence" change: Env Amount blends between a flat gate and the envelope instead of scaling the output, so a zero stage level is silence only at Env Amount 1.

## Impact

- Code: `src/dsp/envelopes/mod.rs` (`MSEG::process_sample`, `:259-328`; `set_env_amount`, `:246`), help text in `src/gui/controls.rs:1073`; no change to `src/voice/mod.rs` beyond the existing `set_env_amount` call (`:876`).
- Presets: 45 experimental files reseeded; curated presets unchanged (all at 1).
- Guide: the Env Amount clip row and any clip of an experimental friction preset regenerate.
- Ordering: independent of the other changes; if `add-release-snap` lands too, Release Snap acts on the blended force. `decide-drone-loop-step` changes the looped friction envelope; the blend applies to whatever the envelope produces, so either order works (bank reseed once per change).
- Relation to `scope-gesture-modifiers`: Env Amount is one of the Gesture Modifiers; that change only decides where the panel shows, so the two don't conflict.
