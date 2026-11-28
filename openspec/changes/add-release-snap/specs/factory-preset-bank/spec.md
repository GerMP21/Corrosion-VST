# Spec Delta

## ADDED Requirements

### Requirement: Released notes stay near the held note
Every factory preset not in Kit mode SHALL, rendered through the plugin's output path, keep the RMS of the first 0.5 s after note-off within 6 dB above the RMS of its last held second, at velocity 100 and at velocity 127.

#### Scenario: Bowed preset released
- **WHEN** Bowed Cable Sub renders a middle-C note held 2 s and then released, at velocity 100
- **THEN** the output RMS over the 0.5 s after note-off is no more than 6 dB above the RMS of the second before it

#### Scenario: Whole bank
- **WHEN** each factory preset not in Kit mode renders a held then released middle-C note at velocity 100 and at velocity 127
- **THEN** every tail is within 6 dB above its held second, and the bank test names any preset that is not
