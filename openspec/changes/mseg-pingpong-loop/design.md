# Design

## Context

- `MSEG::loop_region` (`src/dsp/envelopes/mod.rs:373-384`) maps `LoopMode::PingPong` to the fixed region `(3, 4)`; `advance_stage` (`:344-367`) jumps from the end stage back to the start stage, which is all Forward does. There is no notion of direction.
- `process_sample` (`:259-320`) computes each stage as `start_level + (target − start)·curve(progress)`, with a fixed `(start, target)` pair per stage. A backward stage is that same mapping with `progress` replaced by `1 − progress`.
- Only friction exciters run the MSEG (`Voice::configure_envelope`, `src/voice/mod.rs:811`, loop set at `:866-875`). Drone mode forces Forward 3 → 4 (`src/lib.rs:713-720`), so Ping-Pong never runs in Drone mode.
- The factory bank: `experimental.rs:173` draws `loop_mode` from `[0, 0, 1, 2]`; 33 presets store 2 and 9 of those use a friction exciter (Bow 1, Corrugated Drag 2, Heavy Grinding 3, Stiff Point Scrape 2, Tension Rise 1). The curated folders set `loop_mode` only to 1 (`curated.rs:648`).

## Goals / Non-Goals

**Goals:** Ping-Pong honours Start and End, never jumps, stays real-time safe and bounded (the `MAX_TRANSITIONS` cap still applies); Forward and Off are bit-identical to today.

**Non-Goals:** changing Drone mode's forced loop (that is `decide-drone-loop-step`); looping Onset or Release; per-stage direction controls.

## Decisions

1. **Implement, don't remove (recommended).** Ping-Pong as specified gives something Forward can't: a loop without the jump back to the start level, i.e. a smooth tremolo or swell for bowed and scraped sounds. Removal is cheaper and bit-identical (Ping-Pong → Forward 3 → 4) but throws away a mode users can already pick, and needs a host migration plus a BREAKING note for Loop Mode automation (range 0..2 → 0..1).
2. **A direction flag plus the existing stage table.** Add `loop_backward: bool` to `MSEG` (runtime state, reset on `trigger`). When backward, `progress` becomes `1 − progress` and `advance_stage` walks to the previous stage of the region; at the start stage it flips forward and re-enters the start stage forward; going forward, reaching the end stage flips backward and re-enters the stage before it. Each re-entered stage starts at `step`, as transitions do now. No new allocation, one branch per sample in a cold-ish path.
3. **Turns repeat the boundary stage.** The sequence for region `s … e−1` is `s → … → e−1 → (e−1)← → … → s← → s → …`. Played backward, a stage ends at its own start level, which is the previous stage's end level, so every turn is continuous. A single-stage region (the default 3 → 4) becomes a triangle between the peak and the sustain level, one Decay time per slope.
4. **Hold played backward is still flat**, Sustain is never inside a region (it can only be the end stage), so only Attack, Hold and Decay ever run backward.
5. **Release from a backward stage** keeps today's rule: `release()` stores `current_level` and ramps from it.

### If the maintainer chooses removal instead

- Spec delta: MODIFY "Every loop configuration is real-time safe" to modes Off and Forward (0–1); REMOVE nothing else; ADD "Ping-Pong state migrates to Forward 3 → 4" to `mseg-envelope` (preset files in `migrate_preset_json`, host state in `Plugin::filter_state`).
- `loop_mode_param` range 0..1, `LOOP_MODE_ITEMS` two entries, the experimental generator maps a drawn 2 to `(1, 3, 4)`, and the proposal gains a **BREAKING** line for Loop Mode automation.

## Risks / Trade-offs

- [9 experimental presets change character] → the bank tests (floor and character) and a reseed; their names say "experimental", and the gating task plays them first.
- [Golden pin `loop_output_matches_golden` changes for the `pingpong` row] → re-pin it in the same commit, with the reason in its doc comment; the Off and Forward rows must stay unchanged, which proves Forward is untouched.
- [A backward Attack with a strong curve sounds like a reversed swell, not like Decay] → intended; curves are mirrored in time, which is what "backward" means.

## Migration Plan

None for the recommended branch: same parameter, same range, same values. Saved projects using Ping-Pong sound different (called out in the proposal and the Wiki).

## Open Questions

(none: the only decision is the gating choice in tasks 1.1, and each branch is specified above)
