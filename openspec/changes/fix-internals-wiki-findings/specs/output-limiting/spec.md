## ADDED Requirements

### Requirement: Switching the limiter mode never replays old audio
After Limiter Mode changes, the output SHALL contain no audio older than the Lookahead limiter's delay, and the switch SHALL NOT insert a gap of silence.

#### Scenario: Hard to Lookahead after a pause
- **WHEN** a note plays through Lookahead, the mode is switched to Hard for 1 s of different audio, then back to Lookahead
- **THEN** none of the first 48 samples after switching back contain audio from the first Lookahead period

#### Scenario: No dropout on the switch
- **WHEN** Limiter Mode is switched from Hard to Lookahead while a sustained note plays
- **THEN** the output's RMS over the first 2 ms after the switch is within 3 dB of the RMS over the 2 ms before it
