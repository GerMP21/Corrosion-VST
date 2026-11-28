## Purpose
Defines how benchmark results are saved per machine and compared later, so a CPU regression is flagged by numbers rather than noticed by chance.

## ADDED Requirements

### Requirement: Benchmarks can be saved and compared per machine
A baseline command SHALL save, for the current machine, each benchmark's median over 5 runs; a compare command SHALL run the benchmarks again the same way and report each benchmark's ratio to its saved median, flagging those slower than a threshold set above the machine's measured run-to-run noise. Baselines SHALL be stored outside the repository and keyed by machine, and comparing against a different machine's baseline SHALL be refused.

#### Scenario: A regression is flagged
- **WHEN** a change makes `voice_manager_sixteen_voices` 30 % slower and the compare command is run against a baseline saved before it on the same machine
- **THEN** the report flags that benchmark with its ratio and the command exits non-zero

#### Scenario: Noise is not flagged
- **WHEN** the compare command is run twice on an unchanged tree on the same machine
- **THEN** no benchmark is flagged

#### Scenario: Another machine's baseline
- **WHEN** the compare command finds only a baseline saved on a different machine
- **THEN** it refuses to compare and says which machine the baseline belongs to
