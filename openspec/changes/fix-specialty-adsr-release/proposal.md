# Proposal

## Why

The specialty exciters' ADSR (Pneumatic Jet, Electromagnetic Hum, Tension Snap, Particle Rain) computes its release slope from the **Sustain level**, not from the level the note had when the key came up. With Env Sustain at 0 (a legal value) a note released during Attack or Decay never falls: the exciter keeps blowing, humming or raining forever and the voice only ends when it is stolen or a panic arrives. This is the one Critical finding (C1) of `FABLE-CODE-REVIEW-2026-11-14.md`: a stuck note anyone can reach with plain parameter values. The same rule also makes Release last `level ÷ sustain` times longer than set when a note is released above a low Sustain.

## What Changes

- The specialty ADSR's Release ramps from the level at note-off to 0 over Env Release, whatever the Sustain level, as the friction MSEG already does. Env Release now means "time to silence from wherever the note was".
- A note released in Sustain releases exactly as today (bit-identical): the level at note-off is the Sustain level, so the slope is unchanged.
- Notes released during Attack or Decay above Sustain release faster than today (in Release, not `Release · level / Sustain`); notes released during Attack below Sustain release slower (in Release, not faster).
- The unreachable `Sustain` branch that re-checks `!self.active` (N6) is removed: `note_off` already moves the ADSR to Release.
- Wiki (EN + ES): the ADSR section says Release is the time to silence from the level at note-off; Troubleshooting gains "a note keeps sounding after I let go" only if anything is left worth saying (the trap is gone).
- Not BREAKING: no parameter changes meaning for saved state; the release of affected notes changes.

## Capabilities

### New Capabilities
- `specialty-envelope`: the ADSR force envelope of the specialty exciters: its stages, how Release starts and ends, and that every released note ends.

### Modified Capabilities

(none)

## Impact

- Code: `src/voice/mod.rs` (`process_force_envelope`'s Specialty arm, `release_force_envelope`, a new `adsr_release_from` field on `Voice`, reset in `configure_envelope` and `reset`).
- Presets: 44 top-level factory presets use a specialty exciter (13–16). One, `percussion/chain_snap` (Tension Snap), has Sustain 0; Tension Snap makes one snap per note, so its audible change is expected to be small, but it is measured. Held notes of the others are unchanged; only notes released during Attack or Decay change. Drum-kit pads on those exciters are covered by the bank tests. Reseed if the seeder's levels move.
- Docs: `docs/ARCHITECTURE.md`, Internals `Envelopes-and-Modulation.md` (its "Trap: Sustain 0 plus an early note-off" paragraph goes), user Wiki `exciters/envelopes.mdx` and Troubleshooting (EN + ES).
- Review: C1 and N6 marked FIXED; their items removed from `docs/BACKLOG.md`.
