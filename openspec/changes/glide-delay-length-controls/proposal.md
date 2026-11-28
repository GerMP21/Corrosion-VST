# Proposal

## Why

Three Space controls set a delay length, and moving any of them makes the read head jump: **Echo Delay Time** (and the tempo-synced division that replaces it), **Factory Size** and **Spring Tank Size**. Automating them, turning the knob while a tail rings, or a tempo change under a synced echo clicks (review finding O5, `FABLE-CODE-REVIEW-2026-11-14.md`; `docs/BACKLOG.md`). The echo already reads between samples for its modulation, so gliding its delay is cheap; the two reverbs need the same idea.

## What Changes

- Each of the three delay lengths follows its control through a glide instead of jumping: a one-pole smoother (50 ms) whose speed is capped at one sample per sample, so the read head never runs faster than twice or slower than zero times real time (a pitch shift of at most an octave while it moves), like a tape machine's varispeed.
- While a delay glides, its line is read between samples: the echo's existing Hermite interpolator; linear interpolation for the Factory combs and the Spring tank.
- Once a glide settles, the delay snaps to its exact whole-sample length, so static settings sound exactly as today (bit-identical renders).
- The first value after load or reset is taken at once (no glide from a default), as the BODY smoother does.
- The Factory combs and the Spring tank move from "buffer length = delay" to a fixed ring with a read offset, which is what makes a fractional, moving delay possible. With a static delay the two forms compute the same samples.
- Users hear it: a sweep of Delay Time or Size now sounds like a tape or tank being stretched instead of clicking. The Wiki's Space page says so, in English and Spanish.
- Not **BREAKING**: ranges and meanings are unchanged.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `space-effects`: adds that the echo delay, Factory Size and Spring Tank Size glide without clicks, settle to their exact delay, and leave static settings unchanged.

## Impact

- Code: `src/dsp/post_processing/space.rs`: `FactoryReverb` (`update_delays` `:107-123`, `process` `:126-160`), `SpringReverb` (`update_delay` `:230-237`, `process` `:240-265`), `FactoryEcho` (`process_stereo` `:401-463`). No change in `post_chain.rs` or `lib.rs` beyond passing the same parameters.
- Real-time safety: no new buffers; the rings are the existing ones, sized for `MAX_SAMPLE_RATE`.
- Docs: `docs/ARCHITECTURE.md` (Space section), `docs/internals/Body-Spread-and-Space.md`.
- Wiki: `web/guide/src/content/docs/output-chain/space.mdx` and `es/output-chain/space.mdx` (Factory Size, Tank Size, Delay Time, Sync); guide clips are static renders and don't change.
- Presets: none move (static renders are bit-identical); bank tests must still pass.
- Review: O5 marked FIXED, removed from `docs/BACKLOG.md`.
