## ADDED Requirements

### Requirement: Changing Quality doesn't click
Switching the quality mode while audio plays SHALL move the diode stage to its new oversampling without a discontinuity: no output step larger than the signal's own largest step in the surrounding milliseconds.

#### Scenario: Switching every pair of modes under a sine
- **WHEN** a 200 Hz sine at 0.9 of the ceiling runs through the post chain and the quality mode is switched between every pair of Eco, Normal, High and Render, one switch at a time
- **THEN** the largest sample-to-sample step in the 10 ms after each switch is no larger than 1.1 times the largest step in the 10 ms before it

#### Scenario: Steady state is unchanged
- **WHEN** the quality mode stays fixed for a whole render
- **THEN** the output is bit-identical to the same render before this change
