## ADDED Requirements

### Requirement: The whole editor shows sharply at every device pixel ratio
At any device pixel ratio, the web app's editor canvas SHALL be at least as tall, in CSS pixels, as the editor's whole content at the page's width, so every panel and the keyboard can be reached by scrolling the page. The canvas's drawing buffer SHALL have the device's resolution (its CSS size times the device pixel ratio, within one pixel), so text and lines are not upscaled.

#### Scenario: HiDPI laptop
- **WHEN** `/play/` is opened in a 1600 × 1000 CSS px window at a device pixel ratio of 1.5
- **THEN** the canvas is as tall as at a ratio of 1 for the same window width, the keyboard strip at the bottom of the editor is drawn, and the canvas's drawing buffer is 1.5 times its CSS size

#### Scenario: Ratios 1, 1.5 and 2 agree
- **WHEN** the same window width is rendered at device pixel ratios 1, 1.5 and 2
- **THEN** the canvas's CSS height is the same in all three, within 2 CSS px

#### Scenario: The check runs in CI
- **WHEN** a change makes the canvas shorter than the editor's content at any of the three ratios
- **THEN** CI's `web` job fails, naming the ratio and both heights
