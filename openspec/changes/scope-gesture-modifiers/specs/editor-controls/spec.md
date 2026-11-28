# Spec Delta

## ADDED Requirements

### Requirement: Envelope-specific controls show only where they act
The editor SHALL draw the Gesture Modifiers panel (Env Amount, Vel To Peak, Global Time, Vel To Level, Vel To Time, Curve) only while the selected exciter belongs to the friction family, whose envelope those controls shape. The six parameters SHALL stay exposed to the host and SHALL keep acting on friction notes.

#### Scenario: Friction exciter selected
- **WHEN** the exciter is Bow, Stiff Point Scrape, Heavy Grinding, Corrugated Drag or Tension Rise
- **THEN** the Gesture Modifiers panel and its six faders are drawn in the exciter column

#### Scenario: Hit or Specialty exciter selected
- **WHEN** the exciter is any of the other eleven models, Hand Strike included
- **THEN** no Gesture Modifiers panel or fader is drawn, and the exciter column ends after its envelope panel

#### Scenario: Automation of a hidden control
- **WHEN** the host automates Curve while Hand Strike is selected, and the user then switches the exciter to Bow
- **THEN** the Curve fader appears showing the automated value, and a Bow note uses it
