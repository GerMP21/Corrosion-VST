## ADDED Requirements

### Requirement: The Bow's roughness ripple has one pitch at every rate
The Bow exciter's roughness ripple SHALL have a frequency in hertz set by the bow speed alone, below the Nyquist frequency at every supported sample rate, so a bowed note's roughness has the same pitch at 44.1, 48 and 96 kHz.

#### Scenario: Same ripple at three rates
- **WHEN** a default Bow note with Roughness 1 is rendered at 44.1, 48 and 96 kHz
- **THEN** the ripple's frequency, measured on the exciter force, is the same at the three rates within 1 %

#### Scenario: No folded ripple
- **WHEN** the bow speed is at its maximum at 44.1 kHz
- **THEN** the ripple's frequency is below 22.05 kHz and equals the frequency at 96 kHz within 1 %
