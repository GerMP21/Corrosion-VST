# Design

## Context

See proposal.md for why. The specialty ADSR lives as plain fields on `Voice` (`adsr_phase`, `adsr_level`, `adsr_attack`, `adsr_decay`, `adsr_sustain`, `adsr_release`) and advances once per sample in `Voice::process_force_envelope`.

Checked on `dev` (2026-11-26); the review's line numbers have moved by a few lines:
- `src/voice/mod.rs:940-945`: the `Sustain` arm re-checks `!self.active` and moves to Release. `note_off` (`src/voice/mod.rs:1476-1479`) sets `active = false` and calls `release_force_envelope` (`:963-979`), which already moves Attack, Decay and Sustain to Release. The only other writer of `active = false` is `reset` (`:1537`), which also sets the phase to Idle. So the branch is unreachable (N6).
- `src/voice/mod.rs:946-952`: `release_rate = adsr_sustain / adsr_release.max(dt)`; at Sustain 0 the rate is 0 and the level never falls (C1).
- The voice stops rendering only when the ADSR is Idle (`src/voice/mod.rs:1739-1744`), so a stalled Release keeps the voice alive.

The friction MSEG already solved this: `MSEG::release` stores `release_level` (`src/dsp/envelopes/mod.rs:236-240`) and Release ramps from it.

## Goals / Non-Goals

**Goals:**
- Every released specialty note reaches silence in Env Release from its note-off level.
- Notes released in Sustain stay bit-identical.

**Non-Goals:**
- Curves for the ADSR (it stays linear).
- Gesture Modifiers on the ADSR (`scope-gesture-modifiers`).
- Changing the Hit AR or the MSEG.

## Decisions

1. **Store the level at note-off.** `release_force_envelope` copies `adsr_level` into a new `adsr_release_from: f32` when it moves the ADSR to Release. The Release arm uses `release_rate = adsr_release_from / adsr_release.max(dt)`. A note released in Sustain has `adsr_level == adsr_sustain`, so the rate is the same `f32` value as today and the output is bit-identical.
   - *Alternative:* store a precomputed rate. Same cost, but keeping the level mirrors the MSEG's `release_level` and is easier to read in a debugger.
   - *Alternative:* compute the slope from `max(level, sustain)`. Still stalls when both are 0 (released at the very start of Attack); rejected.
2. **The level at note-off is the Sustain level in the Sustain phase.** In Sustain `adsr_level` is already `adsr_sustain` (Decay clamps to it), so `adsr_release_from = adsr_level` covers all three phases with one line.
3. **Release from level 0 ends at once.** If the level at note-off is 0 (released on the first sample, or Sustain 0 after the Decay ended), the Release arm goes Idle on its first sample. That's what happens today in the second case; the first case used to stall when Sustain was 0.
4. **Remove the `!self.active` check in the Sustain arm (N6)** in the same edit; the Sustain arm just returns `adsr_sustain`. Proven unreachable above; a test that releases a note in Sustain and checks the phase is Release on the next sample guards it.
5. **Spec as a new capability, `specialty-envelope`.** No existing spec covers the ADSR (`mseg-envelope` is the friction MSEG). `scope-gesture-modifiers` and `env-amount-flat-envelope` may add requirements here later.

## Risks / Trade-offs

- [Presets whose notes are released during Decay above a low Sustain now release faster] → the bank tests and `curated_measurements_table` show it; only `percussion/chain_snap` has Sustain 0 among the 44 top-level specialty presets, and it is a Tension Snap (one snap per note). Record the before/after table in Measurements.
- [Tension Snap's single snap may already be over at note-off] → then nothing changes for it; the measurement says so.
- [Drum-kit pads on specialty exciters] → `PAD_LEVELS` are levelled on the held pad; the bank's kit checks run in CI.

## Migration Plan

None: no parameter changes meaning, so presets and projects load as before; only the release of some notes changes. Not BREAKING.

## Measurements

(Filled in during implementation: the review's stuck-note table before/after, `curated_measurements_table` diff for the specialty presets, render hashes of notes released in Sustain before/after.)
