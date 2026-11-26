# Tasks

## 1. Listen and decide

- [ ] 1.1 Prototype option A behind a test-only switch on `HrtfSpread` and render A/B clips (today vs A) of the default patch, a curated chord preset with Spread 0.8 and an experimental preset, with side-to-mid ratio and mono level per clip; give them to the maintainer and record the choice (A or B) in design.md
- [ ] 1.2 Confirm `rate-invariant-spread-and-ridge-wrap` and `smooth-space-and-width` are applied (or record why not), so this change builds on them; verify their spread tests pass on the current tree

## 2. Tests first

- [ ] 2.1 Under A: write the post-chain-routing scenarios (side rises with Spread, mono stays mono, mono level within 1 dB) as tests and verify the first and third fail on current code. Under B: write the "Spread focuses the image" test, verify it passes, change the spec delta to B's requirement, and run `openspec validate widen-hrtf-spread`

## 3. Implement A

- [ ] 3.1 Rewrite the remix as designed (side ×(1 + w), no crossfeed, the levelled proximity mid faded in with w, identity at 0); verify the group 2 tests pass, R3's continuity test and R4's rate tests still pass, `zero_width_preserves_stereo` passes and `no_alloc` passes
- [ ] 3.2 Run `curated_measurements_table` and the bank tests before and after, reseed with `cargo run --release --bin seed_presets`, record what moved in design.md, and have the maintainer listen to the curated folders; verify `generated_bank_equals_the_files_on_disk` and the character checks pass
- [ ] 3.3 Regenerate the guide clips (`scripts/build-guide-clips.sh`) and record it

## 4. Help text and docs (A or B)

- [ ] 4.1 Make the Spread help in `src/gui/controls.rs` describe what Spread does, run `cargo run --bin export_guide_data` and update the Spanish entry in `web/guide/src/data/parameters.es.json`; verify `guide_data_equals_the_files_on_disk` passes
- [ ] 4.2 Update the `spread_width` doc in `src/params.rs`, `docs/ARCHITECTURE.md`, Internals `Body-Spread-and-Space.md` (the formula, the "Raising Spread Width narrows" note and the diagram with its text twin), and the Wiki's `output-chain/dispersion.mdx` Spread section in English and Spanish, plus a Troubleshooting entry under A ("an old project sounds wider"); verify `npm run check` in `web/guide/` and `cargo test --no-default-features --test internals_wiki` pass

## 5. Close-out

- [ ] 5.1 Update the C5 note on the mono remix in `docs/archive/FABLE-CODE-REVIEW-2026-07-07.md` (date, change, proof) and remove both the mono remix item and "Spread Width's help is backwards" from `docs/BACKLOG.md`; verify by reading both
- [ ] 5.2 Run `cargo fmt --check`, clippy with `-D warnings`, `cargo test --workspace --no-default-features`, `cargo test --lib`, `no_alloc` and the internals_wiki test; verify all pass
- [ ] 5.3 Update CLAUDE.md "Current state" (marking BREAKING under A) and commit and push to `dev`; verify the push

## Workflow follow-up

- Archive with `/opsx:archive` after the maintainer has listened to the reseeded bank.
