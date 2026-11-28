## ADDED Requirements

### Requirement: Lowering Quality thins held notes without a click
When the quality mode is lowered, every sounding voice SHALL stop computing the modes beyond the new mode's count within 50 ms, fading them out without a discontinuity. Raising the quality mode SHALL leave sounding voices unchanged; notes started after the switch use the new count.

#### Scenario: Eco relieves a held Render drone
- **WHEN** a Drone note is held at Render quality and the quality is switched to Eco
- **THEN** within 50 ms the voice computes no more modes than an Eco note of the same object and pitch would

#### Scenario: No click on the switch
- **WHEN** a struck Pipe note rings at Render quality and the quality is switched to Eco 200 ms after the strike
- **THEN** the largest sample-to-sample step in the 50 ms after the switch is no larger than the largest step in the 50 ms before it

#### Scenario: Raising quality leaves held notes alone
- **WHEN** a note is held at Eco and the quality is switched to Render
- **THEN** the held note's output is bit-identical to the same note with no switch, and a note started after the switch has the Render mode count

#### Scenario: Fixed quality is unchanged
- **WHEN** a render plays notes at one quality mode throughout
- **THEN** the output is bit-identical to the same render before this change

## MODIFIED Requirements

### Requirement: Retuned notes keep every mode they can reach
A mode kept by the bank SHALL stay in the bank for the whole note, so bending or retuning a note down never removes modes that come into the audible band. The only exception is a lowered quality mode, which may remove the modes beyond the new mode count (see "Lowering Quality thins held notes without a click").

#### Scenario: Bend down two octaves
- **WHEN** a held note is retuned down 24 semitones
- **THEN** the bank holds the same modes as at note-on
