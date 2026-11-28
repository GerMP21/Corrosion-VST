# Proposal

## Why

The post-chain drive's tube curve is non-monotonic: it reaches 1.0 at an input of 1.5 and then falls, to 0.85 at 3 and 0.71 at the ±5 clamp. Hot peaks fold back down (louder in, quieter out). The fold adds upper harmonics, and the drive runs at the host rate without oversampling, so they alias. Nothing in the code or the docs says whether the fold is intended (review finding R8, `FABLE-CODE-REVIEW-2026-11-14.md`; `docs/BACKLOG.md`). It is the kind of decision the maintainer takes by ear, so this change renders the options first and implements only the chosen one.

## What Changes

- Render A/B clips of the two options on presets and test signals that drive the curve past 1.5, for the maintainer to choose.
- **Recommended (B): a monotonic curve.** Above the 0.5 knee the curve becomes `0.5 + 0.5·tanh(2·(|x| − 0.5))`: it stays within 0.028 of today's curve up to an input of 1.5 (largest gap at 1.25; 0.881 vs 0.900 at 1, 0.982 vs 1.000 at 1.5) and keeps a slope of 1 at the knee; above 1.5 it approaches 1 instead of folding (1.000 vs 0.845 at 3, vs 0.712 at 5). Louder in is never quieter out.
- **Alternative (A): keep the foldback** and document it as a feature: the code, `docs/ARCHITECTURE.md`, the Internals page and the Wiki's Overdrive page say that hot inputs fold back and why.
- Either way the behaviour is pinned by a test, and the review finding is closed.
- Under B, presets with a high Drive Amount (or a strong Violence macro) change sound above an input of 1.5: bank tests, a reseed, and the guide clips that render the drive are regenerated.
- Not **BREAKING**: no parameter changes meaning or range; only the curve's shape above the knee.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `post-chain-drive`: adds a requirement on the shape of the saturation curve above its knee (monotonic under B; a documented foldback under A).

## Impact

- Code: `src/dsp/post_processing/lorenz_drive.rs` (`tube_saturate`, `:193-201`) and its tests.
- Presets: under B, `src/presets/factory/levels.rs` and the preset files after `seed_presets`; bank tests.
- Wiki: `web/guide/src/content/docs/output-chain/overdrive.mdx` and `es/output-chain/overdrive.mdx`; guide clips for Drive Amount (and any other clip whose render reaches the fold) regenerated under B.
- Docs: `docs/ARCHITECTURE.md` (drive section), `docs/internals/Filter-and-Drive.md` (the paragraph on `tube_saturate`, which today says it is undocumented whether the fold is intended).
- Review: R8 marked FIXED in `FABLE-CODE-REVIEW-2026-11-14.md`, removed from `docs/BACKLOG.md`.
