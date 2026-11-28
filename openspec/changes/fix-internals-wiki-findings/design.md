# Design

## Context

Each claim was re-checked against `dev` on 2026-11-26 by reading the code (none re-measured yet; the gating and test-first tasks measure them):

| # | Item | Where | Confirmed |
|---|---|---|---|
| 1 | Bow ripple | `src/dsp/exciters/bow.rs:120-126`: `bow_position += rate::step(bow_velocity, ratio)`, wrapped at 1, then `sin(2π·bow_position)`. The phase moves ~`v` cycles per 48 kHz sample, so the ripple is `v · 48 kHz` in true frequency and folds. The backlog's three numbers fit `v ≈ 0.938` exactly (48 000 − 45 041 = 2959; 45 041 − 44 100 = 941). | yes (arithmetic) |
| 2 | Wire Stiffness | `src/dsp/exciters/wire_brush.rs:165-179`: `alpha = dt/(RC + dt)` used in the high-pass form `alpha·(y + x − x₋₁)`. With `alpha ≪ 1` that is a scaled differentiator whose gain grows with the cutoff, so stiffness mostly scales level. | yes (reading) |
| 3 | Pos Wander | `src/params.rs:2306-2312` (range 0–1) vs `src/dsp/interaction.rs:81-84` (`amount.clamp(0.0, 0.5)`). Factory bank: max 0.3998 (`experimental.rs:186` draws 0–0.4). | yes |
| 4 | Web diode dot | `Plugin::process` publishes after `process_block` (`src/lib.rs:1511-1512`); `WebEngine::process` (`src/web/mod.rs:146-162`) calls `process_block` and never publishes; `report()` takes the meter (`:216`). | yes (reading) |
| 5 | Lookahead stale samples | `limit_frame` (`src/lib.rs:1279-1291`) picks the path per block; the lookahead limiters are reset only in `panic_frame` (`:1267-1268`) and `reset()`. | yes (reading) |
| 6 | Empty kit → Tonal | `Preset::from_params` writes `kit` only when non-empty (`src/presets/mod.rs:704-707`); `migrate_old_kit_mode` (`:83-90`) turns Kit without `kit` into Tonal. Host state is unaffected: `kit` is a `#[persist]` field (`src/params.rs:97`) that is always written, so `migrate_host_old_kit_mode` (`src/lib.rs:410-420`) sees it. | yes (reading; host claim to test) |
| 7 | Drop Envelope Spread | host 40–400 ms (`src/params.rs:2071-2078`) vs `spread_ms.clamp(50.0, 1000.0)` (`src/dsp/exciters/metal_chain.rs:111`). | yes |
| 8 | Stale docs | Examples confirmed: Brightness "±2 octaves" (`src/lib.rs:194`, `:244`) while `2^(2b)` with `b` in ±0.5 is ±1 octave; `PlayMode` doc (`src/params.rs:1250-1262`) still describes 10-note kit slots; `src/gui/mod.rs:1-10` (4 sections, Pipe/Plate/Tank/Chain); `src/dsp/utils/mod.rs:1-5` ("not used in the real-time plugin path"); `WdfLadderFilter` in `wdf_filter.rs` is a TPT ladder. | partly: the backlog says the full ~30-item list is in the archived `add-internals-wiki/design.md`; it is not (that file only says "stale comments and docs"). The Internals pages describe each discrepancy, so the list is rebuilt from them. |

## Goals / Non-Goals

**Goals:** each item fixed so the code, its help text, the Wiki and the Internals pages agree; each sound change gated by the maintainer's ear; every fix covered by a test that fails on today's code.

**Non-Goals:** Spread Width's help (`widen-hrtf-spread`); the stale comments `tidy-review-nits` owns (N3 P3.1 comments, N4 "never fires", N7 MSEG doc order); real Bow stick-slip (`bow-stick-slip`); splitting files (`split-largest-source-files`).

## Decisions

### 1. Bow ripple: `K · v` hertz (recommended)

- The ripple phase advances by `K · v / sample_rate` cycles per sample, `K` set so the default patch's ripple at 48 kHz stays at today's ~2959 Hz (`K ≈ 3150 Hz` per unit speed, measured in task 1.1). Lower speeds give a slower ripple, which is what a rougher, slower bow does.
- Alternative A: keep today's 48 kHz sound exactly by defining the ripple as the folded 48 kHz frequency (`fold(v · 48 kHz)` in hertz) at every rate. Bit-near at 48 kHz for every speed, but the pitch-versus-speed curve stays a zig-zag nobody designed.
- Alternative B: drop the ripple from the force (Roughness keeps its friction term). Spec delta becomes "Roughness never adds a component above Nyquist".
- Bowed presets: render before/after, bank tests, reseed. Land this before `bow-stick-slip`.

### 2. Wire Stiffness: a real high-pass, level-compensated (recommended)

- `alpha = RC/(RC + dt)`; the output scaled by a gain that keeps a single impulse's peak or energy roughly constant across 0–1 (fit in task 2.1), so the control moves brightness, as the help text says.
- Alternative: keep the sound and rename/re-describe it as a level control ("Wire Force"?): a parameter-name change is a host-visible rename; the spec delta becomes "its help says it sets the brush's level". Not recommended: Pressure already sets level.
- Wire Brush presets move: bank tests, reseed.

### 3. Position Wander: shrink the range to 0–0.5 (recommended)

- `position_wander_param` becomes 0–0.5 (default unchanged at 0). Values above 0.5 in presets and projects clamp on load to 0.5, which is exactly what the bus did with them, so saved sounds are bit-identical: no migration code. **BREAKING for automation:** normalised automation now spans half the old plain range.
- Alternative A, rescale (`depth = 0.5 · value`, range kept 0–1): automation keeps its normalised meaning but every stored value would need halving, and old and new presets can't be told apart without a new marker field; **BREAKING** too, with migration risk.
- Alternative B, let the bus use 0–1: the strike position then sits at 0 or 1 for part of each cycle, where `sin(nπp) = 0` and the exciter stops driving the modes, so wander above 0.5 would add dropouts. Not recommended.

### 4. Publish the diode meter inside the shared block path

`process_block` ends by publishing `post_chain.take_clipper_reduction_db()` to the meter; `Plugin::process` drops its own publish. One path, so the plugin and the web app can't diverge again. Plugin output is bit-identical (the meter is read-only for audio).

### 5. Limiter switch: clear, then crossfade from the Hard path

On a mode change into Lookahead, clear both delay lines (`LookaheadLimiter::reset`) and, for the delay's length (48 samples), crossfade linearly from the Hard-limited sample to the lookahead output, so there is neither stale audio nor a 1 ms gap. On a change into Hard, nothing is replayed (the Hard path has no state); clear the lookahead lines then too, so the next switch starts clean. Mode changes are already detected per block (`latency_change`).

### 6. An empty kit in Kit mode writes `kit` (recommended)

`Preset::from_params` writes the `kit` field whenever Note Mode is Kit, even with no pads; other modes keep omitting an empty kit. A preset saved before `per-key-kits` still has no `kit` and still loads as Tonal. Alternative: keep today's behaviour and document it in the Wiki's kit page (spec unchanged).

### 7. Drop Envelope Spread: clamp 40–1000 ms

Match the clamp to the host range's bottom. No factory preset stores under 50 ms; user presets stored at 40–50 ms change (they were inert there).

### 8. Stale docs: list first, then fix

Task 8.1 rebuilds the list from the Internals pages (every "the comment says", "despite its name", "still says" note) and the backlog's examples, into `## Measurements` with file and line, then each line is fixed. The filter file is renamed to `ladder_filter.rs` with `LadderFilter` (a mechanical rename, Internals source links and ARCHITECTURE updated), after `trim-post-chain-per-sample-work` lands because O3 edits that file.

## Risks / Trade-offs

- [Items 1 and 2 move presets the maintainer has already approved] → gated A/B, bank character checks, a reseed listing what moved.
- [Item 3's automation break] → called out in the proposal and the Wiki; the factory bank and saved state are unaffected.
- [Item 5's crossfade mixes two signals 48 samples apart for 1 ms] → a comb for 1 ms; measured by the no-dropout scenario and a click check (`scripts/check_clicks.py`) on the switch.
- [A large rename collides with other in-flight changes] → item 8's rename is its own commit, applied last.

## Migration Plan

- Item 3: none needed (clamping on load reproduces today's depth); release note for automation.
- Item 6: presets saved after this change carry `"kit"` in Kit mode; older readers ignore nothing (the field already exists).

## Open Questions

(none: items 1, 2, 3 and 6 are decided at their gating tasks between the options above)

## Measurements

(filled in during apply)
