## ADDED Requirements

### Requirement: The body rests while BODY is 0
While BODY and its smoothed value are both 0, the body resonance SHALL do no processing and hold no ringing state. When BODY is raised from 0, the body SHALL start from rest: its contribution SHALL be that of a freshly reset body fed the post chain's input from that moment on. While BODY is above 0, the body's output SHALL be bit-identical to the body before this requirement.

#### Scenario: Raising BODY starts from rest
- **WHEN** a 200 Hz tone plays through the post chain with BODY 0 for one second, and BODY is then set to 1 (0.2 on the 0–1 internal scale) with space off
- **THEN** the body's contribution in the 50 ms after the change equals, within 1e-6 per sample, that of a reset body fed the same input from the change on and mixed through the same BODY smoother

#### Scenario: BODY above 0 is unchanged
- **WHEN** a broadband signal passes through the post chain with BODY 2, before and after this change
- **THEN** the outputs are bit-identical

#### Scenario: BODY 0 matches no body
- **WHEN** BODY is 0 and a broadband stereo signal passes through the post chain with space off
- **THEN** the output equals the same chain with the body stage removed, sample for sample
