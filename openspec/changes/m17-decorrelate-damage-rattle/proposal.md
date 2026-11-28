# Proposal: m17-decorrelate-damage-rattle

## Why

M17 of the 2026-07-07 review (`docs/archive/FABLE-CODE-REVIEW-2026-07-07.md`, left open in `docs/BACKLOG.md`): Damage adds a rattle, a burst of white noise gated by the note's level, to every voice. Earlier changes made it real white noise with a smooth gate and renamed its misleading variable, but two things were left undone on purpose because they change the sound:
- the rattle is **one mono noise added to both channels**, so a damaged, wide note has a dead-centre buzz glued to it while its modes spread out;
- it is **not filtered**: full-band hiss, where a cracked metal object buzzes mostly up high.

The maintainer now wants it decided by ear.

## What Changes

Decision for the maintainer, from A/B clips (task group 1):
- **A1 (recommended): decorrelate.** The stereo path gives each channel its own noise (two independent streams, each at today's level), so the rattle spreads with the note. The mono path (offline tools, mono renders) is unchanged.
- **A2: decorrelate and high-pass.** As A1, plus a gentle high-pass (around 1 kHz, converted to the sample rate), levelled so the rattle's loudness stays the same; it buzzes rather than hisses.
- **B: keep the mono rattle.** No engine change; the documentation already says it is mono.

Under A1/A2, every preset with Damage above 0 sounds different in its rattle only: wider (and, for A2, brighter). The rattle's per-channel level stays the same; its mono fold-down is 3 dB quieter, because two independent noises sum by power. Damage 0 is bit-identical. Not BREAKING.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `material-transforms`: adds "Damage's rattle spreads with the stereo image" (A1, A2), with a high-pass scenario under A2. Under B the delta becomes a requirement that the rattle is identical on both channels, by design.

## Impact

- Code: `src/voice/mod.rs` (`rattle_noise`, `:1632-1655`; the stereo path at `:1936-1947`; a second `Noise` seeded per note beside `rattle_noise` at `:572`, `:1088`, `:1548`).
- Presets: every preset with Damage > 0 (many in the bank, notably percussion and experimental) — run the bank tests, reseed if levels move, regenerate the guide clips (Damage clips at least).
- Docs: `docs/ARCHITECTURE.md`, Internals `Material-Transformations` and `Voices-and-Polyphony`, Wiki `objects/environmental-exposure.mdx` (Damage) in English and Spanish.
- Related: `merge-mono-stereo-render-paths` (O7) merges `process_sample` and `process_sample_stereo`; whichever lands second keeps the mono path's rattle single and the stereo path's per channel.
