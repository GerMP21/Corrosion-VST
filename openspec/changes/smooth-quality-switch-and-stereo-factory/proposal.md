# Proposal: smooth-quality-switch-and-stereo-factory

## Why

Two Optional findings of the 2026-11-14 review (`FABLE-CODE-REVIEW-2026-11-14.md`) sit at the end of the post chain and are both about what the stereo output does when nothing "should" change:
- **O9:** changing Quality while audio plays clears the diode stage's half-band filters (`OversampledClipper::set_oversample_factor` → `clear_filters`), so the switch gives a short discontinuity.
- **O10:** the Factory reverb is mono: the chain sums the input to mono and returns `(wet, wet)`, while Spring and Echo keep left and right apart. A big hall that collapses to the centre is the odd one out.

O9 is a click users can trigger; O10 is a sound choice that needs the maintainer's ear first.

## What Changes

- **O9 (recommended: crossfade):** a Quality change moves the diode stage to its new oversampling factor with a short crossfade (2–5 ms) instead of clearing its filters mid-signal, so switching Eco/Normal/High/Render while a note rings doesn't click. Fallback, if the measured step is below the dry signal's own: no code change, one line in the Quality tooltip and the Wiki's Quality page.
- **O10 (decide by ear; recommended: stereo room):** the Factory reverb gives a stereo tail, Freeverb style: the right channel runs its own combs and all-passes with delays offset by a few mutually prime samples, both fed by the same mono sum. Alternative: keep the mono room and document it as a design choice.
- Both are gated: the first task group measures O9's discontinuity and renders A/B clips of O10 for the maintainer; implementation follows their choice.
- Factory-space presets change sound under O10 (wider, same level within the bank tests' bounds). Not BREAKING: no parameter or format changes.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `output-clipper`: adds a requirement that a Quality change doesn't click (O9, crossfade option).
- `space-effects`: adds a requirement that the Factory room's tail is stereo while its level and decay stay as they are (O10, stereo option). Under the "keep mono" choice this delta becomes a requirement that the Factory tail is identical on both channels, stated as intended.

## Impact

- Code: `src/dsp/post_processing/oversampled_clipper.rs` (O9), `src/dsp/post_processing/space.rs` and `post_chain.rs` (`process_space`, O10).
- Memory: O10 doubles the Factory room's buffers (about +0.25 MB per instance, sized for 192 kHz in `new()`); CPU of Factory mode roughly doubles for the reverb part, measured.
- Presets: every preset with Space = Factory and Amount > 0 sounds wider; run the bank tests, reseed if levels move, regenerate the guide clips.
- Docs: `docs/ARCHITECTURE.md`; Internals `Output-Protection`, `Body-Spread-and-Space`, `Post-Chain`; user Wiki (EN + ES) `quality.mdx` and `output-chain/space.mdx`; the Quality help text if the tooltip option is chosen (`export_guide_data`, `parameters.es.json`).
- Order: after `smooth-space-and-width`, which rewrites `process_space` for the mode-switch crossfade (R1); after `m14-quality-reaches-held-notes` is decided, since both act on a Quality change (they can share the switch detection).
