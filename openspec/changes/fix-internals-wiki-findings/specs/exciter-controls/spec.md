## ADDED Requirements

### Requirement: Brush Wire Stiffness changes the brush's tone
Raising Brush Wire Stiffness SHALL make a Wire Brush note brighter (more energy above 2 kHz relative to its total), and SHALL change the note's overall level by no more than 6 dB across the control's range.

#### Scenario: Stiffer wires sound brighter at a similar level
- **WHEN** a Wire Brush note on Plate, C3 at velocity 100, is rendered at Brush Wire Stiffness 0 and at 1
- **THEN** the note at 1 has a higher share of its energy above 2 kHz, and the two notes' RMS levels differ by at most 6 dB

### Requirement: Drop Envelope Spread acts over its whole range
Every value of Drop Envelope Spread the host offers SHALL set the Metal Chain's drop spread to that value: no part of the range is clamped to a neighbouring value.

#### Scenario: The bottom of the range acts
- **WHEN** a Metal Chain note is rendered with Drop Envelope Spread 40 ms and 50 ms
- **THEN** the two outputs differ, and the link impacts of the 40 ms note all land within 40 ms of the first one (before random offsets)
