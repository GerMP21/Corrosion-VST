<!--
Thanks for contributing to Corrosion!

Please read CONTRIBUTING.md (at the root of the repository) before opening this pull request.
Pull requests go to `dev`, never to `main`.

How to fill in this template:
- Replace every [text in square brackets] with your answer.
- Sections marked "if applicable" can be deleted when they don't apply.
- Don't write inside these comment markers: anything inside them is hidden in the published pull request.
- Replies may take a while: Corrosion is maintained in limited time.
-->

## Why

[What problem does this solve, or what need does it address? Describe what was wrong or missing before this change, and who it affects.]

## Related work

- **Issue:** Closes #[issue number], or "None"
- **Discussion:** [link, or "None"]
- **Planning artifact:** [OpenSpec change (`openspec/changes/<name>`), or "None" for small fixes]
- **Builds on:** [other pull requests this depends on or follows, or "None"]

## Type of change

<!-- Check all that apply. -->

- [ ] Bug fix
- [ ] New feature
- [ ] Sound or DSP change
- [ ] Factory presets or drum kits
- [ ] Editor (GUI)
- [ ] Web app
- [ ] Refactor (no behaviour change)
- [ ] Performance
- [ ] Wiki or documentation
- [ ] Tests only
- [ ] Build, CI, or configuration
- [ ] Dependencies

## What changed

[Short summary of the change.]

<!--
If the pull request implements several planned changes or review findings, give each one its own subsection with its identifier, for example:

### H12: skip redundant mode retunes
- What was wrong.
- What changed, and where.
-->

## Behaviour changes

<!-- What users will notice: sound, controls, defaults, presets, editor, web app, performance. Repeat the block for each area, or replace it with "None". -->

- **Area:** [what changes]
  - **Before:** [previous behaviour]
  - **After:** [new behaviour]

- [ ] No breaking changes
- [ ] This pull request introduces a breaking change (saved presets, DAW projects or host automation)

**Who is affected and how to migrate:** [only for breaking changes; otherwise delete this line. Presets and projects should migrate; host automation can't.]

## How was this tested?

- **Tested on commit:** [short hash]
- **Environment:** [OS, Rust version, and the DAW or browser you checked in, with versions]
- **CI:** [link to the run and result]
- **Manual checks:** [what you verified by hand or by ear, in which host, and how]
- **Unchanged sound:** [if you claim something stays bit-identical, how you proved it (render hash comparison against a clean worktree), or delete this line]

<!-- The site's deploy check runs only on `main`; on a pull request it may fail or be skipped, which is expected. -->

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
- [ ] `cargo test --workspace --no-default-features`
- [ ] `cargo test --lib` (editor tests)
- [ ] `cargo test --no-default-features --test no_alloc` (debug build)
- [ ] `cargo doc --no-deps --no-default-features` adds no warnings
- [ ] `cargo test --no-default-features --test internals_wiki` (the Internals wiki's paths, coverage and links)
- [ ] `npm run check -- --complete` in `web/guide/` (if the Wiki or its generated data changed)
- [ ] `web/scripts/build.sh` (if the web app changed)
- [ ] New tests fail without this change and pass with it

## Audio, screenshots, or recordings (if applicable)

[For sound changes, attach short before and after clips. For editor or Wiki changes, attach before and after screenshots.]

## Performance (if applicable)

[Benchmark and filter, the median of 5 runs (`python3 scripts/bench_ratios.py 5`) before and after, and the hardware.]

## Generated files and migrations (if applicable)

<!-- Check everything this change needed. -->

- [ ] Preset migration (`migrate_preset_json`) and DAW project migration (`Plugin::filter_state`)
- [ ] Factory presets reseeded (`cargo run --release --bin seed_presets`); what moved: [presets and levels]
- [ ] Golden mode-bank fixture regenerated, with the reason documented
- [ ] Wiki data exported (`cargo run --bin export_guide_data`) and Spanish entries added to `parameters.es.json`
- [ ] Wiki clips regenerated (`scripts/build-guide-clips.sh`)
- [ ] Wiki screenshots retaken (`node web/scripts/screenshots.cjs`)

## Dependencies (if applicable)

<!-- Repeat the block for each package. -->

- **Package:** [name and version; added, updated, or removed]
  - **Why it's needed:** [why existing dependencies or the standard library aren't enough]
  - **License:** [license name; it must be compatible with the GPL v3 or later]

- [ ] Each new package is the real, maintained one from the expected publisher
- [ ] It doesn't allocate or lock on the audio thread, if it's used there
- [ ] Lockfile changes are included (`Cargo.lock`, `web/Cargo.lock`, `package-lock.json`)
- [ ] `THIRD-PARTY-NOTICES.md` is regenerated (`python3 scripts/third_party_notices.py`), and third-party material that isn't a crate has an entry there

## Not in this pull request

[What is deliberately out of scope, known limitations, and follow-ups, or "None".]

## After merging (if applicable)

- [ ] Manual steps, documented in: [file or link]
- [ ] OpenSpec change to archive, or the next planned change: [name]

## Notes for the reviewer

[Where to start, the riskiest parts, and anything you're unsure about. On large pull requests, suggest a review order.]

## AI assistance

<!-- Check one. -->

- [ ] No AI tools used beyond minor assistance (single-line autocompletion, spelling or grammar fixes)
- [ ] AI tools were used

<!-- If AI tools were used, fill in the three lines below; otherwise delete them. Name the tool, not the model or its version. A "Generated with" footer alone is not a disclosure. -->

- **Tools:** [names, for example "Claude Code"]
- **Used for:** [implementation, tests, refactoring, documentation...]
- **How I verified the output:** [what you read, ran, listened to, and checked]

---

## Checklist

**Before opening**
- [ ] I checked existing issues, pull requests, Discussions, `openspec/changes/` and `docs/BACKLOG.md` for duplicates
- [ ] This pull request targets `dev`
- [ ] This pull request contains a single logical change

**Code**
- [ ] I reviewed my own diff and removed debug output, commented-out code, and unrelated changes
- [ ] Code is commented where the reasoning isn't obvious
- [ ] Formatting and clippy pass, with no new warnings

**Tests and documentation**
- [ ] I added or updated tests, or explained above why that isn't possible
- [ ] I updated the README and `docs/ARCHITECTURE.md` where this change affects them
- [ ] If the plugin works or sounds different, I updated every Wiki page that describes it, in English and Spanish
- [ ] I updated every Internals wiki page (`docs/internals/`) that describes the files I changed, and named any new file in a page's "Source files" section

**Commits**
- [ ] Commit messages follow the convention (imperative subject under 72 characters, with the change or finding ID when relevant)
- [ ] Commits are signed off (DCO), which is strongly recommended
- [ ] AI-assisted commits credit the tool with a `Co-Authored-By:` trailer, without a model name

**Responsibility**
- [ ] I can explain how and why this change works in my own words
