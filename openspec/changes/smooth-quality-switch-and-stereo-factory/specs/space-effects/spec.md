## ADDED Requirements

### Requirement: The Factory room has a stereo tail
In Factory mode the reverb SHALL produce different left and right tails from the same input, so the room is heard as wide, while its level and decay time stay those of the mono room.

#### Scenario: Mono input gives a wide tail
- **WHEN** an impulse is fed to both inputs with Space = Factory, Amount 1 and default room controls
- **THEN** the normalized correlation between the left and right wet tails over the first 500 ms is below 0.5

#### Scenario: Level and decay are kept
- **WHEN** the same impulse is rendered through the stereo room and through the previous mono room
- **THEN** the stereo room's wet RMS (mean of both channels) is within 1 dB of the mono room's, and its 30 dB decay time is within 5 %

#### Scenario: Decay still doesn't depend on the sample rate
- **WHEN** the impulse test runs at 44.1, 48, 96 and 192 kHz
- **THEN** each channel's 30 dB decay time is within 5 % of its 48 kHz time
