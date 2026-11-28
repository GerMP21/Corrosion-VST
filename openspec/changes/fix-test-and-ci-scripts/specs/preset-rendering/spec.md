## MODIFIED Requirements

### Requirement: The audition set reflects the plugin
The rendered audition files SHALL be produced by the offline renderer from the current factory presets. The preset render tool's default command SHALL find every factory preset, including those in subfolders, and SHALL write one file per preset with a name that is unique across folders.

#### Scenario: Regenerated audition set
- **WHEN** the preset render tool is run over the factory bank
- **THEN** it writes one WAV per preset whose samples equal the offline renderer's output for that preset, quantized to the file's bit depth

#### Scenario: Default command renders the bank
- **WHEN** the preset render tool is run with no `--preset-dir`
- **THEN** it renders all 214 factory presets, and two presets with the same file name in different folders produce two different output files
