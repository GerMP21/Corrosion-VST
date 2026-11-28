## Purpose
Defines how the editor's rendered appearance is checked against reference images in CI, so layout and styling regressions are caught without a person looking.

## ADDED Requirements

### Requirement: The editor is compared with reference images in CI
CI SHALL render the editor in the web app at a fixed window size and device pixel ratio for a fixed set of views (at least the default patch, Kit mode, a friction exciter's panel and the open preset browser), compare each with its reference image in the repository, and fail when the share of differing pixels exceeds the check's threshold. A failure SHALL name the view and keep the captured and difference images as job artifacts.

#### Scenario: An unchanged editor passes
- **WHEN** CI runs the check on a commit that doesn't change the editor
- **THEN** every view matches its reference within the threshold and the job passes

#### Scenario: A moved panel fails
- **WHEN** a change moves a panel by more than a few pixels
- **THEN** the check fails, names the affected views, and uploads their difference images

### Requirement: References change only on request
Reference images SHALL be updated only by an explicit command run by a developer, and the updated images SHALL be committed with the change that moved the editor.

#### Scenario: Intended editor change
- **WHEN** a change deliberately alters the editor and the developer runs the update command
- **THEN** the references are rewritten, the check passes again, and the new images appear in the change's diff
