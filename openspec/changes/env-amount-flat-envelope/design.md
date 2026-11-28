# Design: env-amount-flat-envelope

## Context

See proposal.md for why. Current code (`dev`, 2026-11-26):
- `MSEG::velocity_scaled` (`src/dsp/envelopes/mod.rs:323-329`) returns `(level · gain).clamp(0, 1) · env_amount`, with `gain = (1 − vtl) + vtl · velocity_scale`. Env Amount is a pure output gain.
- `MSEG::release` (`:236-243`) stores `release_level = current_level` (the stage level, before gain and Env Amount); Release ramps `current_level` from it to `l_end` (0).
- The voice sets Env Amount once per note (`src/voice/mod.rs:876`) and the friction exciters' force is the envelope's output times their drive (`process_force_envelope`, `:956-959`; the excitation is skipped when the envelope is ≤ 0, `:1767`). So at Env Amount 0 the friction voice is silent from its first sample.
- The help text already says so: "Scales the friction envelope's force. Lower is quieter, and 0 silences the friction exciters." (`src/gui/controls.rs:1073`).
- Bank: all 22 curated friction presets sit at Env Amount 1. The experimental generator draws Env Amount in about 0.5–1: 45 friction presets are below 1 (0.511–0.969), none at 0.

## Goals / Non-Goals

**Goals:**
- Env Amount 0 means "no envelope shape", not "no sound".
- Env Amount 1 is bit-identical, so the default and every curated preset don't move.

**Non-Goals:**
- Making Env Amount act on the AD or ADSR envelopes (that is `scope-gesture-modifiers`' question).
- Changing Pressure's role as the level control.

## Decisions

### 1. Keep or blend (decide by ear)

- **Recommended: blend.** The envelope's shaped level becomes `a · level + (1 − a) · g`, where `g` is a flat gate: it rises with the Onset stage (so a note doesn't start with a step; progress of Onset from 0 to 1, or 1 at once if Onset is 0) and is 1 from then on while the key is held. At note-off `release_level` stores the shaped value, and Release ramps from it to 0 as today, so the release rules (`Release starts from the current level`, `Release ends without a step`) still hold. Velocity gain is applied after the blend.
- **Alternative: keep.** The spec delta is replaced by one ADDED requirement, "Env Amount scales the friction force, and 0 silences it" (scenario: a Bow note at Env Amount 0 renders all zeros), the change sets nothing else, and the backlog item closes as decided.

Task 1 renders A/B clips (Env Amount 1, 0.5, 0 on Bow · Plate and Corrugated Drag · Pipe, and two experimental presets at about 0.55) under both laws for the maintainer; implementation waits on the choice.

### 2. Bit-identity at 1

At `a = 1`, `1.0 · level + 0.0 · g` equals `level` exactly in IEEE arithmetic, and the final `· env_amount` disappears (the blend replaces it), so the output is bit-identical. Proved two ways: a test-only switch on `MSEG` comparing old and new laws sample by sample across the loop-mode sweep, and `print_48k_exciter_render_hashes` against a clean worktree for the five friction exciters.

### 3. Where it lives

Inside `MSEG`, not the voice: the envelope owns its stages and knows when Onset ends and the gate closes. `set_env_amount` keeps its signature; only `process_sample`, `release` and `velocity_scaled` change. No new state beyond what's needed for `g` during Onset (it reads the existing stage and progress).

### 4. Presets

The 45 experimental friction presets below 1 change level and shape. The seeder re-levels them; any that leaves the floor or its character is redrawn by the generator as it already does, and the list of what moved goes in Measurements. The generator's Env Amount range may widen to 0–1 now that 0 is playable (maintainer's call, recorded in *As built*).

## Risks / Trade-offs

- [A flat push at low Env Amount sounds louder than before for the same value] → the seeder re-levels the factory presets; user presets below 1 get louder; said in the Wiki and the proposal.
- [A friction exciter driven flat from the first sample can click] → `g` follows the Onset stage; a 0-length Onset is the user's choice, as with the envelope itself.
- [The loop modes interact with `g`] → `g` stays 1 through looping stages; covered by the exhaustive loop sweep, which must stay finite and in [0, 1].

## Migration Plan

No parameter changes range or default; no migration. User state below Env Amount 1 sounds different (called out). Rollback is reverting the change and reseeding.
