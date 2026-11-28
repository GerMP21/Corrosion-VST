# Proposal: widen-hrtf-spread

## Why

The post chain's **Spread** control (`spread_width`, 0–1, default 0.5) says "Stereo spread of the image. Higher sounds wider." It does the opposite to the voices' own stereo. The HRTF spread stage rebuilds both channels from a delayed, proximity-filtered mid plus the side scaled by `1 − w/2`, then crossfeeds them by `0.3·w`, so the side's net gain is `(1 − w/2)(1 − 0.6w)`: 0.74 at 0.25, 0.53 at the default, 0.2 at 1. Turning Spread up narrows the image.

Two backlog items describe this one behaviour: the HRTF spread's mono remix, left open by the 2026-07-07 review (under C5), and "Spread Width's help is backwards", found while writing the Internals wiki. They are fixed together here. `fix-internals-wiki-findings` leaves the help text to this change.

## What Changes

Decision for the maintainer, from A/B clips (task group 1):
- **A (recommended): make Spread widen.** Spread 0 leaves the image as it is. Raising it scales the side up, to twice the dry side at 1, with no crossfeed. The delayed, proximity-filtered mid fades in with Spread, levelled so the mono level at low frequencies holds within 1 dB across the range. The control then does what its name and help say. Every preset with Spread above 0 sounds different, and 0.5 is the default, so that is the default patch and nearly the whole bank: wider, same level. Saved projects can't be migrated to their old sound, because the old narrowing isn't reachable with the new control. **BREAKING** for the sound of saved projects and for Spread automation.
- **B: keep the sound, tell the truth.** No engine change. The help text, the Wiki's Spread section (EN + ES), ARCHITECTURE and the Internals page say what Spread does: it focuses the mix, narrowing the voices' own stereo and adding crossfeed and a near-field tone. A test pins the narrowing.

The maintainer chose to keep the spread bit-identical in `m07-m08-interpolate-echo-and-tidy-spread`. That was a tidy-up, not a decision on what Spread should do, but it is a reason to listen before choosing A.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `post-chain-routing`: adds a requirement that "Spread Width widens the image while keeping the mono level". Under B the delta becomes "Spread focuses the image", which pins the narrowing and states it as intended.

## Impact

- Code (A): `src/dsp/post_processing/hrtf_spread.rs` (the remix and its tests), `src/gui/controls.rs:1088` (help, if the wording changes); the `spread_width` parameter's doc in `src/params.rs:720-723`.
- Presets (A): the default patch and every factory preset (214 files; `curated.rs` sets 0.7–0.8 on three, `experimental.rs` draws 0.2–0.9). Run the bank tests, reseed, and listen to the curated folders. Regenerate the guide clips: Spread is on every one.
- Docs: `docs/ARCHITECTURE.md`; Internals `Body-Spread-and-Space`; Wiki `output-chain/dispersion.mdx` (Spread section), `how-it-sounds.mdx` where it mentions spread, and Troubleshooting under A ("my old project sounds wider"), in EN and ES. Also `export_guide_data` and `parameters.es.json` for the help text.
- Order: after `rate-invariant-spread-and-ridge-wrap` (R4) and `smooth-space-and-width` (R3); see design.
