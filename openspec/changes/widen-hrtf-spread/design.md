# Design: widen-hrtf-spread

## Context

`HrtfSpread::process` (`src/dsp/post_processing/hrtf_spread.rs:58-112`, checked on `dev` 2026-11-26), with `w` = Spread Width and `p` = Listener Proximity:
- below `w = 0.001` it returns the input (`:59-61`);
- `mid = (L+R)/2` is delayed by `w` ms (`:67-80`, the same delay for both channels, not an interaural time difference);
- the delayed mid goes through a one-pole low-pass (`0.8/0.2`) and a high-pass built from it (`0.5/0.5`), mixed as `lpf·(0.5 + 0.3p) + hpf·(0.3 + 0.2p)` (`:88-101`);
- the side is added at `(1 − w/2)` (`:99`, `:103`), then crossfeed `c = 0.3w` mixes the channels (`:106-108`); the net side gain is `(1 − w/2)(1 − 2c)` = `(1 − w/2)(1 − 0.6w)`;
- the output is clamped to ±1.5 (`:110`).

The help (`src/gui/controls.rs:1088`) says "Higher sounds wider"; the Wiki's `dispersion.mdx:42` says the same; Internals `Body-Spread-and-Space.md:75` already states the narrowing.

Two other changes touch this function:
- `rate-invariant-spread-and-ridge-wrap` (R4) converts the proximity one-poles' raw per-sample constants with `rate`, bit-identical at 48 kHz;
- `smooth-space-and-width` (R3) removes the jump at `w = 0` (−3.7 dB at 100 Hz between 0 and 0.001).

## Goals / Non-Goals

**Goals:** Spread does what it says (A) or says what it does (B); no level jump anywhere in its range; the voices' header WIDTH control is untouched.

**Non-Goals:** a real HRTF; changing Listener Proximity's meaning beyond levelling the mid at the default proximity; the header WIDTH (per-voice) control.

## Decisions

### Order: R4, then R3, then this change

1. **R4 first.** It is a pure rate fix, bit-identical at 48 kHz. Landing it first means this change's tests run at every rate from the start, and A's rewrite keeps the rate-converted coefficients rather than re-introducing raw ones.
2. **R3 next.** It is a Required fix with its own small test (continuity at 0), and it lands whatever the maintainer decides here. Under B it is the only engine change the spread gets.
3. **This change last.** It is decided by ear and may rewrite the function. Its tests must keep R3's continuity test and R4's rate tests passing. Under A, R3's crossfade near 0 may become unnecessary, because A's processing is the identity at 0 by construction. If so, A removes it with R3's test still green.

### A: a mid/side widener with the proximity mid faded in

- `side_out = side·(1 + w)`: 1 at 0 (dry), 1.5 at the default, 2 (+6 dB) at 1. No crossfeed, since crossfeed is what narrowed it.
- The mid becomes `mid_out = (1 − w)·mid + w·prox(delayed mid)`. `prox` is the proximity filter mix, scaled so its gain is 1 at low frequencies at `p = 0.5`: today's mix gives 0.65 there, which is the R3 drop. So Listener Proximity still tilts the tone around the default, and Spread doesn't change the level. The 1 ms mid delay stays, scaled by `w`, keeping the stage's "near-field" colour.
- `w = 0` is then exactly the identity. The bypass stays as a cheap path. The ±1.5 safety clamp stays.
- The ranges are unchanged (0–1, default 0.5). Hosts and presets keep their values; the sound changes.

*Alternatives:*
- Keep the old curve below 0.5 and widen above it. That keeps the default's sound, but the control would narrow, then widen.
- Add a new parameter and freeze the old one for old projects. That is two Spread controls, and the old one still narrows.

Both were rejected as worse for users than one honest control.

### B: document

- Help text: "Focuses the mix: higher narrows the voices' own stereo and adds crossfeed and a near-field tone."
- The Wiki's Spread section and ARCHITECTURE say the same.
- The spec delta becomes "Spread focuses the image", with a scenario that the side-to-mid ratio falls as Spread rises.

### Presets under A

The seeder re-levels every preset. Character checks may move: the chords' flatness and the drones' width. Run `curated_measurements_table` before and after, and have the maintainer listen to the curated folders. If a curated preset relied on the old focus, lower its Spread rather than re-voicing it.

## Risks / Trade-offs

- [A changes almost every sound in the bank and in saved projects] → the maintainer listens first (task 1.1). It is marked BREAKING. The Wiki's Troubleshooting says how to get the old focus back: lower Spread, or use the header WIDTH.
- [A wider default patch may sound worse on mono playback] → the mid is kept, so the mono fold-down keeps its level; the side cancels in mono, as any widener's does.
- [Clamping at ±1.5 bites more with +6 dB side] → measure peak levels across the bank; the diode stage after the spread bounds the output anyway.

## Migration Plan

Under A: no preset or project migration is possible (the old narrowing isn't reachable). Presets are reseeded, the change is called BREAKING in the commit and the Wiki, and the Troubleshooting entry explains it. Under B: nothing.

## Measurements

To fill in: side-to-mid ratio and mono level per Spread Width before and after; bank levels and character tables before and after; peak levels at Spread 1; render hashes at Spread 0.
