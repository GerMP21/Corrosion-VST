# Design: smooth-quality-switch-and-stereo-factory

## Context

Checked on `dev` (2026-11-26):
- **O9.** `PostProcessingChain::set_quality_mode` (`post_chain.rs:100`) calls `clipper.set_oversample_factor(mode.clipper_oversample_factor())` (factors 1/4/8/16, `post_chain.rs:27-30`). `set_oversample_factor` (`oversampled_clipper.rs:261-268`) clears every half-band state and the DC blocker (`clear_filters`, `:314-320`) when the number of stages changes. The parameters reach the chain once per block, so the switch lands on a block boundary.
- **O10.** `process_space` (`post_chain.rs:243-256`) feeds `FactoryReverb::process((left + right) * 0.5)` and returns `(wet, wet)`. `FactoryReverb` (`space.rs:53-170`) is four parallel combs (`COMB_DELAYS_48K`, 32–40 ms, mutually prime) into two all-passes, buffers sized for 192 kHz in `new()`. Spring is per channel, Echo stereo.
- The space-effects spec already asks that Spring give a mono tail from mono input; nothing pins the Factory tail as mono.

## Goals / Non-Goals

**Goals:** no click on a Quality change; a Factory room that is as wide as the other two, if the maintainer wants it, without changing its level or decay.

**Non-Goals:** Space mode switching (R1, `smooth-space-and-width`); Factory Size and the other delay-length zippers (O5, `glide-delay-length-controls`); held notes' mode counts on a Quality change (M14, `m14-quality-reaches-held-notes`).

## Decisions

### O9: crossfade between the old and new oversampling

Cascaded 2× stages keep their rates when a stage is added or removed: stage 0 always runs host→2×, stage 1 2×→4×, and so on. So the states of the stages both factors share are still valid; only an added stage starts cold, and a removed stage simply stops. Clearing everything throws away good state.

Plan:
1. **Measure first** (task 1.1): the largest output step at each switch, against the signal's own largest step. If no switch exceeds the signal's step (the cold stage's transient is tiny), take the fallback: keep the code, and say "switching Quality while a note rings can give a tiny click" in the Quality help and the Wiki.
2. **Otherwise, crossfade:** keep a second, preallocated set of half-band states. On a change, run the old configuration and the new one side by side for 3 ms (144 samples at 48 kHz, converted with `rate`), the new one starting from the old one's shared-stage states, and crossfade linearly (equal-gain: both paths carry the same correlated signal). Then drop the old one. At most one switch per block can be in flight; a switch during a crossfade restarts it from the current mix.

*Alternatives:* not clearing at all (an added stage's zero state still gives a small transient, and a removed stage's DC path changes level); a mute-and-fade (audible dip). Rejected.

### O10: a Freeverb-style stereo room (decide by ear)

Options for the maintainer, rendered as A/B clips in task 1.2 (`Chords → Cable Echo Pad` with Mode = Factory, a struck Pipe, and the default patch with Amount 0.5):
- **A (recommended):** a right-channel comb and all-pass set with delays offset by a small mutually prime spread (Freeverb uses 23 samples at 44.1 kHz; here about 1 ms, converted to the rate), fed by the same mono sum. Proven, cheap to reason about, keeps decay and level.
- **A′ (cheaper):** shared combs, a second pair of all-passes for the right channel. Half the memory, less decorrelation (the comb tails stay identical, so low frequencies stay mono).
- **B:** keep the mono room; document it in ARCHITECTURE, the Internals page and the Wiki's Space page as a choice.

The spec delta is written for A (and A′, which must pass the same correlation bound or it isn't chosen). Under B it becomes "the Factory room's left and right tails are identical, by design".

Level: the comb gains stay as they are and each channel averages its own four combs, so the RMS per channel stays close to the mono room's; the spec bounds it to 1 dB. The size-0 bypass (`size < 0.001 → 0`) stays.

## Risks / Trade-offs

- [Crossfade needs a second filter-state set] → preallocated in `new()`, sized for `MAX_FACTOR`; `no_alloc` covers a Quality switch.
- [The stereo room changes every Factory preset] → bank tests and reseed; guide clips regenerated; the maintainer listens before merge.
- [More CPU in Factory mode] → measure with `scripts/bench_ratios.py 5` (median of 5); the room is a small part of a voice-heavy patch.
- [Correlation bound too strict for A′] → the bound comes from what A measures; if A′ can't meet it, it isn't offered.

## Migration Plan

None: no parameters or formats change. Presets keep their values and sound wider under A.

## Measurements

To fill in: O9's step at each switch before and after; O10's L/R correlation, RMS and decay per option; CPU ratios; preset levels that moved.
