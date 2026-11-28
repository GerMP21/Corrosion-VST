# Design

## Context

See proposal.md for why. Today (`dev`, 2026-11-26; the review's line numbers hold within a few lines):

- **Echo.** `FactoryEcho::process_stereo` (`space.rs:401-463`) computes `base_delay = (0.02 + 0.78·delay_time)·sample_rate` every sample straight from the control. Unmodulated it rounds to a whole sample (`:428-430`); modulated it reads with a Hermite interpolator (`:432-443`). `delay_time` comes from `set_parameters` (`:381-391`), which the post chain calls once per block with either the knob or the synced division (`lib.rs:1111-1120`, `sync_rate_to_delay_time`). A change jumps the read head by up to 0.78 s.
- **Factory.** `FactoryReverb::update_delays` (`:107-123`) sets each comb's length from Size (`0.5 + 1.5·size` times the 48 kHz delays `COMB_DELAYS_48K`), and the comb buffer is used as a ring of exactly that length (`index %= delay`, `:113`; `index = (index + 1) % delay`, `:143`). A size change shortens or lengthens the ring at once. The output is also scaled by `size` (`:158`), and `size < 0.001` returns 0 (`:127`). The allpass delays don't depend on Size.
- **Spring.** `SpringReverb::update_delay` (`:230-237`) sets `delay_length = 80 ms · (0.5 + tank_size)`, again a ring of that length (`:236`, `:258`). The output is scaled by `tank_size · 0.7` (`:262`), and `tank_size < 0.001` returns 0 (`:241`).
- All three buffers are already sized for the longest delay at `MAX_SAMPLE_RATE` plus 8 samples (`capacity_for`, `:33-35`), so a ring with a read offset needs no new memory.
- The echo's Machinery Movement smoother is 20 ms (`ECHO_MOVEMENT_SMOOTHING_SECONDS`, `:317`). The review suggested reusing it; at 20 ms a full Delay Time sweep would move the read head about 40 times faster than real time, a chirp rather than a glide, hence the speed cap below.

## Goals / Non-Goals

**Goals:**
- No click from any delay-length control, automated or turned by hand, including tempo-sync division and tempo changes.
- Static settings bit-identical, so presets and the guide's clips don't move.

**Non-Goals:**
- Smoothing Space Amount or the Space mode switch: R1 and R2 in `smooth-space-and-width`.
- The Spring's rate-dependent dispersion constant: R4 in `rate-invariant-spread-and-ridge-wrap`.
- Changing the reverbs' timbre while static.

## Decisions

### 1. One glide law for all three: a 50 ms one-pole, capped at one sample per sample

- Each delay keeps a `current` length (f32, in samples) and a `target` from the control. Per sample: `step = (target − current)·k`, clamped to ±1 sample, `current += step`. `k` comes from 50 ms with `rate::`-style conversion (defined in seconds, so the same at every rate).
- Small changes take about 50 ms; large ones are limited by the cap. A full Delay Time sweep (0.02 → 0.8 s, 37 440 samples at 48 kHz) takes about 0.8 s, with the pitch of the repeats shifted by at most an octave meanwhile: a tape-like glide.
- When `|target − current|` falls under 1e-3 samples, `current = target` and the delay reads whole samples again (the exact path), so a settled delay is bit-identical to today.
- Alternatives considered:
  - Crossfade between the old and new read positions over 20–50 ms: no pitch shift, but it needs two reads per line and a timer per delay, and a fast automation sweep becomes a string of crossfades (comb-filtering). The glide is what tape and analogue delays do and what the review suggested. Task 1.3 renders both for the echo so the maintainer can overrule by ear.
  - A pure one-pole without the cap (the review's suggestion): a large jump moves the read head tens of times faster than real time, which is a chirp.

### 2. Rings with a read offset for the reverbs

- The Factory combs and the Spring keep one write index each over the full buffer, and read at `write − delay`. For a whole-sample delay this computes exactly what the "ring of length delay" does today (both read the value written `delay` samples ago), so static renders stay bit-identical. Task 2.1 proves it with a render hash before any glide exists.
- While gliding, the combs and the Spring read with linear interpolation (two taps). Hermite (four taps) would be smoother, but inside a feedback loop the difference lasts only as long as the glide; the echo keeps its Hermite reader.

### 3. The wet gain follows the glide too

- Factory Size and Tank Size also scale the wet output (`ap_out·size`, `delay_sample·tank_size·0.7`). A size step is therefore also a gain step. The gain uses the glided size (mapped back from the glided delay, or a second one-pole with the same `k`), so the "no click" bound holds.
- The `< 0.001` early returns use the target, as today, so a Size or Tank Size of 0 still silences the processor at once.

### 4. First value and resets

- `new()` and `reset()` mark each glide unprimed; the first `set_parameters` (or `update_delay[s]`) after that sets `current = target`. A sample-rate change re-primes, since the lengths are in samples.
- R1 (`smooth-space-and-width`) clears processors that become inactive on a Space mode switch; a cleared processor is also re-primed, so it doesn't glide from a stale length when it comes back.

### Ordering

Run after `smooth-space-and-width` (it restructures `post_chain.rs`'s Space path and clears inactive processors) and `rate-invariant-spread-and-ridge-wrap` (R4 edits `SpringReverb::process`). Run before `trim-post-chain-per-sample-work`, whose O1 guard on `set_sample_rate` must cover the glides' coefficients.

## Risks / Trade-offs

- [The glide's pitch shift is audible on a long Delay Time sweep] → it's the intended tape character; the crossfade alternative is rendered in task 1.3 for the maintainer to compare.
- [Linear interpolation in the reverbs' feedback loops dulls the tail while gliding] → it lasts only during the glide; the static path is exact.
- [A fast automation (LFO-like) on Size never settles, so the reverb never reads exactly] → acceptable: it's what the user asked for, and the cap keeps it click-free.
- [Bit-identity breaks through a subtle reorder of the comb's read and write] → task 2.1 lands the ring restructure alone, with a render-hash comparison, before the glide.

## Migration Plan

None: no parameter or preset change.

## Measurements

To fill in: the step bound per scenario (worst ratio), the time a full Delay Time and Size sweep takes to settle, the render-hash comparison for static settings, the echo glide-vs-crossfade verdict, and the `post_processing_chain` benchmark median before and after.
