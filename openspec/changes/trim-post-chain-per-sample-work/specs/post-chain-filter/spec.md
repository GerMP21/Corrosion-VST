## ADDED Requirements

### Requirement: Component drift is refreshed at a control rate
While Component Tolerance is above 0 and the cutoff and resonance are not moving, the filter MAY refresh its drifted coefficients every 16 samples instead of every sample. Its level over any 50 ms window SHALL stay within 0.1 dB of a per-sample refresh fed the same input and the same drift.

#### Scenario: Control-rate drift tracks the per-sample drift
- **WHEN** white noise at 0.25 RMS runs through the filter at cutoff 2 kHz, resonance 0.5 and Component Tolerance 1 for ten seconds at 48 kHz, with the drift refreshed every 16 samples and, in a reference, every sample, from the same noise seed
- **THEN** every 50 ms window's level differs by at most 0.1 dB between the two

### Requirement: Tolerance back to 0 restores the exact cutoff
When Component Tolerance returns to 0, the filter SHALL apply exactly the set cutoff again within one block, without waiting for the cutoff or resonance to move.

#### Scenario: Tolerance back to 0 restores the cutoff
- **WHEN** the filter runs with Component Tolerance 1 for one second, then Component Tolerance is set to 0 with cutoff and resonance unchanged
- **THEN** within one block the filter applies exactly the coefficients of a filter set to the same cutoff and resonance that never had tolerance, and after a reset both produce bit-identical output from the same input

#### Scenario: No tolerance is unchanged
- **WHEN** Component Tolerance is 0
- **THEN** the filter's output is bit-identical to the filter before this requirement
