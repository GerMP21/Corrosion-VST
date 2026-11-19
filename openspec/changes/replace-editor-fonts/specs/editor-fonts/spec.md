# Spec Delta

## Purpose

Sets the fonts the plugin's editor and the web app draw with, so every character on screen has a glyph, text keeps the size the layout was tuned for, and only fonts with known, compatible licenses ship in Corrosion's builds.

## ADDED Requirements

### Requirement: Interface and brand fonts
The editor SHALL draw its interface text in Inter and the header's brand block ("CORROSION" and its two subtitle lines) in IBM Plex Sans, in the plugin and in the web app alike.

#### Scenario: Interface text
- **WHEN** the editor draws a control's label, a value, a menu entry or a tooltip
- **THEN** the text is set in Inter

#### Scenario: Brand block
- **WHEN** the editor draws the header's brand block
- **THEN** "CORROSION", "INDUSTRIAL RESONATOR" and "MODEL: CR-89X" are set in IBM Plex Sans

### Requirement: Every drawn character has a glyph
Every character the editor draws SHALL have a glyph in its fonts: its labels, values and units, and the characters egui's own widgets draw as text for the editor, such as the submenu arrow. No character SHALL fall back to a replacement box or question mark.

#### Scenario: Preset browser folders
- **WHEN** the preset browser opens a folder list
- **THEN** each folder entry shows its submenu arrow as a triangle, not a replacement glyph

#### Scenario: Value units
- **WHEN** a control shows a value with a unit such as "×1.41" or "+6.0 dB"
- **THEN** every character of it is drawn from the editor's fonts

### Requirement: Text keeps the layout's metrics
At every UI scale, the interface font's capitals SHALL be as tall as the previous font's at the same requested size, and its rows SHALL be no more than 1 % taller, so the layout's panels don't grow.

#### Scenario: Same panels
- **WHEN** the editor is drawn at any UI scale
- **THEN** its content still fits the window's width and its header faders keep their room, as the layout tests require

### Requirement: Only the editor's fonts ship
The plugin bundles and the web app SHALL embed no font other than the editor's own. egui's default fonts SHALL NOT be built in. A build check SHALL fail when they reach any shipped build.

#### Scenario: A dependency turns the default fonts back on
- **WHEN** a dependency change makes egui's default fonts part of a plugin or web app build
- **THEN** the third-party notices check fails, naming the font crate

#### Scenario: Notices
- **WHEN** a reader opens the third-party notices
- **THEN** they find Inter's and IBM Plex Sans's licenses and copyright lines, and no entry for egui's default fonts
