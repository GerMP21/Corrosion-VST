# Design: add-release-snap

## Context

See proposal.md for why. Current code (`dev`, 2026-11-26):
- `Voice::note_off` (`src/voice/mod.rs:1476-1484`) only marks the voice inactive and releases the force envelope; since `fix-inert-controls` the friction exciters keep running and the MSEG release (`MSEG::release`, `src/dsp/envelopes/mod.rs:236`, ramping from `release_level`) fades their force.
- While held, a friction exciter pushes the object with a one-signed (static) force component as well as the stick/slide texture, so the object sits deflected. The voice-mix DC blocker (`block-voice-mix-dc`, 10 Hz high-pass) removes that deflection from the output while held, so the held note sounds quiet. When the push goes, the object rings free from the deflection: the snap. A step in the static force is a broadband kick; the faster the push goes, the louder the ring.
- The numbers disagree by measuring point, and must be re-measured before anything else:
  - `reorganize-factory-presets` (design, "The failures came from the release") measured at the plugin output: default patch Plate + Corrugated Drag −48.1 → −17.6 dB (+30.5 dB), Pipe + Corrugated Drag +25.2 dB, Plate + Bow +16.9 dB; bank Bowed Cable Sub +15 dB, Bow · Taut Cable +37 dB.
  - `fix-inert-controls` (design, task 2.4) measured tail over held at the voice (`bow_release_db`, `src/voice/mod.rs:4357`, before the voice-mix DC blocker, so the held energy includes the deflection) and found −11.7 dB for Plate + Corrugated Drag at the default MSEG Release. Those figures can't be compared with the first set.
- The five friction exciters are Bow, Stiff Point Scrape, Heavy Grinding, Corrugated Drag and Tension Rise (`ExciterType::family`, `src/params.rs:958`). The backlog names the first, second and fourth as snapping; Tension Rise's tail is louder than its held note at long releases (+4.3 dB at MSEG Release 1 s) for a different reason (its tension keeps rising), and Heavy Grinding's static push is small.

## Goals / Non-Goals

**Goals:**
- One control that bounds the snap, with today's sound exactly reachable.
- The factory bank checks released notes, not only held ones.

**Non-Goals:**
- Changing MSEG Release's meaning, or the Hit and Specialty releases (C1's specialty release is `fix-specialty-adsr-release`).
- Removing the snap by default without the maintainer's say.

## Decisions

### 1. Separate the static push from the texture at note-off

At note-off the voice splits the friction force into a slow part (a one-pole low-pass of the force, about 20 Hz, tracked while held) and the rest. From note-off on, the rest follows the MSEG release as today; the slow part is withdrawn over an extra ease time `T = (1 − snap)² · T_max` with `T_max` around 1.5 s (a raised-cosine ramp, so it has no corner). At Release Snap 1, `T = 0` and the code path is skipped entirely, which gives bit-identity by construction (proved by render hashes, Decision 4).

*Alternatives:*
- **Lengthen MSEG Release for the slow part only by scaling MSEG Release:** couples two controls and still steps at short releases.
- **An output gain bound after note-off (a governor on the tail):** bounds the level but squashes the ring's shape and pumps; it treats the symptom.
- **Remove the voice-mix DC blocker's role:** the deflection would be audible as DC while held; rejected by `block-voice-mix-dc`.

`T_max` and the cutoff are set in task 1 so the Release Snap 0 bound (spec) holds on every object with margin; the first measurement may show that a ramp alone isn't enough on Taut Cable (a high-Q low mode). If so, stop and report before choosing a stronger mechanism.

### 2. Where the control lives

A host parameter `release_snap` (FloatParam 0–1, linear), drawn as a fader in the friction envelope (MSEG) panel next to MSEG Release, shown only for the friction family (`FIXED_CONTROLS` with `Family(Friction)`). It is read at note-off, like MSEG Release (a held note picks up the value it has when released). It joins `VoiceControls`, the kit pad's per-pad values (`KitPad` stores every voice parameter by ID) and the randomizer's friction parameters.

### 3. The default (decide by ear)

- **Recommended: 1.** Nothing moves: default patch, bank and old state render as today; the bank's new released-note bound is met by giving the offending presets a lower value.
- **Alternative: a bounded default (e.g. 0.5).** New instances and Init ease back; the migration then matters (state without the field gets 1). Under this choice the spec's "Older state keeps its snap" is unchanged, and a scenario "the default patch's friction notes are no more than 6 dB louder after note-off" is added to `friction-release`.

Task 1 renders A/B clips (Release Snap 1, 0.5, 0 on Bowed Cable Sub, Plate + Corrugated Drag, Stiff Point on Plate) for the maintainer; implementation of the default waits on that choice.

### 4. Proving "unchanged at 1"

A test-only switch `Voice::release_snap_enabled` (the pattern of `governor_enabled`) compares the new and old paths sample for sample on all 16 exciters; plus a one-off `print_48k_exciter_render_hashes` comparison against a clean worktree, recorded in Measurements.

### 5. Migrations

- Presets: `migrate_preset_json` (`src/presets/mod.rs`) inserts `release_snap: 1.0` when absent (schema stays "4", additive field).
- Projects: `Plugin::filter_state` (`src/lib.rs`) inserts the same when the state has parameters but no `release_snap` (the pattern of `migrate_host_diode_stage`; an empty state is left to the defaults).
- If the default is 1, both migrations are no-ops in effect but are still written, so a later default change can't silently move old state.

### 6. The bank

`src/presets/factory/measure.rs` gains a held-then-released render; the bank test asserts the released-note bound (spec). Curated friction presets that fail get a hand-chosen Release Snap; `experimental.rs` draws Release Snap in a range that passes (e.g. 0–0.6 for Bow, Stiff Point and Corrugated Drag on high-Q objects), redrawing on failure as it already does for the floor. The seeder rewrites every file (new field) and `levels.rs`.

## Risks / Trade-offs

- [The split misses objects whose deflection sits in a mode near 20 Hz] → measure the worst object in task 1; lower the cutoff or raise `T_max`; report if neither holds.
- [Presets voiced around the snap lose character] → only presets that fail the bound get a lower value; their before/after is listed in Measurements and the maintainer listens.
- [CPU: one more one-pole per friction voice] → negligible; checked within noise with `scripts/bench_ratios.py 5`.
- [The fix-inert-controls voice-level numbers mislead] → all bounds are measured at the plugin output.

## Migration Plan

Additive parameter; migrations above. Rollback is reverting the change and reseeding.

## Open Questions

- `T_max` and the low-pass cutoff (task 1 measurement; doesn't change the spec).
