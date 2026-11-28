# Spec Delta

## Purpose
Defines how a friction-family note (Bow, Stiff Point Scrape, Heavy Grinding, Corrugated Drag, Tension Rise) lets go of the object at note-off: how loud the object's spring-back may be, how the Release Snap control sets it, and how state saved before the control existed keeps its sound.

## ADDED Requirements

### Requirement: Release Snap bounds the released note
At Release Snap 0, a released friction note SHALL be no louder after note-off than while it was held: the RMS of the first 0.5 s after note-off, measured at the plugin output, SHALL NOT exceed the RMS of the last held second.

#### Scenario: Corrugated Drag on Plate eases back
- **WHEN** a Corrugated Drag note on Plate, C4 at velocity 100, default controls except Release Snap 0, is held 1 s and released
- **THEN** the output RMS over the 0.5 s after note-off is at most the output RMS of the held second

#### Scenario: Every friction exciter on every object
- **WHEN** each of Bow, Stiff Point Scrape and Corrugated Drag plays each object at C4, velocity 100, Release Snap 0, held 1 s then released
- **THEN** no combination's 0.5 s tail is louder than its held second

### Requirement: Release Snap 1 keeps today's release
At Release Snap 1, every note SHALL render bit-identical to the same note before Release Snap existed, for every exciter.

#### Scenario: Snap kept
- **WHEN** each exciter 1–16 plays Pipe and Plate at C4, velocity 100, held 1 s then released, at Release Snap 1
- **THEN** the 48 kHz render hash equals the hash of the same note rendered without Release Snap

### Requirement: Release Snap moves the snap monotonically
Between 0 and 1, raising Release Snap SHALL NOT make a released friction note's tail quieter, and its two ends SHALL differ by at least the control-response floor on the default patch's friction notes.

#### Scenario: Sweep on Bow
- **WHEN** a Bow note on Taut Cable, C4 at velocity 100, is released at Release Snap 0, 0.25, 0.5, 0.75 and 1
- **THEN** the 0.5 s tail RMS never falls as Release Snap rises, and the tails at 0 and 1 differ by at least 1 dB

### Requirement: Release Snap leaves other families alone
Release Snap SHALL NOT change any note of the Hit or Specialty families.

#### Scenario: Struck and specialty notes
- **WHEN** Hand Strike and Pneumatic Jet notes are rendered at Release Snap 0 and at 1
- **THEN** each pair renders bit-identical

### Requirement: Older state keeps its snap
A user preset or saved project that has no Release Snap SHALL load with the value that reproduces the release it had before the control existed, through both the preset loader and the host's state restore.

#### Scenario: Old preset
- **WHEN** a preset saved before Release Snap existed, with a Bow on Taut Cable, is loaded
- **THEN** Release Snap is 1 and its released note renders as it did before

#### Scenario: Old project restored by the host
- **WHEN** a saved project whose parameter state has no `release_snap` is restored
- **THEN** Release Snap is 1, whatever the parameter's default is
