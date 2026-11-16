# Design

## Context

- **The site** is static assets assembled by `web/scripts/build.sh` into `web/dist/`:
  - the home page and 404 (`web/site/`) at the root;
  - the web app under `play/`;
  - the Wiki under `wiki/`, an Astro Starlight project in `web/guide/` (`base: '/wiki'`, `outDir: '../dist/wiki'`).
- **The host's two rules:** directories redirect to their trailing-slash form, and an unknown path gets the nearest `404.html`. `web/scripts/serve.cjs` reproduces them for CI and local checks.
- **The Wiki's look** comes from three small component overrides in `web/guide/src/components/`:
  - `DarkThemeProvider.astro` (always dark);
  - `NoThemeSelect.astro` (no theme switch);
  - `SiteLinks.astro` (header links instead of social icons);
  - plus `web/theme/tokens.css` and `src/styles/guide.css`.
- **The Wiki's checks:** `npm run check` (`scripts/check.mjs`) runs its coverage checks over pages' `covers:` front matter. `starlight-links-validator` fails the build on a broken internal link. CI's `guide` job runs `npm ci`, the check, its tests and the build.
- **Current state of the docs:** `docs/ARCHITECTURE.md` is the as-built reference (967 lines, 12 sections). It stays the maintainer's record; the developer site is a reader's guide to the same ground (see proposal.md, Why).
- **Top-level entries under `src/`:** `bin`, `dsp`, `gui`, `gui_notes.rs`, `guide`, `kit`, `lib.rs`, `offline`, `params.rs`, `presets`, `randomizer`, `voice`, `web`.

## Goals / Non-Goals

**Goals:**
- A newcomer can go from "what is a modal resonator here" to the file they need to change, in a few pages.
- The site can't silently point at code that's gone (the source-reference check).
- Adding the site doesn't touch the Wiki's content, its checks or its EN/ES rule.

**Non-Goals:**
- Generated API docs (`cargo doc`) on the site. They stay a local command; the pages link to the source instead.
- Replacing or shrinking `docs/ARCHITECTURE.md`.
- A Spanish translation (maintainer's choice).
- Checking that prose stays true. Only paths and coverage are checked; accuracy rests on the process rule (Decision 6) and review.

## Decisions

### 1. A second Starlight project in `web/devdocs/`
`web/devdocs/` is its own Astro Starlight project:
- `base: '/dev'`, `outDir: '../dist/dev'`, `trailingSlash: 'always'`;
- the same exact pins as `web/guide/` (`astro`, `@astrojs/starlight`, `starlight-links-validator`; `js-yaml` for the check);
- its own `package-lock.json`;
- English only: no `locales`, `lang: 'en'`, which also removes the language switcher;
- `pagefind: true` for offline search;
- `starlight-links-validator` with `errorOnLocalLinks: true`.

A separate project keeps the Wiki's build, checks and EN/ES rules untouched. *Alternatives:*
- **A section inside the Wiki** would inherit the EN/ES twin check and mix two audiences in one sidebar (maintainer's choice: a separate site).
- **A single static HTML page** loses search, the sidebar and link checking.
- **npm workspaces at `web/`** would share `node_modules` and the theme components, but they move the Wiki's lockfile and CI cache paths for a small saving. Revisit if a third site appears.

### 2. Shared look by copy, with a drift test
`DarkThemeProvider.astro` and `NoThemeSelect.astro` are copied into `web/devdocs/src/components/`, and the dev site uses `web/theme/tokens.css`. Its own `devdocs.css` maps Starlight's variables to the tokens, as `guide.css` does. `SiteLinks.astro` is the dev site's own, with three links (Home, Play, Wiki) and no i18n.

The dev site's check compares the two copied components byte for byte with the Wiki's and fails on a difference. Cross-project imports (`../../guide/src/components/…`) would resolve Starlight's virtual modules from the wrong `node_modules`, so they aren't used.

### 3. Pages and their `parts:` and `sources:` front matter
Each page's front matter declares:
- **`parts:`** the architecture parts it covers, from a fixed list in the check that matches the spec's "Architecture coverage" requirement: `layout`, `signal-chain`, `voices`, `resonator`, `exciters`, `transformations`, `post-chain`, `parameters-and-state`, `realtime-safety`, `sample-rate`, `editor`, `web-app`, `factory-bank`, `testing`, `contributing`.
- **`sources:`** repository-relative paths, files or directories.

`npm run check` (`scripts/check.mjs`) fails, naming the page and the item:
- when a part in the list has no page;
- when a page names an unknown part;
- when a `sources:` path doesn't exist;
- when a top-level entry under `src/` isn't covered by any page's `sources:`. Covered means the entry itself or any path under it.

A `<Sources>` component renders the list at the foot of each page, linking every path to `<repository URL>/blob/main/<path>` (or `/tree/main/` for directories). The repository URL is one constant in `src/site.mjs`: `https://github.com/GerMP21/Corrosion-VST` (maintainer, 2026-11-16; the site's other links point there too).

*Alternative:* checking every inline `` `src/…` `` mention in the prose. It's noisier, and it makes every code span a contract. The front matter is the contract; inline mentions are left to review.

**Planned pages** (the sidebar order):

| Page | `parts:` | Main `sources:` |
|---|---|---|
| Overview | `layout` | `Cargo.toml`, `src/lib.rs`, `web/` |
| Signal chain | `signal-chain` | `src/voice`, `src/dsp` |
| Voices and polyphony | `voices` | `src/voice`, `src/kit`, `src/gui_notes.rs` |
| The modal resonator | `resonator` | `src/dsp/resonators` |
| Exciters and the interaction bus | `exciters` | `src/dsp` (exciter modules) |
| Material transformations | `transformations` | Rust, Damage and Heat code under `src/dsp` |
| Post chain and output | `post-chain` | the post chain, clipper and limiter modules |
| Parameters, state and presets | `parameters-and-state` | `src/params.rs`, `src/presets`, `src/randomizer` |
| Real-time safety | `realtime-safety` | `tests/no_alloc.rs` |
| Sample rate | `sample-rate` | `src/dsp/utils` |
| The editor | `editor` | `src/gui` |
| The web app | `web-app` | `src/web`, `web/engine`, `web/editor`, `web/www` |
| The factory bank | `factory-bank` | `src/presets/factory`, `src/bin/seed_presets.rs` |
| Testing, CI and benchmarks | `testing` | `tests`, `benches`, `.github/workflows/ci.yml`, `src/offline`, `src/guide`, `src/bin` |
| Contributing | `contributing` | `CONTRIBUTING.md`, `openspec` |

Exact paths are set when each page is written: the check, not this table, is the source of truth.

### 4. Diagrams as components, not images
- **Flow diagrams:** a `Flow.astro` copied from the Wiki. It draws boxes and arrows with a two-way arrow for the exciter–object loop, wraps on narrow screens and has an `aria-label`. It's used for the signal chain and the voice loop.
- **Branching diagrams** (the post chain's routing): inline SVG in the page, using the token colours through `currentColor` and CSS variables.

*Alternative:* Mermaid. It needs a client-side script or a headless browser at build time; neither is worth it for a handful of diagrams.

### 5. Build, serve and CI
- **Build:** `web/scripts/build.sh` gains a step after the Wiki: `(cd devdocs && npm ci --no-audit --no-fund && npm run check && npm run build)`. The deploy is unchanged: the host serves `dist/` as before, and its two rules already cover `/dev` and `dev/404.html`.
- **CI:** a `devdocs` job, a twin of `guide`: Node 22.23.4, `npm ci`, `npm run check`, `npm test` (the check's own tests), `npm run build`. It has no Rust step, but the check reads the repository tree, so the job checks out the whole repository.
- **Site checks:** `web/scripts/site.cjs` adds:
  - `/dev` redirects to `/dev/`;
  - `/dev/` has the three header links;
  - `/dev/no-such-page/` returns the themed 404;
  - the home page links to `/dev/`.

### 6. Keeping the pages true
`CLAUDE.md` and `CONTRIBUTING.md` add one rule, next to "The Wiki follows the plugin": a change that alters the architecture (a new stage, a moved module, a changed contract) updates `docs/ARCHITECTURE.md` and every developer page that describes it, in the same change. The source-reference check catches moved and new files; the rule covers what they say.

### 7. Voice of the pages
The pages are written in English, for a developer who knows Rust and has never seen the code. They follow the Wiki's style qualities where they fit: plain words, explain why, a diagram before the details, and accuracy over charm, checked against the code.

They don't name review findings or OpenSpec changes in the text. A short "History" note links to the archived change only where a design choice needs its reasoning.

## Risks / Trade-offs

- **[Two copies of the theme components]** → The drift check fails on any difference; the copies are two files of under 20 lines.
- **[Pages go stale in what they say]** → The process rule (Decision 6) and review. The check only guarantees the pointers. Accepted: the alternative, generating from `ARCHITECTURE.md`, was declined.
- **[A second `npm ci` lengthens the build and CI]** → The `devdocs` job runs in parallel with `guide`. Its build time goes in `## Measurements` after the first run.
- **[Coverage by `src/` entry is coarse]** → A page naming `src/dsp` covers the whole directory. Finer coverage (every file) would make every new file a docs chore; the coarse rule catches new modules, which is what matters for a map.

## Migration Plan

1. Merge to `main`: the site deploys with `/dev/`; nothing else moves.
2. Add `devdocs` to the required status checks of the `dev` and `main` rulesets.
3. Rollback: revert the merge; `/dev/` returns the site's 404 again.
