# Spec Delta

## ADDED Requirements

### Requirement: Changing Space Mode neither cuts nor replays a tail
A Space Mode change SHALL fade the outgoing mode's wet signal out and the incoming mode's wet signal in over about 20 ms, with no single-sample step, and a mode that is selected again SHALL start from silence, never from the tail it held when it was left.

#### Scenario: Factory re-entered after silence
- **WHEN** Factory plays with Space Amount 1 on a 50 ms burst followed by 50 ms of silence, Space Mode is switched to Off for 1 s, then back to Factory with silent input
- **THEN** the output over the first 100 ms after the switch back is silent (RMS below 1e-6)

#### Scenario: Switching away from a ringing tail
- **WHEN** a Spring tail is ringing at Space Amount 1 and Space Mode is switched to Off
- **THEN** the wet signal reaches 0 within 25 ms, and no sample-to-sample change of the output in that time exceeds the largest change in the 25 ms before the switch

#### Scenario: Echo does not replay old repeats
- **WHEN** Echo is fed a burst, switched to Factory for 2 s with silent input, then switched back to Echo with silent input
- **THEN** the output after the switch back is silent once Factory's fade-out is over

### Requirement: Space Amount moves without steps
Space Amount SHALL reach the Space stage through a per-sample smoother with a time constant of about 20 ms. The first value set after the chain is created or reset SHALL apply at once.

#### Scenario: Automated Space Amount
- **WHEN** Space Amount steps from 0 to 1 between two 512-sample blocks while Factory processes a steady tone
- **THEN** the wet contribution rises over about 20 ms, with no single-sample step larger than one twentieth of the full change

#### Scenario: Static amount is unchanged
- **WHEN** a preset with a fixed Space Amount is rendered
- **THEN** the output is bit-identical to the same render before this change, apart from the first 20 ms after a mode change
