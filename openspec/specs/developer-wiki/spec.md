# developer-wiki Specification

## Purpose
The Internals wiki is the repository's GitHub Wiki. It explains in detail how every part of Corrosion works inside, from the signal chain to CI. Contributors can go from a behaviour to the file that implements it, and the wiki stays true as the code changes.

## Requirements

### Requirement: Pages are kept in the repository
The Internals wiki's pages SHALL live in one directory of the main repository, one Markdown file per wiki page, in English only. Changes to them SHALL go through the same commits and pull requests as the code. The published wiki SHALL be produced from that directory and from nowhere else.

#### Scenario: A page is edited
- **WHEN** a contributor wants to change a wiki page
- **THEN** they edit its file in the repository's wiki directory, in a commit or pull request like any code change

#### Scenario: English only
- **WHEN** a reader opens any Internals page
- **THEN** the page is in English, with no Spanish twin required

### Requirement: Publishing to the GitHub Wiki
Every push to `main` SHALL publish the wiki directory to the repository's GitHub Wiki, and a maintainer SHALL be able to start the same publish by hand. After a publish, the GitHub Wiki SHALL hold the pages of that commit and nothing else. A run with nothing to change SHALL succeed without a new wiki commit.

#### Scenario: Merge to main
- **WHEN** a commit that changes a page in the wiki directory reaches `main`
- **THEN** the GitHub Wiki shows the changed page once the publish run finishes

#### Scenario: Page removed
- **WHEN** a page file is deleted from the wiki directory and the change reaches `main`
- **THEN** the page is gone from the GitHub Wiki after the publish

#### Scenario: Nothing changed
- **WHEN** the publish runs and the GitHub Wiki already matches the wiki directory
- **THEN** the run succeeds and makes no wiki commit

#### Scenario: Edit in the GitHub web editor
- **WHEN** someone edits a page in GitHub's wiki editor and a later publish runs
- **THEN** the edit is replaced by the repository's version, as the footer on every page warns

### Requirement: Coverage of the whole system
The Internals wiki SHALL have at least one page for each part of the system: the repository layout and signal chain; the plugin shell, parameters, voices, expression and kit mode; the resonator, objects, exciters, interaction bus, envelopes, transformations and DSP utilities; the post chain; presets, the factory bank and the randomizer; the editor, the web app and the site tooling; and real-time safety, testing, benchmarks, building and CI.

#### Scenario: Every part has a page
- **WHEN** a reader looks for how any part listed above works
- **THEN** the sidebar leads to a page that describes it

#### Scenario: Signal chain diagram
- **WHEN** a reader opens the signal chain page
- **THEN** the page shows the stages from MIDI input to the limiter as a diagram rendered by GitHub, with the same stages also listed in text

### Requirement: Source-file references
Every Internals page except the home page, the glossary, the sidebar and the footer SHALL end with a "Source files" section. It lists, as links to the repository's `main` branch, the files and directories the page describes. Every listed path SHALL exist in the repository.

#### Scenario: A described file is moved
- **WHEN** a file named in a page's "Source files" section is renamed or deleted and the page isn't updated
- **THEN** the test suite fails, naming the page and the missing path

#### Scenario: Source links
- **WHEN** a reader opens an Internals page
- **THEN** its "Source files" section links each listed path to that file or directory on GitHub

### Requirement: Every source file is described
Every non-test Rust file under `src/` SHALL be named, by its own path, in the "Source files" section of at least one page. Every top-level item of `web/`, `scripts/`, `tests/`, `benches/` and `.github/workflows/` SHALL be named the same way, by its own path or by a path under it.

#### Scenario: A new module nobody documents
- **WHEN** a new Rust file is added under `src/` and no page names it
- **THEN** the test suite fails, naming the file

#### Scenario: A new script
- **WHEN** a new file is added to `scripts/` and no page names it
- **THEN** the test suite fails, naming the file

#### Scenario: Test files are exempt
- **WHEN** a Rust file under `src/` holds only tests (`tests.rs`, a name ending in `_tests.rs`, or a file under a `tests` directory)
- **THEN** the coverage check doesn't require a page to name it

### Requirement: Links between pages resolve
Every link from an Internals page to another Internals page SHALL point to a page that exists. A link to a heading SHALL point to a heading on that page. The sidebar SHALL link every page.

#### Scenario: Broken page link
- **WHEN** a page links to an Internals page that doesn't exist, or to a heading the target page doesn't have
- **THEN** the test suite fails, naming the page and the link

#### Scenario: Page missing from the sidebar
- **WHEN** a page exists in the wiki directory but the sidebar doesn't link it
- **THEN** the test suite fails, naming the page

### Requirement: The wiki follows the code
Every change to the repository SHALL update, in the same change, every Internals page whose description it makes untrue: code, build, tests, CI and tooling, not only sound. A new file SHALL be named by a page in the same change. The rule SHALL be stated in the project's contributor guidance, for both maintainer and contributor sessions, and in the pull request checklist.

#### Scenario: A change alters a described behaviour
- **WHEN** a change alters how a part described by an Internals page works
- **THEN** the same commit or pull request updates that page

#### Scenario: Contributor checklist
- **WHEN** a contributor opens a pull request
- **THEN** the template's checklist asks whether every Internals page that describes the changed files was updated
