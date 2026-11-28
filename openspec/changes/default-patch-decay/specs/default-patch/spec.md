# Spec Delta

## Purpose
Defines how the patch a new plugin instance starts with sounds: a struck note that decays from its strike rather than holding a loud plateau, and state saved under older defaults that keeps its sound.

## ADDED Requirements

### Requirement: The default note decays from its strike
A note played on the default patch SHALL lose level from shortly after its strike: rendered at C4, velocity 100, held 4 s, its 100 ms RMS one second after the onset SHALL be at least 6 dB below its loudest 100 ms RMS, and it SHALL NOT hold any 1 s span whose level varies by less than 1 dB in its first 3 s.

#### Scenario: Default patch held
- **WHEN** a new instance plays C4 at velocity 100 with every control at its default, held for 4 s
- **THEN** the 100 ms RMS at 1 s after the onset is at least 6 dB below the loudest 100 ms RMS, and no 1 s window in the first 3 s stays within 1 dB

#### Scenario: Default patch across the keyboard
- **WHEN** the default patch plays C2, C3 and C5 at velocity 100, held 4 s
- **THEN** each note meets the same bounds

### Requirement: Saved state keeps its old defaults
A user preset or saved project that does not store a parameter whose default this capability changed SHALL load with that parameter's previous default, through both the preset loader and the host's state restore, so it renders as it did.

#### Scenario: Old preset without the field
- **WHEN** a preset file saved before the default changed, which omits the changed parameter, is loaded
- **THEN** the parameter takes its previous default and a note renders bit-identical to the same preset loaded before the change

#### Scenario: Old project
- **WHEN** a saved project whose parameter state lacks the changed parameter is restored by the host
- **THEN** the parameter takes its previous default
