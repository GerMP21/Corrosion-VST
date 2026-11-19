# Design

## Context

- **Why:** see proposal.md.
- **GitHub Wikis** are a separate git repository, `<repo>.wiki.git`. Its rules:
  - Every `Page-Name.md` at its root is a page titled "Page Name"; directories don't appear in page URLs, so names must be unique.
  - `Home.md` is the landing page; `_Sidebar.md` and `_Footer.md` are shown on every page.
  - Pages link to each other by name (`[text](Page-Name)`, `[text](Page-Name#heading)`).
  - GitHub renders Mermaid in wiki Markdown.
  - The wiki repository exists only after a first page is created in the web UI.
- **This session can't reach the wiki repository.** The cloud git proxy authorizes `GerMP21/Corrosion-VST` only, and `git ls-remote …wiki.git` asks for credentials. Anything that writes the wiki has to run in GitHub Actions.
- **Existing documentation:**
  - `docs/ARCHITECTURE.md`: 1051 lines, the maintainer's as-built record, now with two Mermaid diagrams. It stays (maintainer's choice).
  - The user Wiki at `/wiki/` (`web/guide/`): for players, in English and Spanish. It is untouched here.
- **`add-developer-site`** (proposed 2026-11-16, no task done) planned the same content as an Astro site at `/dev/`. The maintainer chose the GitHub Wiki instead. Its page plan, `sources:` check and voice are reused here; its site, CI job and theme work are dropped.
- **Size of what must be covered:**
  - 89 non-test Rust files under `src/` (96 with test files);
  - 5 Rust files in `web/engine` and `web/editor`;
  - 10 entries in `scripts/`, 20 in `tests/`, 1 in `benches/`, 1 workflow.
- **CI** (`.github/workflows/ci.yml`) runs `cargo test --workspace --no-default-features` on Linux (gnu and musl), Windows and macOS, with `permissions: contents: read`.

## Goals / Non-Goals

**Goals:**
- A contributor can go from a behaviour ("why does the bow snap on release?") to the file and function that does it in two or three pages.
- The wiki can't silently point at code that's gone, and a new file can't land without a page that names it.
- Publishing needs no step from anyone after the one-time setup.

**Non-Goals:**
- Generated API docs. `cargo doc` stays a local command, and pages link to the source.
- Shrinking or replacing `docs/ARCHITECTURE.md`.
- A Spanish translation.
- Checking that prose stays true. Only paths, coverage and links are checked; what the pages say rests on the rule (Decision 7) and review.
- Changing the site, the web app or the user Wiki.

## Decisions

### 1. Source in `docs/internals/`, published by a workflow
The pages are flat files in `docs/internals/`, named as the wiki names them (`Signal-Chain.md`). The source can't be the wiki repository itself (maintainer's choice; see Context): it would skip review and CI, and these sessions couldn't edit it.

The directory is named `internals`, not `wiki`, because "Wiki" already means the player-facing Corrosion Wiki everywhere in `CLAUDE.md`, `CONTRIBUTING.md` and the site. Readers see "Corrosion Internals": the title on `Home.md` and in the sidebar heading.

*Trade-off:* page-to-page links use wiki names without `.md`, so they don't resolve when browsing `docs/internals/` in GitHub's file view. Pages are meant to be read in the wiki. The footer says so, and the directory carries no README, because it would publish as a page.

### 2. The publish workflow, `.github/workflows/wiki.yml`
- **Triggers:** `push` to `main` when `docs/internals/**` or the workflow changes, plus `workflow_dispatch`.
- **Permissions:** `contents: write`, for this workflow only.
- **Concurrency:** group `wiki`, so a newer run cancels an older one; the newest `main` always wins.
- **Steps:**
  1. Check out the repository.
  2. Clone `https://x-access-token:${{ github.token }}@github.com/${{ github.repository }}.wiki.git`.
  3. Delete every tracked file in the clone, then copy `docs/internals/*.md` in.
  4. If `git status --porcelain` is empty, stop with success.
  5. Otherwise commit as `github-actions[bot]`, with the message `Publish docs/internals from <short sha>`, and push to the wiki's current branch.
- **No third-party action:** about 20 lines of shell, so nothing outside GitHub's own `actions/checkout` (pinned as `ci.yml` pins it) gets a write token.
- **The clone fails when the wiki isn't initialized.** The step names the fix: create any page once in the web UI.

*Why `main` only:* the pages' source links point at `blob/main/`. Publishing from `dev` would describe code that isn't released yet and break those links until the next merge.

*Alternatives:*
- Publishing from `dev` (see above).
- A marketplace wiki-sync action: it hands a write token to third-party code.
- A personal access token: the job's own token is enough for the same repository's wiki. A `WIKI_TOKEN` secret is the fallback if GitHub refuses (see Risks).

### 3. Page set and sidebar
These are the planned pages, in sidebar order; file names are the wiki names. Exact `Source files` lists are set when each page is written, and the test (Decision 5), not this table, is the source of truth for coverage.

| Group | Page | Main sources |
|---|---|---|
| Start | `Home` | none (map of the wiki, how to read it, link to `docs/ARCHITECTURE.md`) |
| | `Repository-Layout` | `Cargo.toml`, `.cargo/config.toml`, `web/Cargo.toml`, top-level directories |
| | `Signal-Chain` | `src/lib.rs`, `src/voice/mod.rs`, `src/dsp/post_processing/post_chain.rs` |
| Plugin | `Plugin-Lifecycle` | `src/lib.rs` (`initialize`, `process`, latency, `filter_state`, MIDI and CCs) |
| | `Parameters` | `src/params.rs` |
| | `Voices-and-Polyphony` | `src/voice/mod.rs`, `src/voice/manager.rs` |
| | `Note-Expression` | `src/voice/manager.rs`, `src/lib.rs` |
| | `Kit-Mode` | `src/kit/mod.rs` |
| Engine | `Modal-Resonator` | `src/dsp/resonators/{mod,core,basic}.rs` |
| | `Objects` | `src/dsp/profile.rs`, `src/dsp/profiles/*`, the object-specific `src/dsp/resonators/*` |
| | `Exciters` | `src/dsp/exciters/mod.rs`, `src/dsp/mod.rs` (shared machinery, drive levels, governor, release, sub-sample events) |
| | `Hit-Exciters`, `Friction-Exciters`, `Specialty-Exciters` | the exciter files, grouped by the code's `ExciterFamily` |
| | `Interaction-Bus-and-Contact` | `src/dsp/interaction.rs`, `src/dsp/utils/contact.rs` |
| | `Envelopes-and-Modulation` | `src/dsp/envelopes/mod.rs` |
| | `Material-Transformations` | `src/dsp/transforms.rs` |
| | `DSP-Utilities` | `src/dsp/utils/*` except `rate.rs` |
| | `Sample-Rate-Rule` | `src/dsp/utils/rate.rs` |
| Post chain | `Post-Chain` | `src/dsp/post_processing/{mod,post_chain}.rs` (routing, quality modes, oversampling, latency) |
| | `Filter-and-Drive` | `wdf_filter.rs`, `lorenz_drive.rs` |
| | `Body-Spread-and-Space` | `fem_body.rs`, `hrtf_spread.rs`, `space.rs` |
| | `Output-Protection` | `half_band.rs`, `oversampled_clipper.rs`, `lookahead_limiter.rs`, the voice-mix DC blocker |
| State | `Presets-and-Migrations` | `src/presets/{mod,browser}.rs`, `src/gui/file_dialog.rs` |
| | `Factory-Bank` | `src/presets/factory/*`, `src/bin/seed_presets.rs` |
| | `Randomizer` | `src/randomizer/mod.rs` |
| Front ends | `Editor` | `src/gui/*`, `src/gui_notes.rs` |
| | `Web-App` | `src/web/*`, `web/engine`, `web/editor`, `web/www`, the nih-plug patch |
| | `Site-and-User-Wiki-Tooling` | `web/site`, `web/theme`, `web/guide`, `web/scripts`, `src/guide/*`, the guide bins, `scripts/build-guide-clips.sh` |
| Quality | `Real-Time-Safety` | `tests/no_alloc.rs` |
| | `Testing` | `tests/*`, `src/offline/*`, `src/bin/{render,render_presets}.rs`, the WAV scripts |
| | `Benchmarks-and-Performance` | `benches/performance.rs`, `scripts/bench_ratios.py`, `web/scripts/bench-*.mjs` |
| | `Building-and-Bundling` | `scripts/{bundle,check_bundle,validate-plugins,verify-local}.sh`, `scripts/third_party_notices.py` and `THIRD-PARTY-NOTICES.md`, `docs/REAPER_TESTING.md` |
| | `CI-and-Releases` | `.github/workflows/ci.yml`, `.github/workflows/wiki.yml` |
| Process | `Contributing-and-Keeping-This-Wiki-Current` | `CONTRIBUTING.md`, `openspec/`, `CLAUDE.md` |
| | `Glossary` | none |

`_Sidebar.md` lists them under the group headings. `_Footer.md` says: "Corrosion Internals is generated from `docs/internals/` in the repository. Edits made here are overwritten on the next publish." It links to the directory.

### 4. Page format
- **Title:** the page's first line is an `#` heading. GitHub shows the file name as the title, so the heading only repeats it for readers of the source.
- **Diagrams:** Mermaid fenced blocks. Each diagram is followed by the same content as a list or table, so a reader whose renderer skips Mermaid loses nothing.
- **Code references inline:** `` `src/dsp/interaction.rs` `` or `` `Voice::process_sample` ``. Inline mentions aren't checked; the "Source files" section is the contract. Repository links anywhere on a page are checked (Decision 5).
- **Ending:** every page ends with `## Source files`, a bullet list of links of the form `https://github.com/GerMP21/Corrosion-VST/blob/main/<path>` for files and `…/tree/main/<path>` for directories, each with a few words on what the page uses it for.
- **Exempt from "Source files":** `Home`, `Glossary`, `_Sidebar` and `_Footer`.

### 5. The check: `tests/internals_wiki.rs`
An integration test in the existing suite, with no new dependency. It reads files relative to `CARGO_MANIFEST_DIR`, normalizes `\r\n` (Windows checkouts) and compares paths with `/` separators. Its tests, each failing with every offending page and item listed at once:
- **`every_page_lists_its_source_files`:** each non-exempt page has a `## Source files` section with at least one link, and every link in it uses the repository's `blob/main/` or `tree/main/` prefix.
- **`every_linked_repository_path_exists`:** every `blob/main/` or `tree/main/` link anywhere on a page names an existing file (blob) or directory (tree).
- **`every_source_file_is_named`:**
  - every `.rs` under `src/` is named by its own path in some page's "Source files" section, except test files: `tests.rs`, `*_tests.rs`, and anything under a directory named `tests`;
  - every top-level item of `web/`, `scripts/`, `tests/`, `benches/` and `.github/workflows/` is named by its own path or a path under it;
  - ignored: `Cargo.lock`, `package-lock.json` and the untracked build outputs `dist`, `node_modules` and `target`.
- **`wiki_links_resolve`:** every relative link (no scheme, no leading `/`) names an existing page, and its `#fragment` matches a heading on that page. Headings are slugged as GitHub does: lower case, characters other than letters, digits, spaces, `-` and `_` removed, spaces to `-`, and a `-1`, `-2` suffix on repeats.
- **`sidebar_links_every_page`:** `_Sidebar.md` links every page except itself and `_Footer`.
- **Helper unit tests:** the slugger, the section parser and the link extractor, on fixed strings.

Each assertion is run red against a mutation before the pages exist: a missing path, an unnamed file, a broken anchor, a page left out of the sidebar. That follows the project's tests-first habit.

*Why a Rust test rather than a script and a CI job:* it runs everywhere `cargo test` already runs (four platforms, musl), needs no new job or required check, and fails locally in the same command contributors already run.

*Why file-level coverage for `src/`:* the maintainer asked for a detailed wiki of everything. Directory-level coverage, which `add-developer-site` used, would let a whole new exciter land unnamed. The cost is that every new `src/` file needs a line in a page, which is the point of the rule.

### 6. Voice of the pages
- **Reader:** English, for a developer who knows Rust and has never opened the code.
- **Style:** plain words; explain why before how; a diagram before the details; numbers where they matter (rates, sizes, dB, CPU ratios); every claim checked against the code while writing.
- **History stays out of the prose:** no review finding IDs or change names in the text. A short *History* line at the end of a section may link to an archived change or to `docs/ARCHITECTURE.md` where a design choice needs its reasoning.
- **Not the user Wiki's voice:** the u-he style is for players. These pages are technical, but never padded.

### 7. Keeping the pages true: the rule
`CLAUDE.md` gets a new ⚠️ section right after "The Wiki follows the plugin", titled "The Internals wiki follows the code, ALWAYS". Its rule: every change (code, build, tests, CI, scripts, tooling) updates, in the same change, every page in `docs/internals/` that describes what it touched, and names any new file. The section also says:
- **How to find the affected pages:** `grep -rl '<path>' docs/internals/` for each changed file.
- **The check:** `cargo test --no-default-features --test internals_wiki`.
- **What it doesn't replace:** `docs/ARCHITECTURE.md` and the user Wiki are still updated as before.

The rest of `CLAUDE.md` changes to match:
- the Build and test table gets the check;
- the OpenSpec workflow's "Docs" step names the Internals pages;
- "Current state" gets the change's row.

`CONTRIBUTING.md` states the same rule next to "The Wiki follows the plugin". The PR template gets a checklist item ("I updated every Internals page that describes the files I changed") and an entry for the check. `docs/ARCHITECTURE.md` and `README.md` link to the wiki.

### 8. Removing `add-developer-site`
The old change directory is deleted in this change's first commit, with a line in this design (Context) saying what carried over. It was never archived and has no synced specs, so nothing under `openspec/specs/` changes.

## Risks / Trade-offs

- **[The job's token can't push to the wiki]:** GitHub has allowed `GITHUB_TOKEN` with `contents: write` to push to the same repository's wiki, but it isn't checked in this session. → The first run on `main` proves it. If it's refused, fall back to a fine-grained `WIKI_TOKEN` secret with Contents read and write on this repository (a one-line workflow change), and record that here.
- **[Mermaid not rendered in the wiki]** → Every diagram has a text equivalent (Decision 4). Check after the first publish.
- **[Pages go stale in what they say]** → The rule and review. The test only guarantees the pointers and coverage. Accepted: generating prose from code was never on the table.
- **[Coverage turns CI red mid-way]** → While the pages are being written, the coverage test fails by design. Commits stay local, or the coverage test stays `#[ignore]` with a reason, until every page is written. It is un-ignored before the first push, so `dev` stays green.
- **[Two documents for one architecture]** (`ARCHITECTURE.md` and the wiki) → They have different roles: the record with history and numbers, against the explanation. Each links the other, and the rule names both.
- **[Edits in the web UI are lost]** → The footer warns, and the maintainer restricts wiki editing to collaborators.
- **[Writing about 35 accurate pages is long]** → The tasks are split by page group, each verified against the code and by the test, so the work can stop and resume at any group boundary.

## Migration Plan

1. Implement and push to `dev`; CI runs the new test on every platform.
2. **One-time setup (maintainer):** create a placeholder page in the GitHub Wiki so the wiki repository exists, and restrict wiki editing to collaborators.
3. Merge `dev` into `main`: `wiki.yml` publishes. Check the Home page, the sidebar, a Mermaid diagram and a heading link on the live wiki.
4. **Rollback:** revert the merge. To clear the wiki, run the workflow from a commit whose `docs/internals/` holds only a `Home.md`.


## Measurements

- **The checks, run red (task 2.2)** on a scratch page with a link to a missing file, a link to a missing heading, a link to a missing page, and no "Source files" section. Each check fails and names the item:
  - `every_page_lists_its_source_files`: "Scratch-Mutation: no `## Source files` section".
  - `every_linked_repository_path_exists`: "no file `src/no_such_file.rs`".
  - `wiki_links_resolve`: "`Home#no-such-heading`: Home has no heading" and "`No-Such-Page` names no page".
  - `sidebar_links_every_page`: "_Sidebar doesn't link: [\"Scratch-Mutation\"]".
  - `every_source_file_is_named`, on the stub wiki: 134 unnamed items (non-test `src/` files plus the top-level items of `web/`, `scripts/`, `tests/`, `benches/` and `.github/workflows/`).

  With the scratch page removed, the suite passes, with the two coverage checks ignored until the pages exist (task 2.3).
