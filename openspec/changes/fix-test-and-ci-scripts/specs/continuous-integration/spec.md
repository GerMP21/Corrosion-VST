## MODIFIED Requirements

### Requirement: The nightly release
After a run on `main` (a push or a manual run) in which every checks, bundle, validation, web and guide job passed, the CI SHALL replace the `nightly` pre-release with that commit's six zips, tagged at that commit. Runs on other branches and on pull requests SHALL NOT publish. A failing run SHALL leave the existing nightly unchanged. Two publishing runs SHALL NOT overlap.

#### Scenario: A green run on main replaces the nightly
- **WHEN** a push to `main` passes every job
- **THEN** the `nightly` pre-release points at that commit and holds exactly its six zips, and the previous nightly's assets are gone

#### Scenario: dev and pull requests never publish
- **WHEN** a run on `dev` or on a pull request passes every job
- **THEN** the `nightly` release is unchanged

#### Scenario: A red run keeps the old nightly
- **WHEN** a run on `main` fails any job
- **THEN** the `nightly` release still holds the previous green commit's zips

#### Scenario: A broken web app blocks the nightly
- **WHEN** a run on `main` passes the checks, bundle and validation jobs but fails the web or guide job
- **THEN** the nightly job does not run and the `nightly` release is unchanged

## ADDED Requirements

### Requirement: The local verification script says what it checks
`scripts/verify-local.sh` SHALL run, on Linux, every step of CI's `checks` job (formatting, clippy, the workspace tests, the GUI library tests, the separate real-time safety test, docs with warnings denied, the benchmark compile, the offline renderer smoke and its WAV check) plus the third-party notices check and a Linux bundle, and its header SHALL list the CI jobs and platforms it does not cover.

#### Scenario: A docs warning fails locally
- **WHEN** a change adds a `cargo doc` warning and `scripts/verify-local.sh` is run
- **THEN** the script fails at its docs step

#### Scenario: Skipped jobs are named
- **WHEN** a developer reads the script's header
- **THEN** it names the validators, the web job, the guide job and the Windows and macOS lanes as not run locally
