# Spec Delta

## MODIFIED Requirements

### Requirement: Loop region is defined by start and end stages
With looping enabled and the gate held, reaching the loop end stage SHALL restart the loop start stage, reaching that stage's start level through a short glide (see "A loop restart has no step"). Stages are numbered 0 Onset, 1 Attack, 2 Hold, 3 Decay, 4 Sustain, 5 Release. A start of 0 is treated as 1 and a start or end of 5 as 4, since Onset and Release are never looped.

#### Scenario: Default loop Decay to Sustain
- **WHEN** loop mode is Forward with start 3 and end 4 and the gate is held
- **THEN** the envelope repeats the Decay stage each time it reaches Sustain, rising from the sustain level back to the peak through the restart glide before each Decay, at the same period as before this change plus the glide time

#### Scenario: Custom loop end
- **WHEN** loop mode is Forward with start 1 (Attack) and end 3 (Decay) and the gate is held
- **THEN** on reaching Decay the envelope restarts Attack, and it never enters Decay or Sustain while the gate stays held

## ADDED Requirements

### Requirement: A loop restart has no step
When a loop restarts its start stage, the envelope SHALL move from its current level to that stage's start level over a glide of at least 2 ms and at most 20 ms, with no sample-to-sample change larger than the glide's total change divided by the glide's length in samples. A glide SHALL be real-time safe and SHALL end at once when the gate closes, Release starting from the level the glide had reached.

#### Scenario: Drone's forced loop at the defaults
- **WHEN** a Bow note on a Plate is held for 3 s in Drone mode with default controls
- **THEN** every return from the sustain level (0.5) to the peak (0.75) takes between 2 and 20 ms, and no single-sample change of the force envelope exceeds 0.25 divided by the glide's length in samples

#### Scenario: Release during a glide
- **WHEN** the gate closes halfway through a restart glide
- **THEN** Release starts from the glide's current level, with no step, and the envelope reaches Idle within the release time plus one buffer

#### Scenario: Loops without a level change
- **WHEN** a loop restarts a stage whose start level equals the current level
- **THEN** the envelope's output is the same as a restart with no glide
