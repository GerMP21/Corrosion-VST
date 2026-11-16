# Tasks

## 1. Scaffold the developer site

- [ ] 1.1 Create `web/devdocs/`, an Astro Starlight project with the Wiki's exact pins (`astro`, `@astrojs/starlight`, `starlight-links-validator`, `js-yaml`), `base: '/dev'`, `outDir: '../dist/dev'`, `trailingSlash: 'always'`, English only (no `locales`), `pagefind: true` and the links validator with `errorOnLocalLinks: true`; verify `npm ci && npm run build` writes `web/dist/dev/index.html` from a placeholder start page
- [ ] 1.2 Copy `DarkThemeProvider.astro` and `NoThemeSelect.astro` from the Wiki, add `devdocs.css` mapping Starlight's variables to `web/theme/tokens.css`, a `SiteLinks.astro` with Home, Play and Wiki, and a `src/site.mjs` holding the repository URL; verify the built start page is dark, has no theme or language switcher, and shows the three header links
- [ ] 1.3 Add the dev site's own `404.md`, linking to `/dev/` and `/`; verify `node web/scripts/serve.cjs` answers `/dev/no-such-page/` with it

## 2. The check, tests first

- [ ] 2.1 Write `scripts/check.test.mjs` cases for each failure in design Decision 3 (a missing part, an unknown part, a missing `sources:` path, an uncovered top-level `src/` entry) and for the copied components drifting from the Wiki's; verify they fail before `check.mjs` exists
- [ ] 2.2 Implement `scripts/check.mjs` (`npm run check`) with the part list, `sources:` existence, `src/` coverage and the byte-for-byte comparison of the two copied components, each failure naming the page and item; verify `npm test` passes and the check fails on the placeholder site, naming every part and `src/` entry still missing
- [ ] 2.3 Add the `<Sources>` component, which renders a page's `sources:` as links to the repository (`blob/main/` for files, `tree/main/` for directories); verify the links on a built test page resolve on GitHub

## 3. Diagrams

- [ ] 3.1 Copy `Flow.astro` from the Wiki into `web/devdocs/src/components/`; verify a test page's flow wraps at 375 px without horizontal scroll and exposes its `aria-label`

## 4. Write the pages

Each task writes its pages with their `parts:` and `sources:`, in the voice of design Decision 7, checks every claim against the code, and verifies that `npm run check` no longer reports that page's parts or `src/` entries and that `npm run build` passes.

- [ ] 4.1 Overview (`layout`): what Corrosion is technically, the two Cargo workspaces, the crate and directory map, how a note travels in one paragraph, and where to start reading
- [ ] 4.2 Signal chain (`signal-chain`): MIDI to the limiter as a `Flow` diagram plus the per-sample voice loop, with what each stage does and why it sits there
- [ ] 4.3 Voices and polyphony (`voices`): the voice lifecycle, stealing tiers, sustain and panic, per-note expression, kit pads, and the editor's note ring
- [ ] 4.4 The modal resonator (`resonator`): modes and their parameters, building and retuning the bank in place, pruning unsoundable modes, the per-sample loop's data layout, and level normalization
- [ ] 4.5 Exciters and the interaction bus (`exciters`): the 16 models in families, the two-way bus, drive levels and the governor, implicit contact, noise and sub-sample events
- [ ] 4.6 Material transformations (`transformations`): Rust, Damage and Heat, what each does to the modes, and why the fundamental is kept
- [ ] 4.7 Post chain and output (`post-chain`): the stages and their routing as an SVG diagram, quality modes and oversampling, the diode stage, output gain, the limiter, and reported latency
- [ ] 4.8 Parameters, state and presets (`parameters-and-state`): the parameter table and its constructors, host IDs, the preset schema, both migration paths with the Position Envelope example, user presets and the randomizer
- [ ] 4.9 Real-time safety (`realtime-safety`) and Sample rate (`sample-rate`): the no-allocation contract and how `tests/no_alloc.rs` proves it, the hot-loop rules, and the 48 kHz tuning rule with `rate`
- [ ] 4.10 The editor (`editor`) and The web app (`web-app`): egui and baseview, the control tables and inline help, the keyboard, the wasm builds, the AudioWorklet, the nih-plug patch and the bridge
- [ ] 4.11 The factory bank (`factory-bank`): presets built in code, the seeder and levelling, the drum kits, and the bank tests
- [ ] 4.12 Testing, CI and benchmarks (`testing`) and Contributing (`contributing`): the test lanes and what each covers, golden fixtures, proving "unchanged", the CI jobs, benchmark noise and medians, and the OpenSpec flow with links to `CONTRIBUTING.md`
- [ ] 4.13 Set the sidebar order from the design's table; verify `npm run check` passes with no missing part or uncovered `src/` entry and the search finds "interaction bus" on the exciters page

## 5. Wire it into the site

- [ ] 5.1 Add `web/devdocs` to `web/scripts/build.sh` after the Wiki; verify a full build has `dist/dev/index.html` next to `dist/index.html`, `dist/play/` and `dist/wiki/`
- [ ] 5.2 Link `/dev/` from the home page's header or footer; verify it at 1280 × 720 and at 375 px with no horizontal scroll
- [ ] 5.3 Extend `web/scripts/site.cjs` (the `/dev` redirect, the three header links, the themed 404 under `/dev/`, the home page's `/dev/` link); verify it fails before 5.1–5.2 and passes after, against `serve.cjs`
- [ ] 5.4 Add the `devdocs` CI job (Node 22.23.4, `npm ci`, `npm run check`, `npm test`, `npm run build`, full checkout); verify it passes on a pushed `dev` and record its run time in design.md `## Measurements`

## 6. Process and docs

- [ ] 6.1 Add the rule of design Decision 6 to `CLAUDE.md` (next to "The Wiki follows the plugin", and in "User guide data and clips" or a new section with the dev site's commands) and to `CONTRIBUTING.md` (requirements and the PR template's documentation checkbox); verify both read the same rule
- [ ] 6.2 Document the developer site in `docs/ARCHITECTURE.md` (the site's paths table and the web build) and the README (a link to `/dev/`); verify no stale path remains with a search for `/wiki/` in the site sections
- [ ] 6.3 Close out: `npm run check`, `npm test` and `npm run build` in `web/devdocs/`, `npm run check -- --complete` in `web/guide/` (unchanged), `site.cjs` against a full local build, `openspec validate add-developer-site`; record the results and build sizes in design.md `## Measurements`, update "Current state" in `CLAUDE.md`, then commit and push to `dev`

## Workflow follow-up

- Add `devdocs` to the required status checks of the `dev` and `main` rulesets (maintainer, in GitHub).
- After the maintainer merges to `main`, check `https://corrosion-vst.org/dev/` live with `site.cjs`, then archive the change with `/opsx:archive`.
