# Proposal

## Why

Corrosion now invites outside contributors (`CONTRIBUTING.md`, issue forms, a PR template). The only description of how the synth is built, though, is `docs/ARCHITECTURE.md`: 967 dense lines written as the maintainer's as-built record, full of review findings and change names. A newcomer who wants to fix an exciter or add a post-chain stage has no readable map of the engine, and the public site only speaks to players (`/play/`) and sound designers (`/wiki/`).

## What Changes

- **A developer site at `/dev/`** (English only, maintainer's choice), a second Astro Starlight site next to the Wiki. It explains the synth's technical design and architecture for developers. The pages are written by hand for that reader, not generated from `docs/ARCHITECTURE.md`. They cover:
  - the repository and crate layout;
  - the signal chain, from MIDI to the limiter;
  - voices, polyphony and expression;
  - the modal resonator;
  - the 16 exciters and the interaction bus;
  - the material transformations;
  - the post chain, quality modes, oversampling and output protection;
  - parameters, state, presets and migrations;
  - the real-time safety contract and the sample-rate rule;
  - the editor and the web app;
  - the factory bank;
  - testing, CI and benchmarks;
  - how to contribute.
- **Each page names the source files it describes** (`sources:` front matter). A check fails when a named file no longer exists or a top-level module under `src/` isn't covered by any page. Pages can still go stale in what they say, but not in what they point at.
- **The same look as the rest of the site:** dark-only, the shared tokens, header links back to Home, Play and the Wiki, offline search, its own themed 404, and internal links checked at build time.
- **Built and checked like the Wiki:** `web/scripts/build.sh` builds it into `dist/dev/`, it deploys with the site, and CI runs its checks and a full build.
- **The home page links to `/dev/`.**
- **Process:** `CLAUDE.md` and `CONTRIBUTING.md` gain a rule: a change that alters the architecture updates `docs/ARCHITECTURE.md` and the developer pages that describe it, in the same change.

## Capabilities

### New Capabilities
- `developer-docs`: the developer site at `/dev/`, covering its paths, coverage of the architecture, the source references and their check, look and navigation, search, and how it's built and checked.

### Modified Capabilities
- `site-shell`: the site's paths gain `/dev/` (with its trailing-slash redirect and themed 404), the home page links to the developer site, and the shared dark theme covers it.

## Impact

- **New:** `web/devdocs/`, an Astro Starlight project with the same pinned versions as `web/guide/`, its own lockfile, `npm run check` and pages under `src/content/docs/`.
- **Changed:**
  - `web/scripts/build.sh` builds the developer site.
  - `web/site/index.html` gets the `/dev/` link.
  - `web/scripts/site.cjs` checks `/dev/`.
  - `.github/workflows/ci.yml` gets a `devdocs` job.
  - `CLAUDE.md`, `CONTRIBUTING.md`, `README.md` and `docs/ARCHITECTURE.md` get the new section and rule.
- **Dependencies:** no new packages. It uses the same `astro`, `@astrojs/starlight` and `starlight-links-validator` pins as the Wiki, plus a second `npm ci` in the build and in CI.
- **Not affected:** the plugin, the engine, the Wiki's content and its EN/ES checks. The Wiki rule doesn't extend to `/dev/`, which is English only.
- **Repository settings:** the new CI job's check, `devdocs`, should be added to the required status checks of the `dev` and `main` rulesets.
