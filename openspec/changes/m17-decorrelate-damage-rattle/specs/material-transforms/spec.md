## ADDED Requirements

### Requirement: Damage's rattle spreads with the stereo image
In the stereo render path, the damage rattle SHALL be independent noise on each channel, at the same per-channel level as a single shared rattle, so it spreads across the image instead of sitting in the centre. Damage 0 SHALL add no rattle at all.

#### Scenario: Left and right rattles are independent
- **WHEN** a sustained, damaged note (Damage 6, Bow on a Plate, Width 1.5) is rendered in stereo and the rattle is isolated by subtracting the same render with the rattle disabled
- **THEN** the normalized correlation between the left and right rattle signals over 1 s is below 0.1

#### Scenario: Per-channel level is kept
- **WHEN** the same damaged note is rendered with independent and with shared rattle noise
- **THEN** each channel's rattle RMS is within 1 dB of the shared rattle's RMS

#### Scenario: Undamaged notes are unchanged
- **WHEN** any note is rendered at Damage 0
- **THEN** the output is bit-identical to the same render before this change
