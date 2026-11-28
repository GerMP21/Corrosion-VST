# Spec Delta

## Purpose
Defines the linear ADSR force envelope of the specialty exciters (Pneumatic Jet, Electromagnetic Hum, Particle Rain, Tension Snap): how its stages run while a key is held and how its Release brings every released note to silence.

## ADDED Requirements

### Requirement: Release starts from the level at note-off
When a specialty note is released, its force envelope SHALL fall linearly from the level it had at note-off to 0 over Env Release, whichever stage it was in (Attack, Decay or Sustain) and whatever the Sustain level.

#### Scenario: Released during Decay with a low Sustain
- **WHEN** a Pneumatic Jet note on a Pipe with Env Attack 0.05 s, Env Decay 0.3 s, Env Sustain 0.1 and Env Release 0.5 s is released 0.1 s after note-on
- **THEN** its force envelope reaches 0 between 0.45 s and 0.55 s after note-off, not ten times later

#### Scenario: Released in Sustain
- **WHEN** a specialty note with Env Sustain 0.7 is held past its Decay and then released
- **THEN** its output is bit-identical to the same note before this change

### Requirement: Every released specialty note ends
A released specialty voice SHALL stop rendering after its force envelope and its resonator tail have died away, for every Env Sustain value including 0, so it frees its voice slot without being stolen.

#### Scenario: Sustain 0, released during Decay
- **WHEN** Pneumatic Jet, Electromagnetic Hum and Particle Rain each play Pipe C4 with Env Attack 0.05 s, Env Decay 0.3 s, Env Sustain 0 and Env Release 0.5 s, released 0.1 s after note-on
- **THEN** each force envelope is 0 within Env Release plus 10 ms of note-off, and no voice is still sounding 20 s after note-off (output RMS between 19 and 20 s is 0)

#### Scenario: Sustain 0, released after the Decay ended
- **WHEN** the same notes are released 1 s after note-on, after the Decay has reached 0
- **THEN** no voice is still sounding 20 s after note-off
