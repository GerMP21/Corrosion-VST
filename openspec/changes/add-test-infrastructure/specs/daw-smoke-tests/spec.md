## Purpose
Defines the scripted DAW smoke test: what loading, rendering and reopening the plugin in REAPER must prove, and how the script reports a skip instead of a false pass.

## ADDED Requirements

### Requirement: The REAPER smoke test renders and restores the plugin
The REAPER smoke test SHALL load the CLAP plugin in REAPER on a virtual display, render a MIDI item through it to a WAV file, save the project with a non-default parameter value, reopen it, and check that the WAV is finite, not silent and under the output ceiling, and that the reopened project restored the parameter. Any failed step SHALL make the script exit non-zero, naming the step.

#### Scenario: A working build passes
- **WHEN** the script runs on a Linux machine with REAPER and a freshly built CLAP bundle
- **THEN** it renders a WAV whose RMS is above −60 dBFS and whose peak is at most the Analog Ceiling, reopens the project with the saved parameter value, and exits 0

#### Scenario: A silent plugin fails
- **WHEN** the bundle under test produces silence
- **THEN** the script exits non-zero and names the WAV check

### Requirement: A skipped DAW test never reports a pass
When REAPER or the plugin bundle is missing, the smoke test SHALL report that it was skipped and exit with a distinct code (77), and SHALL NOT print a pass.

#### Scenario: No REAPER installed
- **WHEN** the script runs on a machine without REAPER
- **THEN** it prints that the test was skipped and why, and exits 77
