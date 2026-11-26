# Design: m17-decorrelate-damage-rattle

## Context

Checked on `dev` (2026-11-26), `src/voice/mod.rs`:
- `rattle_noise(signal_level)` (`:1632-1655`) returns 0 at Damage 0; otherwise white noise from `self.rattle_noise` (a `Noise` seeded per note with `salted(seed, 0)`, `:1088`), rate-levelled with `rate::white_noise`, halved, and scaled by `damage·(level − threshold)·0.08` above a damage-dependent threshold.
- The stereo path (`:1936-1947`) calls it once with the mono level and adds the same sample to both channels, and to the high-pass boost path through `mono + rattle`. The mono path (`:1852-1859`) adds it to the one channel.
- The comment at `:1648-1651` and Internals `Material-Transformations` document the mono rattle.

## Goals / Non-Goals

**Goals:** a rattle that spreads with the note (A1), optionally buzzier (A2), with the same per-channel level; Damage 0 untouched.

**Non-Goals:** the rattle's gate and threshold, its level law, Metal Chain's own Internal Rattle (a different control), and the damage companion modes.

## Decisions

### A1: a second noise stream for the right channel

- Add `rattle_noise_right: Noise` beside `rattle_noise`, preallocated, seeded per note with `salted(seed, 1)` (a new salt, so it is independent of every other stream) and reseeded where the first one is (`:1548`).
- Split the gate from the noise: compute the gate amount once from the mono level, then multiply it by each channel's noise. Left keeps today's stream, so a mono fold-down of the old left channel is unchanged in character.
- The high-pass boost path takes the average of the two rattles (it is a mono path, as today).
- The mono path keeps one stream: offline mono renders and `process_sample` stay bit-identical.

*Alternative:* a decorrelating all-pass or a one-sample delay on one channel. Cheaper, but a short delay of white noise is still strongly correlated after any filtering downstream, and an all-pass colours it. Two streams cost one RNG step per sample on damaged voices only.

### A2: optional high-pass

A one-pole high-pass per channel, about 1 kHz (coefficient from `rate`, exactly its 48 kHz value at 48 kHz), with a fixed make-up gain so the rattle's RMS stays within 1 dB of A1's. Under A2 the spec gains a scenario: "the rattle's energy below 300 Hz is at least 10 dB under its energy above 2 kHz".

### B: keep it

The spec delta becomes "the rattle is the same noise on both channels, by design", with a test pinning left = right in the rattle; the comment and docs already describe it.

## Risks / Trade-offs

- [The mono fold-down of the rattle is 3 dB quieter] → inherent to decorrelation; the rattle is a texture well under the note's level; mention it in the Wiki only if the maintainer hears it.
- [Presets with Damage move] → bank tests; reseed if levels move; the maintainer listens to the percussion folder.
- [`no_alloc`] → the second `Noise` lives in the voice from `new()`.

## Migration Plan

None. Presets keep their values; damaged presets sound wider.

## Measurements

To fill in: L/R correlation and per-channel RMS for A1 and A2; preset level changes; render hashes at Damage 0.
