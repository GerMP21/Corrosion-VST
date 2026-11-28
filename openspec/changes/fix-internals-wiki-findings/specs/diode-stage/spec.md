## ADDED Requirements

### Requirement: The web app's dot shows the diode working
The web app's limiter dot SHALL follow the diode stage's gain reduction the same way the plugin's does: the engine SHALL report the reduction after every block it renders.

#### Scenario: A loud note lights the web app's dot
- **WHEN** the web engine renders a block that drives the diode stage into 6 dB of reduction
- **THEN** its next report carries a reduction of at least 6 dB, and the editor draws the dot at full brightness

#### Scenario: A quiet note leaves it at rest
- **WHEN** the web engine renders a block that stays under the diode's knee
- **THEN** its next report carries a reduction under 0.1 dB
