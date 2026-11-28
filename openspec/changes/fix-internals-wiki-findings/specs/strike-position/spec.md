## ADDED Requirements

### Requirement: Position Wander's whole range acts
Every value of Position Wander the host offers SHALL set a different wander depth: the parameter's range SHALL equal the depth the strike-position bus applies (0 to 0.5), and saved presets and projects SHALL keep the depth they had.

#### Scenario: The top of the range acts
- **WHEN** a held Bow note on Plate is rendered with Position Wander at 90 % and at 100 % of its range
- **THEN** the two outputs differ

#### Scenario: A stored value above the new range keeps its sound
- **WHEN** a preset or a DAW project storing Position Wander 0.8 is loaded
- **THEN** Position Wander is 0.5 and the note is bit-identical to the same preset rendered before this change
