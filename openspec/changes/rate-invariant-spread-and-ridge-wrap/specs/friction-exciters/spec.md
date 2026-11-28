# Spec Delta

## Purpose
Defines how the friction-family contact models (Bow, Stiff Point Scrape, Heavy Grinding, Corrugated Drag, Tension Rise) behave over the whole life of a held note, so long notes and drones keep the character of their first second.

## ADDED Requirements

### Requirement: Corrugated Drag's ridge rate holds on long notes
Corrugated Drag SHALL produce its ridge bumps at the same rate, within 0.1 %, however long the note has been held, for every Drag Speed and Ridge Spacing in their host ranges.

#### Scenario: Ten-minute drone
- **WHEN** the Corrugated Drag exciter runs at the default patch's drag speed and Ridge Spacing against a still resonator for 10 minutes at 48 kHz
- **THEN** the bump rate measured over the last second is within 0.1 % of the rate measured over the first second

#### Scenario: Fastest drag on the finest ridges
- **WHEN** the same test runs at Drag Speed 2.5 and Ridge Spacing 0.01
- **THEN** the exciter's force stays finite and its bump count over the last second is within 0.1 % of the first second's
