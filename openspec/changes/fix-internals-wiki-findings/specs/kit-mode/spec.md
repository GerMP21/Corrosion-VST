## MODIFIED Requirements

### Requirement: Kits are saved
A kit SHALL be saved with a preset and in the plugin's host state, and loading either SHALL restore every pad exactly. A preset saved in Kit mode SHALL always carry its `kit` field, empty when there are no pads; a preset saved in another Note Mode carries it only when the kit has pads. Loading a preset without a `kit` field SHALL leave no pads.

#### Scenario: Preset round trip
- **WHEN** a kit with pads on keys 36, 38 and 42 is saved as a preset and loaded back
- **THEN** the three pads are restored with identical sounds and tunes, and no other key has a pad

#### Scenario: Project round trip
- **WHEN** the host saves and restores the plugin state with a kit
- **THEN** the kit is restored exactly

#### Scenario: Empty kit round trip
- **WHEN** a preset is saved in Kit mode with no pads and loaded back
- **THEN** Note Mode is Kit and no key has a pad
