<!--
Thanks for contributing to Corrosion! Please read CONTRIBUTING.md first.
Pull requests go to `dev`, never to `main`.

Keep it short: a few lines per section is plenty. Delete anything that doesn't apply.
Text inside these comment markers is hidden in the published pull request.
-->

## Why

[What was wrong or missing, and who it affects.]

**Related:** [issue (`Closes #123`), Discussion, or OpenSpec change (`openspec/changes/<name>`); or "None"]

## What changed

[Short summary. For several findings or changes, one bullet each with its ID.]

## Behaviour changes

[What users will notice (sound, controls, defaults, presets, editor, web app), before and after; or "None".]

- [ ] Breaking change (saved presets, DAW projects or host automation). Who is affected and how to migrate: [...]

<!--
Also mention here, if they apply: migrations, reseeded presets (what moved), regenerated golden fixture,
Wiki data, clips or screenshots, new dependencies (name, why, license; regenerate THIRD-PARTY-NOTICES.md),
benchmark medians (`python3 scripts/bench_ratios.py 5`). Attach before/after audio or screenshots when useful.
-->

## How was this tested?

[Environment (OS, Rust, DAW or browser), what you checked by hand or by ear, and how you proved any "sounds the same" claim.]

- [ ] `cargo fmt --check` and `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
- [ ] `cargo test --workspace --no-default-features` (plus `cargo test --lib` for editor changes)
- [ ] Docs updated where they describe the change: Wiki (`web/guide/`, EN + ES), Internals wiki (`docs/internals/`), README, `docs/ARCHITECTURE.md`

<!-- The site's deploy check runs only on `main`; on a pull request it may fail or be skipped, which is expected. -->

## AI assistance

<!-- Name the tool, not the model or its version. Minor help (autocompletion, spelling) needs no disclosure. -->

- [ ] No AI tools used beyond minor assistance
- [ ] AI tools were used: [tool, e.g. "Claude Code"], for [what], verified by [what you read, ran or listened to]

## Notes for the reviewer (optional)

[Where to start, risky parts, what's out of scope or left for later.]
