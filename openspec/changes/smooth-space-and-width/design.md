# Design

## Context

See proposal.md for why. Checked on `dev` (2026-11-26):
- `src/dsp/post_processing/post_chain.rs:225-237`: with Space Mode Off the dry signal goes straight on; otherwise `process_space` (`:243-257`) runs **only** the selected processor (Factory on the mono sum, Spring per channel, Echo stereo) and the chain adds `wet × space_amount`. Nothing clears or fades the others.
- `post_chain.rs:155-162`: `set_space_mode` and `set_space_amount` store the value as is. They are called once per block from `src/lib.rs:1084` and `:1095`.
- BODY already has the pattern to copy: `body_amount_target`, `body_amount_primed` and a 20 ms one-pole (`BODY_SMOOTHING_TAU_SECONDS`, `smoothing_coeff`, `post_chain.rs:65-69, 141-147, 212-217`).
- `src/dsp/post_processing/hrtf_spread.rs:60-62`: `if spread_width < 0.001 { return (left, right) }`. Above it the mid is delayed by up to 1 ms with Width and mixed through the proximity low-pass/high-pass, whose low-frequency gain is `0.5 + 0.3·proximity`.
- Buffer sizes (`space.rs`): every Space buffer is sized for 192 kHz in `new()`. The Echo holds two buffers of `capacity_for(38 400 · 1.1)` ≈ 169 k samples each; the Factory reverb's combs and allpasses and the Spring's line are smaller.

## Goals / Non-Goals

**Goals:**
- No click and no replayed tail on a Space Mode change; Space Amount glides; Spread Width continuous from 0.
- Bit-identical output when no control moves (static Space Amount, Width ≥ 0.05).

**Non-Goals:**
- What Spread Width does to the stereo image above 0.05 (`widen-hrtf-spread`).
- Delay-length glides for Echo Delay Time, Factory Size and Spring Tank Size (`glide-delay-length-controls`).
- A stereo Factory reverb (`smooth-quality-switch-and-stereo-factory`).
- Smoothing Spread Width itself; it is set per block like before. A per-sample Width smoother can follow if a sweep still zippers.

## Decisions

1. **Mode change: 20 ms equal-gain crossfade, then clear.** On a mode change the chain keeps the previous mode as `fading_out` with a gain ramping 1 → 0 over 20 ms (rate-converted), while the new mode's wet ramps 0 → 1. Both processors run during the fade. When the fade ends, the outgoing processor is cleared.
   - *Alternative:* clear the inactive processors at once (the review's minimum). Fixes the replay but keeps the click on a ringing tail; rejected.
   - *Alternative:* keep every processor running all the time. Fixes both but triples Space CPU forever; rejected.
   - Off counts as a mode with a wet of 0, so Factory → Off fades Factory out, and Off → Spring fades Spring in.
   - A new change during a fade: the processor already fading out is cleared at once (it is under its fade gain anyway), and the current one becomes the outgoing one. Bounded: at most two processors ever run.
2. **Clearing without a CPU spike.** Clearing the Echo writes about 338 k floats. Doing it in one sample costs a few hundred microseconds inside one block; acceptable once per user gesture, but it lands in a single block. The clear is spread instead: a processor marked dirty is cleared in slices of 256 samples per processed frame until done (the Echo is clean after about 1 320 frames, 28 ms at 48 kHz), and selecting a dirty processor finishes its clear before it runs. All writes are `fill(0.0)` on preallocated buffers, so nothing allocates.
   - Measure both (one-shot vs sliced) with a block-time benchmark; keep the one-shot clear if its worst block stays under 5 % of a 512-sample block at 48 kHz (*As built* note either way).
3. **Space Amount smoother:** `space_amount_target`, `space_amount_primed`, the same `smoothing_coeff` and snap-to-target below 1e-6 as BODY. Static amounts are bit-identical because the smoothed value equals the target from the first sample after priming.
4. **Spread Width fade:** the delay line and both one-poles run every sample at every Width. The processed output is computed as today; the stage returns `dry + (processed − dry) · fade`, with `fade = smoothstep(width / 0.05)` clamped to 1. At `width ≥ 0.05` the stage returns `processed` directly (branch on `fade == 1.0`), so the output is bit-identical there.
   - *Alternative:* keep the bypass and only reset state on re-entry. Leaves the 3.7 dB jump; rejected.
   - 0.05 is the review's suggestion; it is 5 % of the knob's travel and no factory preset sits below it.
5. **Ordering with `widen-hrtf-spread`.** That change rewrites how the processed path builds the image. Landing this one first gives it a stage whose state is always current and a fade it can keep; if it lands first instead, this change applies the same fade around its new processed path.

## Risks / Trade-offs

- [Two Space processors for 20 ms] → a short CPU bump on a mode change only; measured in Measurements.
- [Running the spread's filters at Width 0] → about 10 multiply-adds per sample; within benchmark noise.
- [A preset that automates Space Mode rhythmically] → now crossfades instead of cutting; that's the intended fix.

## Migration Plan

None. No parameter or state changes; not BREAKING.

## Measurements

(Filled in during implementation: the R1 replay test before/after, the R3 Width table before/after, worst block time on a mode change (one-shot vs sliced clear), bank-test results, render hashes of the bank with static controls before/after.)
