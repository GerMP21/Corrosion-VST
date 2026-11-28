## ADDED Requirements

### Requirement: A moving strike position weights the modes accurately
While the effective strike position moves (wander or Position Envelope above 0), each mode's position weight SHALL be within 1e-5 of `sin(n·π·p)` for mode number `n` and effective position `p` at that sample, for every mode the resonator holds. With the position still, the weights SHALL be bit-identical to the weights before this requirement.

#### Scenario: Wandering weights stay accurate
- **WHEN** a Pipe note with the largest mode count (Render quality) plays with Wander 0.5 and Wander Rate 5 Hz for two seconds at 48 kHz
- **THEN** at every sample every mode's position weight is within 1e-5 of `sin(n·π·p)` computed in double precision

#### Scenario: Still position is unchanged
- **WHEN** a note plays with Wander 0 and Position Envelope 0
- **THEN** its output is bit-identical to the output before this requirement

#### Scenario: Wandering output barely moves
- **WHEN** the same wandering note is rendered before and after this requirement
- **THEN** the difference between the two outputs is at least 80 dB below the note's RMS
