## MODIFIED Requirements

### Requirement: Loop region is defined by start and end stages
With Forward looping and the gate held, reaching the loop end stage SHALL restart the loop start stage. Stages are numbered 0 Onset, 1 Attack, 2 Hold, 3 Decay, 4 Sustain, 5 Release. A start of 0 is treated as 1 and a start or end of 5 as 4, since Onset and Release are never looped. The same numbering, clamping and region apply to Ping-Pong looping.

#### Scenario: Default loop Decay to Sustain
- **WHEN** loop mode is Forward with start 3 and end 4 and the gate is held
- **THEN** the envelope repeats the Decay stage each time it reaches Sustain, producing output identical to the behavior before this change

#### Scenario: Custom loop end
- **WHEN** loop mode is Forward with start 1 (Attack) and end 3 (Decay) and the gate is held
- **THEN** on reaching Decay the envelope restarts Attack, and it never enters Decay or Sustain while the gate stays held

## ADDED Requirements

### Requirement: Ping-Pong plays the loop region forward and backward
With Ping-Pong looping and the gate held, the envelope SHALL play the stages from the loop start stage up to the stage before the loop end stage forward, then the same stages in reverse order, each played backward (from its end level to its start level, over its own time), then forward again, for as long as the gate stays held. The output SHALL not jump at either turn: the level at each turn equals the level just before it, within one sample's change.

#### Scenario: Default Ping-Pong is a triangle
- **WHEN** loop mode is Ping-Pong with start 3 and end 4, the peak is 1, the sustain level 0.5, the Decay 0.3 s with a linear curve, and the gate is held for 2 s
- **THEN** after the first Decay the level rises from 0.5 back to 1 over 0.3 s, then falls to 0.5 over 0.3 s, and repeats, never stepping between consecutive samples by more than one sample's slope

#### Scenario: Start and End are honoured
- **WHEN** loop mode is Ping-Pong with start 1 (Attack) and end 3 (Decay) and the gate is held
- **THEN** the envelope plays Attack, Hold, Hold backward, Attack backward, Attack, and so on, and never enters Decay or Sustain while the gate stays held

#### Scenario: Ping-Pong differs from Forward
- **WHEN** the same envelope is held for 2 s with Ping-Pong 3 → 4 and with Forward 3 → 4
- **THEN** the two outputs differ, and only the Forward output has a step larger than one sample's slope

#### Scenario: Degenerate Ping-Pong region
- **WHEN** loop mode is Ping-Pong with start 3 and end 1 and the gate is held
- **THEN** the envelope output is identical to the same settings with loop mode Off

#### Scenario: Release from a backward stage
- **WHEN** the gate is released while a stage is playing backward
- **THEN** the envelope enters Release from the current level and becomes idle within the release time plus one buffer
