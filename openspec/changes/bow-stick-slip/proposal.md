# Proposal: bow-stick-slip

## Why

Corrosion's Bow never sticks. The resonator's surface moves at 0.0002–0.007 under a bow moving at about 0.94 (the banks are normalized to a common strike level, so their velocities are small), so the friction always sits on its sliding branch: the "bow" is a sliding drag with a roughness ripple, not the stick-slip (Helmholtz) motion that makes a bowed string or a bowed cymbal sing. `fix-inert-controls` tried real stick-slip by letting the bow see the surface's speed ×200–×1000; it stuck and slipped, but every one of the 12 bowed chord presets fell into the raucous regime (a subharmonic near 20 Hz, partials up to 600 cents off, −16 to −35 dB), so the maintainer kept the sliding bow and filed this item. Real stick-slip is the one exciter behaviour the plugin's name for it promises and doesn't deliver.

## What Changes

- The Bow solves its contact against the surface it is bowing every sample (the surface's response to the bow's own force, as Hand Strike already does with the resonator's contact prediction), with a friction curve that has a sticking branch and hysteresis, so the contact sticks, slips once per period and sticks again.
- Bow Pressure and Bow Speed are mapped into the playable region of a bowed system at the current strike position (the region between the minimum and maximum bow force for that speed), so the defaults and the factory presets produce pitched Helmholtz motion; the raucous regime stays reachable at the extremes of Bow Pressure, as on a real bow.
- Rosin Grip and Slip Curve keep their roles (static and sliding friction, and the shape of the transition) and keep acting (`control-response`).
- Bow's drive level (`EXCITER_DRIVE_LEVEL[1]`) is re-tuned so bowed notes sit at the common exciter level.
- The bowed presets are re-voiced by ear (22 curated: 12 chords, 5 leads, 3 drones, 2 basses; and the 9 experimental Bow presets regenerated), then reseeded; all pass their folder character.
- User presets and projects that use the Bow will sound different (a sound change that can't be migrated). No parameter changes meaning, so it is not BREAKING for automation.
- User Wiki (EN + ES: Bow page, Gesture Modifiers/MSEG where they mention the bow, Troubleshooting for the raucous regime), help texts, Internals and ARCHITECTURE updated; guide clips regenerated.

## Capabilities

### New Capabilities
- `bow-exciter`: how the Bow drives the object — stick-slip motion once per period, a pitched result free of subharmonics in its playable region, and rate independence.

### Modified Capabilities
- (none: the folder character, control-response and exciter-level requirements already bind the re-voiced presets and the re-tuned drive.)

## Impact

- Code: `src/dsp/exciters/bow.rs` (the model), `src/voice/mod.rs` (Bow dispatch at `:1771` gets the contact prediction; `EXCITER_DRIVE_LEVEL[1]`), `src/dsp/resonators/` only if the prediction needs a velocity term it doesn't expose today.
- Presets: `src/presets/factory/curated.rs` (bowed presets), `experimental.rs` (Bow ranges), reseed and `levels.rs`; the golden mode-bank fixture is unaffected (banks don't change).
- Ordering: after `fix-internals-wiki-findings` (its Bow Roughness ripple fix changes the same file and the bowed sound; voicing once, after both, saves a second re-voice) and after `decide-drone-loop-step` (bowed drones run in Drone mode's forced loop). Before `add-release-snap`, whose snap is measured on the bow's final static deflection.
