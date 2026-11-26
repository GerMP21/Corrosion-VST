## ADDED Requirements

### Requirement: Spread Width widens the image while keeping the mono level
Raising Spread Width SHALL never narrow the stereo image: the ratio of side to mid energy in the output SHALL not decrease as Spread Width rises, and at Spread Width 1 the side SHALL be at least 3 dB above the dry input's side. A mono input SHALL stay mono, and its low-frequency level SHALL stay within 1 dB across the whole range at the default Listener Proximity (Proximity may still tilt the tone above that).

#### Scenario: Side energy rises with Spread Width
- **WHEN** a stereo noise signal with a side-to-mid ratio of −6 dB passes through the post chain (BODY 0, Space off) at Spread Width 0, 0.25, 0.5, 0.75 and 1
- **THEN** the output's side-to-mid energy ratio does not decrease from one setting to the next, and at 1 it is at least 3 dB above its value at 0

#### Scenario: Mono stays mono
- **WHEN** a mono signal (left equal to right) passes through the post chain at any Spread Width
- **THEN** the left and right outputs are equal within 1e-6 per sample

#### Scenario: Spread doesn't change the mono level
- **WHEN** a 100 Hz mono sine passes through the post chain at Listener Proximity 0.5 and Spread Width 0, 0.25, 0.5, 0.75 and 1
- **THEN** its output RMS at every Spread Width is within 1 dB of its RMS at Spread Width 0
