# Spec Delta

## Purpose
Defines how the Bow exciter drives an object: real stick-slip motion, a pitched sound without subharmonics where the bow is played normally, and behaviour that does not depend on the sample rate.

## ADDED Requirements

### Requirement: The bow sticks and slips once per period
On a held Bow note at default controls, the bow SHALL alternate between sticking to the surface (relative speed below the friction curve's sticking threshold) and slipping, with one slip per period of the note's fundamental.

#### Scenario: Taut Cable at the default patch's bow
- **WHEN** a Bow note on Taut Cable, C3 at velocity 100 with default controls, has been held for 1 s
- **THEN** over the next 0.5 s the contact spends between 20 % and 90 % of the time sticking, and the interval between successive slips is within 2 % of the fundamental's period

#### Scenario: The sliding bow is gone
- **WHEN** a Bow note on Plate, C4 at velocity 100 with default controls, is held for 1 s
- **THEN** the contact sticks for at least 20 % of the second held second

### Requirement: Bowed notes are pitched and free of subharmonics
In its playable region (default Bow controls, and every factory preset that uses the Bow) a held Bow note SHALL have its strongest spectral peak within 50 cents of the played pitch or an octave of it, and no spectral peak below 0.9 times the fundamental within 20 dB of the fundamental.

#### Scenario: Default bow across the keyboard
- **WHEN** a Bow note with default controls is held 2 s on Taut Cable, Pipe and I-Beam at C2, C3 and C4, velocity 100
- **THEN** each note's strongest peak is within 50 cents of the played pitch or an octave of it, and no subharmonic peak comes within 20 dB of the fundamental

#### Scenario: Bowed factory presets
- **WHEN** each factory preset that uses the Bow renders a held middle C for 3 s
- **THEN** it has no subharmonic peak within 20 dB of its fundamental, and it passes its folder's character checks

### Requirement: The raucous regime is at the edge, not the default
Bow Pressure at its maximum with Bow Speed at its minimum SHALL be allowed to produce the raucous regime (aperiodic or subharmonic motion), and the change from pitched to raucous SHALL happen only above the default Bow Pressure.

#### Scenario: Pressure sweep
- **WHEN** Bow Pressure is swept from its minimum to its maximum in 8 steps on Taut Cable C3, other controls default
- **THEN** every step up to and including the default is pitched as above, and the first raucous step, if any, is above the default

### Requirement: Stick-slip does not depend on the sample rate
The bow's stick-slip motion SHALL be the same at 44.1, 48 and 96 kHz.

#### Scenario: Three rates
- **WHEN** a Bow note on Taut Cable, C3 at velocity 100 with default controls, is rendered at 44.1, 48 and 96 kHz
- **THEN** the slip interval agrees within 1 % across the three rates, and the note's level agrees within 1 dB
