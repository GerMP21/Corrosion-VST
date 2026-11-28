# Spec Delta

## MODIFIED Requirements

### Requirement: Zero stage level is silence
When the envelope's current stage level is 0 and Env Amount is 1, its output SHALL be exactly 0 for every velocity and every `velocity_to_level` setting. Below Env Amount 1 the output's floor while the gate is held is the flat share, `(1 − env_amount)` times the velocity gain.

#### Scenario: Low velocity at zero level
- **WHEN** an envelope with sustain level 0, Env Amount 1 and `velocity_to_level` 1 is triggered at velocity 0 and held past its decay
- **THEN** every output sample during sustain is 0.0

#### Scenario: Velocity-to-level off
- **WHEN** `velocity_to_level` is 0, Env Amount is 1, and the envelope is held at a sustain level of 0.5
- **THEN** the sustain output is 0.5 (no added offset), for any velocity

#### Scenario: Flat share at zero level
- **WHEN** an envelope with sustain level 0, Env Amount 0.25 and `velocity_to_level` 0 is held past its decay
- **THEN** every output sample during sustain is 0.75

### Requirement: Velocity scales level multiplicatively
While the gate is held, the output SHALL equal `(env_amount · level + (1 − env_amount)) · ((1 − velocity_to_level) + velocity_to_level · velocity_scale)`, where `velocity_scale = 0.5 + velocity / 2` for normalized velocity in [0, 1]. After the gate closes, the output SHALL follow the Release stage from the value it had at the gate's closing.

#### Scenario: Half-scale at velocity zero
- **WHEN** `velocity_to_level` is 1, the velocity is 0, Env Amount is 1, and the current level is 1.0
- **THEN** the output is 0.5

#### Scenario: Full velocity gives the stage level
- **WHEN** the velocity is 1.0
- **THEN** the output equals `env_amount · level + (1 − env_amount)` exactly, for any `velocity_to_level`

## ADDED Requirements

### Requirement: Env Amount 0 plays a flat push
At Env Amount 0, a held friction note SHALL sound at a steady level set by velocity, not silence, and SHALL fade with the envelope's Release when the key is let go.

#### Scenario: Bow at Env Amount 0
- **WHEN** a Bow note on Plate, C4 at velocity 100, is held 2 s at Env Amount 0 and then released
- **THEN** the note is audible while held (within 12 dB of the same note at Env Amount 1), and its force reaches zero within the MSEG Release time plus one buffer after the note-off

#### Scenario: Default unchanged
- **WHEN** each friction exciter plays Pipe and Plate at C4, velocity 100, at Env Amount 1
- **THEN** the render is bit-identical to the same note before this change
