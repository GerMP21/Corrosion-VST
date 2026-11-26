# Design: m14-quality-reaches-held-notes

## Context

Checked on `dev` (2026-11-26):
- `quality_mode_count_scale` (`src/lib.rs:155-162`) maps Eco/Normal/High/Render to 0.5/1/1.5/2. It is read only at note-on (`src/lib.rs:703`, and `:766` for kit pads) and stored in the voice's controls (`src/voice/mod.rs:274`), which pass it to `rebuild_in_place` (`src/dsp/resonators/core.rs:1170`).
- The generator asks for `round(base × scale)` modes (`core.rs:855-872`). After generation the bank is transformed (Damage adds companion modes), pruned of unsoundable modes, weighted by `brightness_weight(index, full_count)` and normalized to `STRIKE_LEVEL_REF` (`core.rs:1213-1247`).
- The clipper part of Quality switches at once (`post_chain.rs:97-101`); O9 (`smooth-quality-switch-and-stereo-factory`) makes that switch click-free.
- The `modal-bank` spec's "Retuned notes keep every mode they can reach" says a kept mode stays for the whole note.
- Documented today in README:101, `docs/internals/Modal-Resonator.md:188` and the Wiki's `quality.mdx:23`.

## Goals / Non-Goals

**Goals:** a downward Quality switch relieves CPU at once without a click; or, under B, the current behaviour pinned by a test.

**Non-Goals:** making a held note sound exactly like a fresh note at the new quality (its weights and normalization were set for the old bank); adding modes to held notes on an upward switch; kit pads' stored sound (they already follow the live quality at note-on only).

## Decisions

### Which modes go

Option A keeps the first `n_new` modes of the held bank and fades the rest, where `n_new` is the mode count a fresh note of that object, pitch and damage would get at the new quality. That is only right if a lower-quality bank is close to a prefix of a higher one (the low modes, which carry the pitch and most of the energy). Task 1.1 checks it per object from the golden-fixture cases: for each object, compare an Eco bank's frequencies with the first modes of the Render bank. Where it isn't a prefix (Damage companions interleave, Industrial Cog's fixed 12 modes, Pipe capped at 9), drop by highest frequency instead, which the fade makes safe either way. Record per object which rule holds.

### How they go

A per-resonator `active_modes` limit and a gain ramp on the modes between the new and old limit:
- the voice manager sees the live scale change (passed in per block, as the post-chain parameters are) and calls `set_quality_scale` on every sounding voice;
- the resonator ramps the doomed modes' output gain from 1 to 0 over 20 ms (converted with `rate`), then shortens its processed range to `active_modes`;
- no allocation and no reordering: the modes stay where they are, the loop's upper bound shrinks. The per-sample loop gains nothing while no fade runs (the fade is a separate, `#[cold]` path, per CLAUDE.md "Keep cold paths out of line in hot loops").

*Alternatives:* rebuilding the bank at the new quality (resets the modes: the ringing note would vanish or re-strike); crossfading two banks (the new bank isn't excited, same problem, and double CPU during the fade). Rejected.

### Option B

No engine change. Add `held_notes_keep_their_mode_count_on_a_quality_change` and keep the docs; the spec delta becomes "A held note keeps its mode count until it ends; the next notes use the new quality", with the MODIFIED requirement dropped.

## Risks / Trade-offs

- [Dropping modes changes a held note's timbre at the switch] → it's the point of the switch; the 20 ms fade avoids a click, and only downward switches do it.
- [Level drop on trim] → measure: struck notes at C2/C4/C6 per object, RMS before and after a Render→Eco switch; expect under 1 dB (the dropped modes are the high, quiet ones). Recorded, not specced, because normalization isn't redone.
- [Hot-loop cost] → bench `voice_manager_sixteen_voices` with `scripts/bench_ratios.py 5`; must stay within noise; hash a fixed-quality render to prove bit-identity.

## Migration Plan

None. No parameter, preset or project format changes.

## Measurements

To fill in: prefix check per object; level change on a downward switch; largest step; CPU ratio (median of 5); render hashes at fixed quality.
