# Contributing to Corrosion

Thank you for your interest in Corrosion! This guide covers where to post,
how to build and test the plugin, the rules the codebase lives by, and how to
send a pull request. Issues, pull requests, discussions, code, comments and
documentation are in English.

By participating, you agree to follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## Where to post

| You have… | Post it as… |
|---|---|
| A confirmed, reproducible bug | A [Bug report](https://github.com/GerMP21/Corrosion-VST/issues/new?template=bug_report.yml) |
| A concrete feature or improvement to propose | A [Feature proposal](https://github.com/GerMP21/Corrosion-VST/issues/new?template=feature_proposal.yml) |
| A question, an idea, sound-design feedback, a DSP review, an architecture discussion, or any change that isn't small | A post in [Discussions](https://github.com/GerMP21/Corrosion-VST/discussions) |

When you post, say which kind of feedback it is: a confirmed bug (steps,
host, expected and actual output), subjective sound-design feedback ("this
preset sounds harsh"), a DSP theory concern (with references where they
apply), an implementation cleanup (ideally with a sketch), or an experimental
idea.

## Before you start

- Search existing issues, pull requests and Discussions to avoid duplicates.
- Check the planned work. What you have in mind may already be there:
  - `openspec/changes/` holds the changes in progress (ignore `archive/`, which
    holds finished ones). With the [OpenSpec CLI](#planning-significant-changes)
    installed, `openspec list` shows the same.
  - `docs/BACKLOG.md` lists the open review findings, known issues and deferred
    features.

  If it's already covered, there's no need to post. If it's related but not
  fully covered, mention the change or backlog item by name.
- Check the [Troubleshooting page of the Wiki](https://corrosion-vst.org/wiki/troubleshooting/).
  Some surprises are documented behaviour.
- For any change that isn't small, start a discussion first, so your effort
  fits the project's direction before you invest time in it.

## Basic requirements

Every contribution, whether AI-assisted or not, must meet these requirements.
Pull requests that don't meet them may be sent back for changes or closed.

1. **Bug reports include the relevant information.** The bug report form asks
   for it. A bug that can't be reproduced can't be fixed.
2. **Pull requests explain why the change exists.** Describe the problem it
   solves or the need it addresses, not just what changed. The diff already
   shows *what*; only you can explain *why*. Link the related issue or
   discussion.
3. **Code is commented where it matters.** Explain the reasoning behind
   non-obvious decisions, workarounds and edge cases. Document public items
   with Rust doc comments. Avoid comments that only restate what the code does.
4. **Code has been double-checked.** Review your own diff before requesting a
   review: remove debug output, commented-out code and unrelated changes, and
   make sure the checks in [Build and test](#build-and-test) pass locally.
5. **Changes include tests.** New features need tests covering the expected
   behaviour and relevant edge cases. Bug fixes need a test that fails without
   the fix and passes with it. For planned changes, write the spec's scenarios
   as tests first and check they fail on the current code. If a change can't
   reasonably be tested (for example, documentation only), explain why in the
   pull request.
6. **Documentation is updated.** Update the README when users see the change,
   `docs/ARCHITECTURE.md` (the as-built reference) when the design changes, and
   the Wiki as described below, all in the same pull request.

## The Internals wiki follows the code

**Every change must update, in the same pull request, every page of the
Internals wiki that describes what it touched.** The Internals wiki is the
repository's [GitHub Wiki](https://github.com/GerMP21/Corrosion-VST/wiki), written in `docs/internals/` and published
from `main`. That covers code, build, tests, CI, scripts and tooling, not only
sound. A new file is named in a page's `## Source files` section in the same
pull request. Don't edit the wiki on GitHub: the next publish overwrites it.

- Find the pages that describe a file with `grep -rl '<path>' docs/internals/`,
  then fix what they say.
- `cargo test --no-default-features --test internals_wiki` checks that every
  named path exists, every source file and script is named by a page, links
  between pages resolve, and the sidebar lists every page. It runs in CI.
- Write for a developer who has never seen the code, in English, in the format
  of the existing pages: Mermaid diagrams with a text twin, links between pages
  as `[text](Page-Name)`, and a final `## Source files` list of repository
  links. Add new pages to `_Sidebar.md`.

## The Wiki follows the plugin

**Any change to how the plugin works or sounds must update the Corrosion Wiki
(`web/guide/`) in the same pull request, in English and Spanish.** A control
that acts differently, a new or removed behaviour, a new model or mode, a fixed
surprise, a changed default or range: update every Wiki page that describes it,
and the Troubleshooting page when it surprises people. A pull request isn't
done while the Wiki says something the plugin no longer does.

Parts of the Wiki are generated from the plugin:

- **After changing a parameter** (range, default, name, unit), a control's
  label or ⓘ text, a model, a mode or the factory bank: run
  `cargo run --bin export_guide_data`, then add or fix the parameter's Spanish
  entry in `web/guide/src/data/parameters.es.json`.
- **After a change that moves the sound engine-wide**, or a change to
  `src/guide/clip_plan.rs`: regenerate the audio clips with
  `scripts/build-guide-clips.sh` (needs `opusenc`).
- **A new parameter** needs a clip plan entry, or an exclusion with a reason.
- **A new model or mode** needs a page that declares it in `covers:`, in
  English and Spanish.
- **After an editor change** (layout, a panel, a control's position): retake
  the screenshots with `node web/scripts/screenshots.cjs` against the local web
  app (needs ImageMagick), and check that the callouts still point at the right
  parts.

`npm run check -- --complete` (in `web/guide/`) fails until the generated data,
the Spanish entries and the pages agree.

**Writing style.** Write as an experienced sound designer sitting next to the
reader: second person, warm, witty when it helps, never marketing ("powerful",
"unleash", "seamless"). Explain what the listener hears and why, give practical
tips (sweet spots, combinations, traps), and assume no DSP training without
talking down. Every claim must agree with the plugin: check tips by ear, by a
render or in the code. Spanish pages use *tú* and natural Spanish, not a
word-for-word translation.

## Rules the codebase lives by

- **Real-time safety.** `process()` and note-on never allocate. Per-voice
  buffers are preallocated and rebuilt in place. `tests/no_alloc.rs` checks it.
- **Sample rate.** Per-sample constants are tuned at 48 kHz and converted with
  `crate::dsp::utils::rate`; each must be exactly its 48 kHz value at 48 kHz.
- **Prove "unchanged".** If you claim a change leaves the sound bit-identical,
  prove it, for example by comparing render hashes against a clean worktree.
  Hashes differ between gnu and musl, so they can't be asserted in CI.
- **Factory presets** are built in code in `src/presets/factory/` and written
  by `cargo run --release --bin seed_presets`, which also re-levels them. Never
  edit the preset files by hand. A change that moves levels or timbre
  engine-wide can push presets off the quality floor: run the bank tests
  (`presets::factory::tests`), reseed, and say in the pull request what moved.
- **Golden mode-bank fixture** (`tests/fixtures/mode_bank_hashes.txt`):
  regenerate it only for an intended sound change, with
  `CORROSION_UPDATE_GOLDEN=1 cargo test mode_banks_match_golden`, and add the
  reason to the doc comment above `mode_banks_match_golden_fixture`.
- **Migrations.** When a parameter's meaning changes, migrate user presets
  (`migrate_preset_json` in `src/presets/mod.rs`) and saved DAW projects
  (`Plugin::filter_state` in `src/lib.rs`). Host automation can't be migrated:
  that makes the change breaking.
- **Benchmarks** are noisy (±10–15 % run to run). Compare within one run, take
  the median of 5 runs (`python3 scripts/bench_ratios.py 5`), and never
  conclude from one pair of numbers.

## Build and test

You need:

- **Rust stable 1.99 or newer**, with `rustfmt` and `clippy`
  (`rustup update stable`).
- **On Linux, for the editor (GUI) build**, the X11 and GL development
  packages. On Debian or Ubuntu:

  ```bash
  sudo apt-get install -y pkg-config libx11-dev libx11-xcb-dev libxau-dev libxcb1-dev libxcursor-dev libxext-dev libxfixes-dev libxft-dev libxi-dev libxinerama-dev libxkbcommon-dev libxrandr-dev libxrender-dev libxtst-dev libxxf86vm-dev libgl-dev
  ```

- **For the Wiki**, Node.js 22.23.4 or newer.
- **For the web app**, the `wasm32-unknown-unknown` target and
  `wasm-bindgen-cli` at the version pinned in `web/Cargo.lock`.
- **Optional:** `opusenc` (Wiki clips), ImageMagick (Wiki screenshots).

**Set your host target first.** `.cargo/config.toml` defaults every `cargo`
command to `x86_64-pc-windows-gnu` and builds into `../corrosion-target`. On
any other host, set your own target, for example:

```bash
export CARGO_BUILD_TARGET=x86_64-unknown-linux-gnu
```

Use `aarch64-apple-darwin` on Apple Silicon and `x86_64-pc-windows-msvc` on
Windows with MSVC.

| Check | Command |
|---|---|
| Format | `cargo fmt --check` |
| Lint | `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` |
| Tests | `cargo test --workspace --no-default-features` |
| Editor (GUI) tests | `cargo test --lib` |
| Real-time safety | `cargo test --no-default-features --test no_alloc` (debug builds only: the checking allocator doesn't exist in release) |
| Docs | `cargo doc --no-deps --no-default-features` (should add no warnings) |
| Internals wiki | `cargo test --no-default-features --test internals_wiki` |
| Third-party notices | `python3 scripts/third_party_notices.py --check` (after a dependency change, run it without `--check` and commit `THIRD-PARTY-NOTICES.md`) |
| Format, lint, tests, editor tests, a render check and the Linux bundle, in one go (Linux) | `./scripts/verify-local.sh` |
| Wiki | `cd web/guide && npm ci && npm run check -- --complete && npm run build` |
| Web app | `web/scripts/build.sh` (writes `web/dist`) |

CI runs these on Linux, Windows and macOS for every pull request, and builds
and validates the plugin bundles. CI on a pull request from a fork may wait for
the maintainer's approval before it runs.

## Planning significant changes

Corrosion is built spec-first with
[OpenSpec](https://github.com/Fission-AI/OpenSpec): research and design come
before any code. New features, significant refactors and architectural changes
follow this process:

1. **Discuss first.** Start a discussion that describes the problem, who it
   affects, what already exists in the codebase, and the alternatives you
   considered. A feature proposal can follow once the idea has a shape.
2. **Write the OpenSpec change** in `openspec/changes/<change-name>/`:
   `proposal.md`, `design.md`, `specs/<capability>/spec.md` and `tasks.md`.
   Check it with `openspec validate <change-name>`.
3. **Get feedback on the plan before implementing it**, as a draft pull request
   that contains only the planning artifacts. Reviewing a plan takes minutes;
   discovering after weeks of work that an implementation went in the wrong
   direction costs everyone far more.
4. **Implement against the approved plan.** Mark each task `- [x]` when it's
   done and verified, record before and after numbers in a `## Measurements`
   section of `design.md`, and add an *As built* note wherever the
   implementation differs from the design. Link the change from your pull
   request.

**You need the OpenSpec CLI**, which runs on Node.js:

```bash
npm install -g @fission-ai/openspec
```

The repository ships OpenSpec commands for Claude Code (`.claude/`) and
OpenCode (`.opencode/`): `/opsx:explore` to think an idea through and
`/opsx:propose` to create a change. They call the CLI, so it must be installed.
You don't need any AI tool: the artifacts are plain Markdown you can also write
by hand.

Small fixes, such as typos, documentation tweaks or contained bug fixes, don't
need a planning artifact.

## Submitting a pull request

**Only the maintainer pushes to `dev` and `main`.** Both branches are
protected: every contribution reaches `dev` through a pull request, and `dev`
reaches `main` through the maintainer's own pull requests.

1. Fork the repository, and create a branch in your fork from `dev`.
2. Open your pull request against `dev`, never against `main`.
3. Keep each pull request focused on a single logical change. Don't mix
   refactoring or formatting changes with behaviour changes.
4. Follow the [commit message convention](#commit-messages). Signing off your
   commits is [strongly recommended](#signing-off-your-commits-dco).
5. Make sure your change meets the [basic requirements](#basic-requirements).
6. Fill in the short pull request template: why the change is needed, what changed,
   how users will notice it, how you tested it, what's out of scope, and
   whether AI tools were used.

To keep your branch up to date with `dev`:

```bash
git remote add upstream https://github.com/GerMP21/Corrosion-VST.git
git fetch upstream
git rebase upstream/dev
```

The site's deploy check runs only on `main`. On a pull request it may fail or
be skipped; that's expected and doesn't block review.

### Commit messages

Write the subject line in the imperative mood ("Add", "Fix", not "Added" or
"Fixes"), in sentence case, without a trailing period, and under 72
characters. If the commit implements a planned change or a review finding, add
its identifier in parentheses:

```
Skip redundant mode retunes (H12)
Block DC on the voice mix (block-voice-mix-dc)
```

Use the body to explain what was wrong and what the commit changes, so the
history makes sense without opening the pull request. Add measurements when
they matter. If the commit breaks existing behaviour, say so explicitly in the
body.

### Signing off your commits (DCO)

Signing off is not mandatory, but it is strongly recommended. A sign-off
certifies, under the
[Developer Certificate of Origin](https://developercertificate.org/), that you
wrote the contribution or otherwise have the right to submit it under the
project's license. For AI-assisted work, it's also your recorded statement that
you take responsibility for the code's origin.

You keep the copyright of what you contribute, and license it to everyone
under the GPL v3 or later, like the rest of Corrosion. The project's notice
reads "Copyright (C) 2026 German Martinez and the Corrosion contributors".
A sign-off doesn't transfer your copyright.

Sign off with the `-s` flag:

```
git commit -s -m "Skip redundant mode retunes (H12)"
```

This adds a `Signed-off-by: Your Name <your.email@example.com>` line using your
Git configuration. To sign off your last commit after the fact, run
`git commit --amend --no-edit --signoff`.

**Commits made by AI tools.** Some tools, such as Claude Code in cloud
sessions, commit under their own identity. A sign-off should come from the
commit's author, so re-author those commits under your name and sign them off
in one pass, then update your branch:

```
git rebase --exec "git commit --amend --no-edit --reset-author --signoff" upstream/dev
git push --force-with-lease
```

Only rewrite your own branch this way. The tool's `Co-Authored-By:` trailer is
kept, so it remains credited. Do this before writing the pull request
description, because rewriting changes the commit hashes. A sign-off added by
an AI tool in its own name doesn't count: the certificate is a statement only a
person can make.

### Code style

Run `cargo fmt` and clippy with `-D warnings` before pushing, and don't add new
warnings. Follow the existing style of the code you're changing. Don't reformat
code unrelated to your change: it hides the real change in the diff and makes
review slower.

### Dependencies

Add new dependencies only when they're genuinely needed. For each one:

- Explain in the pull request why it's needed and why existing dependencies or
  the standard library aren't enough.
- Verify that it's the real, actively maintained package from the expected
  publisher by checking its registry page and source repository, not just its
  name. AI tools sometimes suggest packages that don't exist, and malicious
  packages are sometimes published under those names.
- Confirm that its license is compatible with the GNU GPL v3 or later.
- Make sure it doesn't allocate or lock on the audio thread if it's used there.
- Commit the lockfile changes along with it (`Cargo.lock`, `web/Cargo.lock` or
  `package-lock.json`).
- Regenerate the third-party notices after any change to `Cargo.lock` or
  `web/Cargo.lock`, `cargo update` included: run
  `python3 scripts/third_party_notices.py` and commit `THIRD-PARTY-NOTICES.md`.
  CI fails while it's stale, and also when a crate's license isn't on the
  script's GPLv3-compatible allowlist.
- Third-party material that isn't a crate gets a hand-written entry in
  `THIRD-PARTY-NOTICES.md`. That covers vendored or copied code, generated
  files such as agent skills, fonts, data, and npm code the site ships.

### Breaking changes

If your change breaks existing behaviour, saved presets, DAW projects or host
automation:

- Discuss it in Discussions before implementing it.
- Migrate presets and projects where possible (see
  [Rules the codebase lives by](#rules-the-codebase-lives-by)).
- Say so explicitly in the commit body.
- Mark it in the "Behaviour changes" section of the pull request template,
  explaining who is affected and how to migrate.

## Review times

Corrosion is maintained in limited time, so reviews and replies may take a
while, sometimes several weeks. A slow response doesn't mean your contribution
was ignored or is unwelcome.

- If 30 days pass without a response, one polite follow-up on the pull request
  or discussion is welcome.
- Reviews may go through several rounds of requested changes.
- Pull requests waiting on changes from the contributor may be closed after 30
  days without activity. You're welcome to reopen them when you're ready to
  continue.
- Pull requests waiting on the maintainer are never closed for inactivity.

## AI-assisted contributions

Contributions made with the help of AI tools (code assistants, chat models,
agents, and similar) are welcome, provided the conditions below are met.

### Due diligence

In addition to the basic requirements, by submitting an AI-assisted
contribution, you confirm that:

- **You understand the problem.** You know what you're trying to achieve and
  why, and you could describe it without the tool's help.
- **You understand the solution.** You have read all of the generated content
  and can explain, at least conceptually, how it works and why it solves the
  problem.
- **You have verified it.** You have run and tested it, listened to it where
  it changes the sound, considered edge cases, and confirmed that every API,
  dependency and reference it uses actually exists.
- **You have checked licensing.** To the best of your knowledge, the
  contribution does not reproduce code under a license incompatible with the
  GPL v3.
- **You take responsibility.** You are the author of record, whatever name the
  commit metadata shows, and are accountable for the contribution as if you had
  written every line yourself. "The AI wrote it" is not an acceptable answer
  during review.

A simple test: if the maintainer asks "why does this work?" or "why this
approach?", you should be able to answer in your own words.

### Disclosure

AI use must be disclosed in the "AI assistance" section of the pull request
template, including:

- The tool or tools used, by name (for example "Claude Code"), without model
  names or versions.
- What they were used for (for example: implementation, tests, refactoring,
  documentation).
- How you reviewed and verified the output.

Minor assistance, such as single-line autocompletion or spelling and grammar
fixes, does not need to be disclosed.

Commits made with an AI tool must also credit it with a `Co-Authored-By:`
trailer that names the tool, **not a model or its version**:

```
Co-Authored-By: Claude <noreply@anthropic.com>
```

Some tools add a trailer with a model name automatically (for example
`Claude <model name>`); change it to the form above before you push. A trailer
or a "Generated with" footer shows that a tool was involved, but it doesn't
replace the "AI assistance" section of the pull request template, which also
covers what the tool was used for and how you verified its output.

### Maintainer discretion

The maintainer may decline any AI-assisted contribution at their sole
discretion, without any obligation to justify the decision. This is not a
judgment of you as a contributor: reviewing generated code takes significant
time, and the maintainer needs to be able to protect it.

Contributions found to contain undisclosed AI-generated content may be closed
without review.

## License

Corrosion is released under the [GNU General Public License v3](LICENSE) or (at
your option) any later version. By contributing, you agree that your
contributions will be licensed under the same terms.

## Questions

If something isn't covered here, ask in
[Discussions](https://github.com/GerMP21/Corrosion-VST/discussions).
