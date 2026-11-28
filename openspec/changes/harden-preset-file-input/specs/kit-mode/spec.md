# Spec Delta

## ADDED Requirements

### Requirement: Pad levels from files are bounded
A pad's level SHALL be limited to −60…+72 dB whenever a kit is loaded from a preset file, a saved project or the web app, so no file can give a pad an infinite or zero-by-overflow gain. Levels inside that range SHALL load unchanged.

#### Scenario: Absurd level in a file
- **WHEN** a preset whose kit has a pad on key 36 with `"level_db": 800` is loaded and key 36 is played
- **THEN** the pad plays with a +72 dB level (a finite gain), and saving the preset again writes 72

#### Scenario: Factory kits unchanged
- **WHEN** each factory drum kit is loaded
- **THEN** every pad's level and gain are identical to before this change

### Requirement: A pad that can't be built leaves only its key empty
When a stored pad's values can't be turned into a sound, loading the kit SHALL NOT panic: every other pad SHALL load, the bad pad's key SHALL be empty, and the plugin SHALL log which key was skipped. The same SHALL hold when the editor loads or auditions that pad.

#### Scenario: A value the parser rejects
- **WHEN** a preset's kit has pads on keys 36 and 38, and key 38's values include a number that overflows to infinity (for example `1e40`)
- **THEN** the preset loads without panicking, key 36 plays its pad, key 38 is silent, and one log line names key 38
