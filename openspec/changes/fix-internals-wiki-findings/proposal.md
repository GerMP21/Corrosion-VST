# Proposal

## Why

Writing the Internals wiki (`add-internals-wiki`, 2026-11-19) meant reading every module against its docs, and the writers found bugs and stale text the 2026-11-14 review didn't list (`docs/BACKLOG.md`, "Found while writing the Internals wiki"). Each is small, but together they make controls lie (Wire Stiffness, Pos Wander, Drop Envelope Spread), make the Bow sound different at every sample rate, leave the web app's diode dot dark, replay stale audio, lose an empty kit, and leave about 30 comments and docs describing code that no longer exists. The maintainer asked for them as one change; it is grouped so each item can be applied and verified on its own.

## What Changes

Each item is its own task group. Items marked *(decide)* start with a gating task for the maintainer; the recommended option is what the specs describe.

1. **Bow Roughness ripple** *(decide, sound change)*: the ripple's phase advances by the bow speed per 48 kHz sample, so it sits near 45 kHz and folds back to 2959 Hz at 48 kHz, 942 Hz at 44.1 kHz, 45 kHz at 96 kHz. Recommended: a ripple in hertz proportional to the bow speed, `K · v` with `K` chosen so the default patch keeps today's 48 kHz pitch, below Nyquist at every rate. Bowed presets may move; bank tests and reseed.
2. **Brush Wire Stiffness** *(decide, sound change)*: the brush's "high-pass" uses the low-pass coefficient `dt/(RC+dt)`, so stiffness is a ~29 dB level control with one shape. Recommended: a real high-pass (`RC/(RC+dt)`) with level compensation, so higher stiffness sounds brighter at roughly the same level, as the help says. Wire Brush presets move; bank tests and reseed.
3. **Position Wander's dead top half** *(decide)*: the host range is 0–1 but the bus clamps to 0–0.5. Recommended: shrink the host range to 0–0.5. Stored values above 0.5 already acted as 0.5, so every saved preset and project sounds the same (no migration needed); **BREAKING for Position Wander automation** (the normalised range maps to half the old values).
4. **Web app's diode dot**: `WebEngine::process` never publishes the clipper's gain reduction, so the dot never moves. The meter is published inside the shared block path so the plugin and the web app both feed it.
5. **Lookahead limiter replays stale samples** on a switch from Hard: its delay line is cleared on the switch and crossfaded from the Hard path over its delay, so no old audio and no 1 ms dropout.
6. **An empty kit saved in Kit mode loads as Tonal** *(decide)*: recommended, a preset saved in Kit mode always writes its `kit` field (empty when there are no pads), so it round-trips as Kit; presets from before `per-key-kits` still load as Tonal.
7. **Drop Envelope Spread's dead 40–50 ms**: the exciter clamps to 50–1000 ms under a 40–400 ms host range; the clamp becomes 40–1000 ms. No factory preset stores a value under 50.
8. **Stale comments and docs** (~30): rebuilt into a list with file and line (the backlog says the archived design lists them; it doesn't, it only says "stale comments and docs"), then each corrected or removed. Includes the Brightness macro's "±2 octaves" (it is ±1), Global Time's description, the `PlayMode` doc still describing 10-note kit slots, the module docs of `src/gui/mod.rs` and `src/dsp/utils/mod.rs`, `basic.rs`, and the filter's file name `wdf_filter.rs` for a TPT ladder. Excluded here because other changes own them: the stale comments in `tidy-review-nits` (N3, N4, N7) and Spread Width's backwards help (`widen-hrtf-spread`).

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `sample-rate-independence`: the Bow's roughness ripple has the same pitch at every rate, below Nyquist.
- `exciter-controls`: Brush Wire Stiffness changes tone, not just level; Drop Envelope Spread acts over its whole range.
- `strike-position`: Position Wander's whole range acts.
- `diode-stage`: the web app's dot follows the diode stage too.
- `output-limiting`: switching Limiter Mode never replays old audio.
- `kit-mode`: a preset saved in Kit mode with no pads loads back in Kit mode.

## Impact

- Code: `src/dsp/exciters/bow.rs`, `src/dsp/exciters/wire_brush.rs`, `src/dsp/exciters/metal_chain.rs`, `src/params.rs` (Position Wander range, docs), `src/lib.rs` (meter publish, limiter switch, macro docs), `src/web/mod.rs`, `src/presets/mod.rs`, comments across `src/`, `docs/ARCHITECTURE.md`, `README.md`, `CLAUDE.md`; possibly a file rename (`wdf_filter.rs`).
- Presets: Bow and Wire Brush presets may move (bank tests, reseed); Position Wander and Drop Envelope Spread move none (the bank's largest Position Wander is 0.40, and no preset stores a Drop Envelope Spread under 50 ms).
- Host automation: **BREAKING** for Position Wander (item 3, recommended option).
- Docs: user Wiki (EN + ES) for items 1–3, 5–7 where they describe the control; Internals pages Friction Exciters, Hit Exciters, Strike-position/Interaction, Output-Protection, Web-App, Kit-Mode, Presets-and-Migrations, and every page the stale-docs list touches; guide data and clips for changed parameters and sounds.
- Order with other changes: item 1 before `bow-stick-slip` (it re-voices the bowed presets once, on the final ripple); item 8's filter rename after `trim-post-chain-per-sample-work` (O3 edits the same file); `split-largest-source-files` moves `lib.rs`/`voice/mod.rs` test modules, so rebase whichever lands second.
