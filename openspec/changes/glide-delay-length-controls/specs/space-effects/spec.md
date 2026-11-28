## ADDED Requirements

### Requirement: Delay-length controls glide
When Echo Delay Time (or the tempo-synced division that replaces it), Factory Size or Spring Tank Size changes while the Space section is running, the affected delay SHALL move to its new length gradually, without a single-sample jump of its read position, and its read position SHALL never move faster than one sample per sample relative to real time.

#### Scenario: Echo Delay Time jump
- **WHEN** a 200 Hz sine at amplitude 0.5 plays through Space = Echo (no modulation, no damping) at 48 kHz, and Delay Time changes from 0.25 to 0.75 between two samples
- **THEN** in the second after the change, no sample-to-sample step of the wet signal exceeds 2.5 times the largest step of the steady wet signal before the change

#### Scenario: Synced division change
- **WHEN** a synced echo at 120 BPM switches from 1/8 to 1/4 while the same sine plays
- **THEN** the same bound holds in the second after the switch

#### Scenario: Factory Size jump
- **WHEN** the same sine plays through Space = Factory and Factory Size changes from 0.2 to 0.8 between two samples
- **THEN** in the second after the change, no sample-to-sample step of the wet signal exceeds 2.5 times the largest step of the steady wet signal before the change

#### Scenario: Spring Tank Size jump
- **WHEN** the same sine plays through Space = Spring and Tank Size changes from 0.2 to 0.8 between two samples
- **THEN** in the second after the change, no sample-to-sample step of the wet signal exceeds 2.5 times the largest step of the steady wet signal before the change

#### Scenario: Read speed is capped
- **WHEN** Delay Time changes from 0 to 1 in one step
- **THEN** the echo's delay length changes by at most one sample per sample until it settles

### Requirement: A settled delay is exact
Once a delay-length control stops moving, its delay SHALL settle within two seconds to exactly the whole-sample length it has today, and with the controls static from the start the Space section's output SHALL be bit-identical to the output before this requirement. The first value set after the Space section is created or reset SHALL apply at once, without a glide.

#### Scenario: Echo settles exactly
- **WHEN** Delay Time changes from 0.25 to 0.75 and two seconds pass, then an impulse is fed with no modulation
- **THEN** the first repeat is a single sample at the 0.75 setting's delay, as before this requirement

#### Scenario: Static settings are unchanged
- **WHEN** an impulse and a noise burst are fed through each Space mode at its default settings and at size, tank and delay values 0.1 and 0.9, at 48 kHz, with the controls set before processing
- **THEN** each output is bit-identical to the output before this requirement

#### Scenario: No glide from the defaults on load
- **WHEN** a new Space section is set to Delay Time 0.9 before its first sample and an impulse is fed
- **THEN** the first repeat arrives at the 0.9 setting's delay
