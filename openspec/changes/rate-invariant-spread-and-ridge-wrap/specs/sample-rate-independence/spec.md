# Spec Delta

## ADDED Requirements

### Requirement: Spread proximity and spring dispersion sound the same at every sample rate
The spread stage's proximity filters and the Spring's dispersion filter SHALL have the same corner frequencies in hertz at every supported sample rate, and SHALL be bit-identical to their previous behaviour at 48 kHz.

#### Scenario: Proximity tone at 44.1 and 96 kHz
- **WHEN** white noise passes the spread stage at Width 0.5 and Listener Proximity 0, 0.5 and 1, at 44.1, 48 and 96 kHz
- **THEN** the output's level in each third-octave band from 100 Hz to 16 kHz at 44.1 and 96 kHz is within 0.5 dB of the 48 kHz level

#### Scenario: Spring tone at 96 kHz
- **WHEN** an impulse passes Space = Spring at Wire Stiffness 0, 0.5 and 1, at 48 and 96 kHz
- **THEN** the first echo's spectrum at 96 kHz is within 1 dB of the 48 kHz echo in each third-octave band up to 16 kHz

#### Scenario: 48 kHz unchanged
- **WHEN** the spread and Spring stages process the same signal at 48 kHz before and after this change
- **THEN** the outputs are bit-identical
