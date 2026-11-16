# Spec Delta

## Purpose

The developer site at `/dev/` explains how Corrosion is built, from the signal chain to the test suite, so that contributors can find their way around the engine and the code.

## ADDED Requirements

### Requirement: Developer site path and language
The site SHALL serve the developer documentation at `/dev/`, in English only, with every page at `/dev/<page>/`. A request for `/dev` without the trailing slash SHALL redirect to `/dev/`. An unknown path under `/dev/` SHALL return a 404 page in the site's theme that links to `/dev/` and to `/`.

#### Scenario: Developer home
- **WHEN** a visitor opens `https://corrosion-vst.org/dev/`
- **THEN** the developer site's start page loads with its navigation

#### Scenario: Missing trailing slash
- **WHEN** a visitor opens `/dev`
- **THEN** the response redirects to `/dev/`

#### Scenario: Unknown developer page
- **WHEN** a visitor opens `/dev/no-such-page/`
- **THEN** the response is a 404 with a themed page that links to `/dev/` and `/`

#### Scenario: No translation
- **WHEN** a visitor opens any developer page
- **THEN** the page is in English and offers no language switcher

### Requirement: Architecture coverage
The developer site SHALL have a page or section for each part of the synth's architecture, from the repository layout and the signal chain to testing and contributing. A check SHALL fail when a part has no page that declares it.

#### Scenario: Every part has a home
- **WHEN** the developer site's check runs
- **THEN** it fails unless each of these parts has a page that declares it: the repository and crate layout, the signal chain from MIDI input to the limiter, voices and polyphony, the modal resonator, the exciters and the interaction bus, the material transformations, the post chain with quality modes and output protection, parameters and state with presets and migrations, the real-time safety contract, the sample-rate rule, the editor, the web app, the factory bank, testing and CI, and contributing

#### Scenario: Signal chain shown as a diagram
- **WHEN** a visitor opens the signal chain page
- **THEN** the page shows the stages in order, from MIDI input to the limiter, as a diagram that also has a text alternative for screen readers

### Requirement: Source references
Every developer page SHALL name the repository files or directories it describes. Each named path SHALL exist in the repository, and each top-level module under `src/` SHALL be named by at least one page. A check SHALL fail otherwise, naming the page and the path.

#### Scenario: A described file is moved
- **WHEN** a file a developer page names is renamed or deleted and the page is not updated
- **THEN** the developer site's check fails, naming the page and the missing path

#### Scenario: A new module nobody documents
- **WHEN** a new top-level module is added under `src/` and no developer page names it
- **THEN** the developer site's check fails, naming the module

#### Scenario: Source links
- **WHEN** a visitor reads a developer page
- **THEN** the page lists the files it describes, each linking to that file in the source repository

### Requirement: Look and navigation
The developer site SHALL be dark-only and use the shared site tokens. Its header SHALL link to the home page, the web app and the Wiki. It SHALL have a sidebar listing every page in reading order.

#### Scenario: Back to the rest of the site
- **WHEN** a visitor is on any developer page
- **THEN** the header shows links to `/`, `/play/` and `/wiki/`, and each resolves

#### Scenario: Same colours
- **WHEN** a visitor moves between `/wiki/` and `/dev/`
- **THEN** both use the same background, text and accent colours from the tokens

### Requirement: Developer search
The developer site SHALL offer full-text search over its own pages, working offline from the built site with no external service.

#### Scenario: Find a term
- **WHEN** a visitor searches the developer site for "interaction bus"
- **THEN** the results include the exciters page that describes it

### Requirement: Build and CI
The developer site SHALL be built into `dist/dev/` by the site's build script and deployed with the rest of the site. CI SHALL run its check (coverage and source references) and a full build in every CI run. A broken internal link SHALL fail the build.

#### Scenario: Broken internal link
- **WHEN** a developer page links to a developer page that doesn't exist
- **THEN** the developer site's build fails, naming the link

#### Scenario: Built with the site
- **WHEN** `web/scripts/build.sh` runs
- **THEN** `dist/dev/index.html` exists next to `dist/index.html`, `dist/play/` and `dist/wiki/`
