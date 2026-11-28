# Spec Delta

## ADDED Requirements

### Requirement: Spread Width is continuous from 0
The spread stage's output SHALL change continuously as Spread Width moves away from 0: at Width 0 it passes the input unchanged, between 0 and 0.05 it fades from the input to the processed spread, and from 0.05 up it is the processed spread. The spread's delay and filters SHALL keep running at Width 0, so leaving 0 never starts from stale state.

#### Scenario: Low tone across the bottom of the range
- **WHEN** a 100 Hz mono sine at 0.5 peak passes the spread stage at proximity 0.5 with Width 0, 0.001, 0.01, 0.025 and 0.05
- **THEN** the output RMS moves monotonically from the Width 0 value to the Width 0.05 value, and Width 0.001 is within 0.1 dB of Width 0

#### Scenario: Leaving Width 0 after a long time
- **WHEN** a 200 Hz sine has played at Width 0.5, then at Width 0 for 2 s while its level changed, and Width then moves back to 0.5
- **THEN** the output peak over the first 10 ms after the move is within 0.5 dB of the steady-state peak at Width 0.5 for the current input (no burst of stale delay or filter state)

#### Scenario: Above the fade range nothing changes
- **WHEN** a preset whose Spread Width is 0.05 or more is rendered
- **THEN** the output is bit-identical to the same render before this change
