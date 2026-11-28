## ADDED Requirements

### Requirement: The saturation curve never folds back
The post-chain drive's saturation curve SHALL be linear for input magnitudes below 0.5, and above that its output magnitude SHALL never decrease as the input magnitude increases, up to the ±5 input clamp. It SHALL approach, and never exceed, an output magnitude of 1. Below an input magnitude of 1.5 it SHALL stay within 0.03 of the previous curve, `0.5 + u/(1 + u²)` with `u = |x| − 0.5`, so moderate drive settings keep their tone.

#### Scenario: Louder in is never quieter out
- **WHEN** the saturation curve is evaluated at input magnitudes from 0 to 5 in steps of 0.01
- **THEN** each output magnitude is greater than or equal to the previous one, and none exceeds 1

#### Scenario: Linear below the knee
- **WHEN** the curve is evaluated at inputs between −0.5 and 0.5
- **THEN** each output equals its input exactly

#### Scenario: Moderate inputs keep their tone
- **WHEN** the curve is evaluated at input magnitudes from 0.5 to 1.5
- **THEN** each output is within 0.03 of the previous curve's output at the same input

#### Scenario: Hot inputs hold near full scale
- **WHEN** the curve is evaluated at input magnitudes 3 and 5
- **THEN** both outputs are at least 0.99, where the previous curve gave 0.845 and 0.712

#### Scenario: Odd symmetry
- **WHEN** the curve is evaluated at an input and at its negation
- **THEN** the two outputs are negatives of each other
