# Design: bow-stick-slip

## Context

See proposal.md for why. Current code (`dev`, 2026-11-26):
- `BowExciter::process_sample(resonator_velocity)` (`src/dsp/exciters/bow.rs:98-129`) computes `v_rel = bow_velocity − resonator_velocity`, maps `|v_rel|` through a Stribeck curve (smoothstep below `stribeck_speed = 0.01 + 0.02·speed·slip_curve`, exponential fall-off above it at `slide_falloff`, `:116-117`) and returns `pressure · μ · sign(v_rel)`. It never solves against the surface: the force doesn't know how the surface will answer it, so there is no sticking branch, only a curve evaluated at the current relative speed.
- The voice calls it with the resonator's velocity (`src/voice/mod.rs:1771`) and scales the force by `EXCITER_DRIVE_LEVEL[1] = 2.1664e-02` (`:638`), the governor and Pressure (`force_scale`, `:1675`).
- The surface moves at 0.0002–0.007 against a bow at about 0.94 (`fix-inert-controls` design, task 2.3), so `|v_rel|` never drops under `stribeck_speed` and the bow always slides.
- `fix-inert-controls` (task 2.3) scaled the seen surface speed by ×200, ×300 and ×1000. At ×1000 the bow stuck and slipped, but all 12 bowed chord presets failed their character (−16 to −35 dB, a ~20 Hz subharmonic, partials up to 600 cents off), Glass Sheet Lead fell −10.7 dB off pitch and 6 experimental Bow presets rose 8–14 dB. A gain on the seen velocity alone moves every preset into the high-force corner of the playable region at once: the bow force wasn't rescaled with it.
- Hand Strike already solves a contact against the surface with `contact_prediction` (`src/voice/mod.rs:1660`, the resonator's predicted next-sample response to a force), so the bus can provide what a friction solve needs.
- The Bow's Roughness ripple aliases (its phase advances by the bow speed per 48 kHz sample, `bow.rs:126`); that fix is in `fix-internals-wiki-findings`, which lands first.

## Goals / Non-Goals

**Goals:**
- Helmholtz-like stick-slip, one slip per period, at the defaults and in the bank.
- The bowed presets keep their folder character after re-voicing.

**Non-Goals:**
- A bowed-string waveguide or a torsional model; the resonator stays modal.
- Keeping the old sliding bow as a mode (Decision 4 records the alternative).
- Changing the other friction exciters (Stiff Point Scrape, Corrugated Drag, Heavy Grinding, Tension Rise).

## Decisions

### 1. Solve the friction curve against the surface's admittance

Each sample the bow takes the surface's free velocity `v_free` (where the surface would move with no bow force) and its admittance `Y` (how much one unit of force moves it this sample) from the resonator's contact prediction, and finds the force `F` and contact velocity `v_c = v_free + Y·F` that satisfy `F = N · μ(v_b − v_c) · sign(v_b − v_c)` (the McIntyre–Schumacher–Woodhouse intersection). If the line meets the curve's sticking branch (`|F| ≤ N·μ_static` with `v_c = v_b`), the contact sticks; otherwise it takes the slipping solution, keeping the previous branch when there are three intersections (hysteresis). The solve is closed-form per branch (the curve is piecewise: a vertical sticking segment and a hyperbolic sliding branch), so the work is bounded and allocation-free.

*Alternatives:* a gain on the seen surface velocity (tried in `fix-inert-controls`; it sticks, but the force isn't rescaled, so presets go raucous); a Newton iteration on the smooth Stribeck curve (unbounded iterations on the audio thread, and no hysteresis).

### 2. Map Bow Pressure and Bow Speed into the playable region

Schelleng's diagram bounds the bow force that gives Helmholtz motion between `F_min ∝ v_b / β²` and `F_max ∝ v_b / β` for a bow at relative position `β` along the object. The bow already has a position: the strike position (`strike-position`). Bow Pressure is mapped onto that interval for the current Bow Speed and position (log-spaced from just under `F_min` to just over `F_max`), with the default inside it; the top of the knob crosses `F_max` into the raucous regime on purpose. The object's "characteristic impedance" in those bounds comes from the bank's admittance at the contact, computed at note-on.

*Alternative:* leave the mapping linear and only re-voice presets. That puts the default patch's bow wherever the old numbers land; the experiment showed that is mostly raucous.

### 3. Units and drive

The bow's speed is expressed in the surface's units (so `v_b` is of the order of the surface velocities, 1e-3) rather than seeing the surface ×1000; the friction force is then re-scaled so that `EXCITER_DRIVE_LEVEL[1]` puts the median bowed note at the common exciter level (`exciters_play_at_a_common_level`, `default_notes_do_not_overdrive_the_voice`). The level is found with `print_exciter_overdrive` as before.

### 4. No legacy bow mode (decide)

- **Recommended:** replace the model. User presets and projects with a Bow sound different; the Wiki and the release notes say so.
- **Alternative:** a Bow Model selector (Sliding / Stick-slip), with preset and host migrations sending old state to Sliding. It keeps user sounds but adds a control whose only purpose is history, and doubles the Bow's tests. Under this alternative the spec gains "Older state keeps the sliding bow" and the default becomes Stick-slip.

### 5. Voicing by ear (decide)

The bowed presets are re-voiced after the model lands: Bow Pressure, Bow Speed, Slip Curve, Rosin Grip and the strike position first, the object controls only if needed. Task group 1 renders before/after clips of four representative presets (Bowed Cable Sub, Pipe Bow Lead, Sheet Bow Organ, Bowed Beam Drone) at the default patch's mapping for the maintainer before the remaining presets are touched.

## Risks / Trade-offs

- [The contact prediction is a one-sample linearisation; with very stiff objects the stick branch may chatter] → hysteresis plus a minimum stick time of one sample; measure on Industrial Cog and Chain, the stiffest banks.
- [CPU: the solve adds a few multiply-adds and a branch per sample per bowed voice] → measured with `scripts/bench_ratios.py 5` on 16 bowed voices; report if above 1.05×.
- [Rate dependence: the per-sample admittance changes with the rate] → the solve uses the prediction for the current rate; covered by the spec's three-rate scenario.
- [User presets with Bow move] → stated in the proposal and in the Wiki; not migratable unless Decision 4's alternative is chosen.
- [Release snap changes] → the static deflection under a sticking bow differs; `add-release-snap` comes after and measures the final model.

## Migration Plan

No parameter changes; presets reseeded. Rollback is reverting the change and reseeding.

## Open Questions

- The exact Bow Pressure taper inside the playable region (found while re-voicing; doesn't change the spec).
