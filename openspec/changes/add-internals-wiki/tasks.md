# Tasks

## 1. Replace the old proposal and scaffold

- [x] 1.1 Delete `openspec/changes/add-developer-site/`; verify `openspec list` no longer shows it and nothing under `openspec/specs/` or the docs references it (`grep -rn add-developer-site` outside this change)
- [x] 1.2 Create `docs/internals/` with `Home.md` (title "Corrosion Internals", a stub map), `_Sidebar.md` and `_Footer.md` (design Decision 3 wording); verify the three files exist and the directory has no `README.md`

## 2. The check, tests first

- [x] 2.1 Write `tests/internals_wiki.rs` with the helper unit tests (GitHub heading slugger with repeats, "Source files" section parser, link extractor) and the five checks of design Decision 5; verify the helper tests pass and each check fails on the stub wiki, listing every offending page or file at once
- [x] 2.2 Run each check red against its mutation on a scratch page (a missing `blob/main/` path, an unnamed `src/` file, a broken `#anchor`, a page left out of `_Sidebar.md`, a page with no "Source files"), then remove the scratch page; record the mutations and results in design.md `## Measurements`
- [x] 2.3 Mark `every_source_file_is_named` and `sidebar_links_every_page` `#[ignore = "pages being written (add-internals-wiki)"]` until group 9, so intermediate commits keep the suite green; verify `cargo test --no-default-features --test internals_wiki` passes and the two ignored tests still fail with `-- --ignored`

## 3. Publishing

- [x] 3.1 Add `.github/workflows/wiki.yml` per design Decision 2 (push to `main` on `docs/internals/**` or the workflow, `workflow_dispatch`, `contents: write`, concurrency group `wiki`, clone the wiki repository with the job's token, mirror, commit only when changed, a clear error when the wiki isn't initialized); verify it with `actionlint` if available and by a local dry run of its shell script against a scratch git repository standing in for the wiki (first publish, unchanged re-run makes no commit, a deleted page disappears)

## 4. Start pages

Each task in groups 4–8 writes its pages in the voice of design Decision 6, with a `## Source files` section (Decision 4), checks every claim against the code, adds the pages to `_Sidebar.md`, and verifies `cargo test --no-default-features --test internals_wiki` passes, with `-- --ignored` showing the pages' files no longer listed as unnamed.

- [ ] 4.1 `Home` (what Corrosion is technically, how to read the wiki, the page map, the relation to `docs/ARCHITECTURE.md` and the user Wiki) and `Repository-Layout` (the two Cargo workspaces, the Windows-default target and output directory, every top-level directory)
- [ ] 4.2 `Signal-Chain`: MIDI to the limiter as a Mermaid diagram with a text twin, the per-sample voice loop, what each stage does and why it sits there
- [ ] 4.3 `Glossary`: every term the other pages use without defining (mode, T60, exciter family, bus, governor, oversampling, latency, seqlock, …); grows with groups 5–8, finished in 9.1

## 5. Plugin shell

- [ ] 5.1 `Plugin-Lifecycle`: nih-plug wiring, `initialize` and latency reporting, the `process` block loop, MIDI and the CCs (64, 120, 121, 123), `filter_state` and the host-side migrations, the editor's note ring drain
- [ ] 5.2 `Parameters`: the parameter table and its generated constructors, IDs and names, smoothing, live vs note-on controls, units and the Output dB mapping
- [ ] 5.3 `Voices-and-Polyphony` and `Note-Expression`: voice lifecycle, stealing tiers, sustain and panic fades, the voice-mix DC blocker hand-off, `NoteAddress`, PolyTuning, choke
- [ ] 5.4 `Kit-Mode`: pads, the seqlock `KitTable`, the pad voice start, `pad_gain`, how kits load and save

## 6. Engine

- [ ] 6.1 `Modal-Resonator`: `SecondOrderMode` and the per-sample loop's data layout, building and `rebuild_in_place`, pruning unsoundable modes, the retune cache, dynamics, strike-level normalization, the golden mode-bank fixture
- [ ] 6.2 `Objects`: the profile system and each of the nine objects (modes, pitch offset, what makes it sound like itself), and the object-specific resonators
- [ ] 6.3 `Exciters`: the exciter interface, families and envelopes, drive levels and the governor, global exciter controls, how releases reach friction and specialty exciters, noise, sub-sample events, deterministic excitation
- [ ] 6.4 `Hit-Exciters`, `Friction-Exciters` and `Specialty-Exciters`: each of the 16 models, its physics in plain words, its controls, and its traps (Release Snap, the jet's saturation hold, Metal Chain's links)
- [ ] 6.5 `Interaction-Bus-and-Contact` and `Envelopes-and-Modulation`: the two-way bus (displacement and velocity feedback), implicit contact, ADSR, MSEG with loop safety, position wander, tempo sync
- [ ] 6.6 `Material-Transformations`, `DSP-Utilities` and `Sample-Rate-Rule`: Rust, Damage and Heat and why the fundamental is kept; body, budget, DC blocker, noise; `rate` and the 48 kHz rule with the test that guards it

## 7. Post chain and state

- [ ] 7.1 `Post-Chain`: routing as a Mermaid diagram with a text twin, quality modes and their timings, oversampling, reported latency
- [ ] 7.2 `Filter-and-Drive`, `Body-Spread-and-Space` and `Output-Protection`: the WDF filter, Lorenz drive, FEM body, HRTF spread, Space and its 192 kHz sizing, the half-band filters, the diode clipper (knee, asymmetry, table, meter), the look-ahead limiter
- [ ] 7.3 `Presets-and-Migrations` and `Randomizer`: schema version 4, `migrate_preset_json` and `filter_state` with the Position Envelope example, user folders, the browser, import and export through `rfd`, what the randomizer varies and keeps
- [ ] 7.4 `Factory-Bank`: presets built in code, the seeder, levelling to `TARGET_RMS`, `levels.rs`, the drum kits and `PAD_LEVELS`, the character checks and the bank tests

## 8. Front ends and quality

- [ ] 8.1 `Editor`: egui and baseview, the layout and `BASE_LAYOUT_HEIGHT`, controls and inline help, the palette, the keyboard and its focus rules, the file dialog thread
- [ ] 8.2 `Web-App` and `Site-and-User-Wiki-Tooling`: the wasm engine and editor crates, the AudioWorklet, the nih-plug patch, `web/www`; the site build, the theme tokens, the guide's Starlight project, generated data, clips and screenshots, and the scripts under `web/scripts`
- [ ] 8.3 `Real-Time-Safety` and `Benchmarks-and-Performance`: the no-allocation contract and how `no_alloc` proves it, hot-loop rules (small mode data, cold paths out of line, no division by constants), benchmark noise, medians of 5 and output hashes
- [ ] 8.4 `Testing`, `Building-and-Bundling` and `CI-and-Releases`: the test lanes and what each covers, ignored measurement tests, proving "unchanged", offline renders, WAV scripts; targets, bundling and validators, REAPER testing; the CI jobs, nightly, and the wiki publish
- [ ] 8.5 `Contributing-and-Keeping-This-Wiki-Current`: branches, OpenSpec, close-out, and the wiki rule with the `grep` recipe and the check

## 9. Finish the check, the rule and the docs

- [ ] 9.1 Finish `Glossary` and `Home`'s map; remove the two `#[ignore]`s from 2.3; verify the full `internals_wiki` test passes with no unnamed file and no page missing from the sidebar
- [x] 9.2 Add the rule of design Decision 7 to `CLAUDE.md`: the new ⚠️ section after "The Wiki follows the plugin", the Build and test row, the OpenSpec "Docs" step. Verify by reading the three places together that they state the same rule and command
- [x] 9.3 Add the rule to `CONTRIBUTING.md` (next to "The Wiki follows the plugin" and in the test commands) and to `.github/pull_request_template.md` (the checklist item and the test command); verify both match `CLAUDE.md`'s wording
- [x] 9.4 Link the wiki from `docs/ARCHITECTURE.md` (top note) and `README.md`; verify the links point to `https://github.com/GerMP21/Corrosion-VST/wiki`
- [ ] 9.5 Close out: `cargo fmt --check`, clippy with `-D warnings` (CI lane), the full `cargo test --workspace --no-default-features`, the musl lane, `openspec validate add-internals-wiki`; record page count, test time and results in design.md `## Measurements`, update "Current state" in `CLAUDE.md`, then commit and push to `dev`

## Workflow follow-up

- Maintainer, in GitHub: create one placeholder page so the wiki repository exists, and restrict wiki editing to collaborators.
- After the maintainer merges to `main`, check the first `wiki` run. If the job's token is refused, switch to a `WIKI_TOKEN` secret (design Risks) and note it in design.md.
- On the live wiki, check the Home page, the sidebar, a Mermaid diagram and a heading link.
- Then archive the change with `/opsx:archive`.
