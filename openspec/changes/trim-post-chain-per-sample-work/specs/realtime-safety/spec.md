## ADDED Requirements

### Requirement: The post chain is configured when something changes
The plugin SHALL reconfigure the post chain's sample-rate-dependent state (smoothing coefficients, the body's mode tables, the Space delays, the filter's coefficients) when the host sample rate changes, not on every block. With the rate and every post-chain control unchanged, the output SHALL be bit-identical to reconfiguring the chain on every block.

#### Scenario: Steady settings are bit-identical
- **WHEN** a preset with Space, BODY above 0, master drive and Component Tolerance 0 is rendered for 16 notes in 512-sample blocks at 48 kHz, once with the chain reconfigured every block and once without
- **THEN** the two outputs are bit-identical

#### Scenario: A rate change still takes effect
- **WHEN** the host rate changes from 48 kHz to 96 kHz between two blocks
- **THEN** from the next block on the post chain uses the same rate-dependent coefficients as a chain configured at 96 kHz from new, and after a reset both produce bit-identical output from the same input

#### Scenario: The steady chain is no slower
- **WHEN** the post-chain and plugin-path benchmarks run five times, interleaved with a build of the previous code
- **THEN** the median of the per-run ratios (new ÷ old) is below 1.0 for the post chain and at most 1.05 for the plugin path
