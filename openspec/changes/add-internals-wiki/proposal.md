# Proposal

## Why

The maintainer has turned on the repository's GitHub Wiki and wants it to hold a detailed description of how everything in Corrosion works inside: the engine, the plugin shell, the editor, the web app, the site and its tooling, the tests and CI. Today the only description is `docs/ARCHITECTURE.md`, a dense as-built record full of review findings and change names. Contributors have no readable map from "what does this stage do" to "which file do I change". And no rule keeps any developer description in step with the code.

## What Changes

- **The Internals wiki: the repository's GitHub Wiki, written in the repository.** Its pages live in `docs/internals/`, one Markdown file per wiki page, so they go through the same pull requests, review and CI as the code they describe. They are in English only (maintainer's choice). Readers see it as "Corrosion Internals", which keeps it apart from the player-facing "Corrosion Wiki" at `/wiki/`.
- **A detailed page set that covers everything.** About 35 pages, each written by hand for a developer who knows Rust but has never seen the code:
  - start pages: home, repository layout, signal chain;
  - the plugin shell: lifecycle and MIDI, parameters, voices and polyphony, note expression, kit mode;
  - the engine: the modal resonator core, the objects, the shared exciter machinery and the exciters by family, the interaction bus and contact, envelopes and modulation, material transformations, DSP utilities, the sample-rate rule;
  - the post chain: routing and quality modes, filter and drive, body, spread and Space, output protection;
  - state: presets and migrations, the factory bank, the randomizer;
  - the front ends: the editor, the web app, the site and the user Wiki's tooling;
  - quality: real-time safety, testing, benchmarks, building, CI and releases;
  - process: contributing and keeping the wiki current, and a glossary.
  The pages use Mermaid diagrams, which GitHub renders, and a sidebar in reading order.
- **Each page names the files it describes.** A `## Source files` section at the foot of each page lists them as links to the repository. A Rust test in the existing test suite (`tests/internals_wiki.rs`) fails:
  - when a named path no longer exists;
  - when a non-test Rust file under `src/`, or a top-level item of `web/`, `scripts/`, `tests/`, `benches/` or `.github/workflows/`, is named by no page;
  - when a link between wiki pages, or to a heading on one, doesn't resolve.
  CI runs it on every platform, with no new job. Pages can still go stale in what they say, but not in what they point at, and a new file can't arrive without a page that describes it.
- **Publishing.** A new workflow, `.github/workflows/wiki.yml`, mirrors `docs/internals/` into the GitHub Wiki on every push to `main`, and on demand. The published wiki therefore always matches the released code that its source links point at (`blob/main/`). Edits made in the GitHub web editor are overwritten, and every page's footer says so.
- **Process rule in `CLAUDE.md`, `CONTRIBUTING.md` and the PR template:** every change updates, in the same change, every Internals page that describes what it touched. That covers code, build, tests, CI and tooling, not only sound. The rule sits next to "The Wiki follows the plugin", with how to find the affected pages (`grep -rl <path> docs/internals/`).
- **`docs/ARCHITECTURE.md` stays** as the maintainer's as-built record (maintainer's choice), and links to the Internals wiki. The wiki links back to it for history.
- **Replaces `add-developer-site`** (maintainer's choice). That proposal, a hand-written developer site at `/dev/` with 0 of 27 tasks done, is deleted. Its page plan, source-reference check and voice carry over here. There is no `/dev/`, no second Astro project and no site change.

## Capabilities

### New Capabilities
- `developer-wiki`: the Internals wiki. Where its pages live and how they're published, what it must cover, the source-file references and their check, links between pages, navigation, and the rule that keeps it current.

### Modified Capabilities
(none: the test runs in CI's existing test lane, and the site, the web app and the user Wiki don't change.)

## Impact

- **New:**
  - `docs/internals/`: about 35 pages, plus `_Sidebar.md` and `_Footer.md`;
  - `tests/internals_wiki.rs`, with no new dependencies;
  - `.github/workflows/wiki.yml`.
- **Changed:**
  - `CLAUDE.md`: the new rule, the commands, and the close-out step;
  - `CONTRIBUTING.md` and `.github/pull_request_template.md`: the rule and a checklist item;
  - `docs/ARCHITECTURE.md` and `README.md`: links to the wiki.
- **Removed:** `openspec/changes/add-developer-site/`.
- **Not affected:** the plugin, the engine and its sound, the site, the web app, and the user Wiki with its EN/ES rule.
- **Repository settings (maintainer, in GitHub):**
  - Create the wiki's first page once, so its git repository exists.
  - Restrict wiki editing to collaborators.
  - The workflow pushes with the job's own token (`contents: write`), so no secret is needed.
