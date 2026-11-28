# Tasks

## 1. Measure and decide

- [ ] 1.1 Write a test that switches Quality between every pair of modes under a 200 Hz sine at 0.9 of the ceiling and reports the largest step after each switch against the step before; record the numbers in design.md Measurements and verify it fails the "Changing Quality doesn't click" bound on current code (if it passes, take the tooltip fallback and skip group 2)
- [ ] 1.2 Render A/B clips of the Factory room as it is, option A and option A′ (Cable Echo Pad with Factory, a struck Pipe, the default patch at Amount 0.5), with L/R correlation, RMS and decay per clip; give them to the maintainer and record the choice (A, A′ or B) in design.md

## 2. Quality switch without a click (O9)

- [ ] 2.1 Implement the crossfade in `OversampledClipper` with a preallocated second state set; verify the test from 1.1 passes, a fixed-quality render is bit-identical to the base commit (render hash), and `cargo test --no-default-features --test no_alloc` passes with a Quality switch added to it
- [ ] 2.2 Update `docs/ARCHITECTURE.md` and Internals `Output-Protection.md` ("Changing the quality mode") to describe the crossfade; verify `cargo test --no-default-features --test internals_wiki` passes
- [ ] 2.3 If the fallback was chosen instead: add the line to the Quality help (`src/gui/controls.rs`), run `cargo run --bin export_guide_data`, add the Spanish text to `web/guide/src/data/parameters.es.json`, and say it in `quality.mdx` (EN and ES); verify `npm run check` in `web/guide/` passes

## 3. Stereo Factory room (O10, if A or A′ was chosen)

- [ ] 3.1 Write the space-effects scenarios as tests (correlation below 0.5, RMS within 1 dB and decay within 5 % of the mono room, decay per rate) and verify the correlation test fails on current code
- [ ] 3.2 Implement the chosen option in `FactoryReverb` and `process_space`, buffers sized in `new()`; verify the tests pass, `factory_combs_stay_distinct_at_every_size` and the existing Factory tests pass, and `no_alloc` passes
- [ ] 3.3 Measure the CPU of Factory mode with `scripts/bench_ratios.py 5` and record the median ratio in design.md
- [ ] 3.4 Run the bank tests (`presets::factory::tests`); if levels moved, reseed with `cargo run --release --bin seed_presets` and record what moved; verify `generated_bank_equals_the_files_on_disk` passes
- [ ] 3.5 Regenerate the guide clips (`scripts/build-guide-clips.sh`) and update `docs/ARCHITECTURE.md`, Internals `Body-Spread-and-Space.md` (the "Mono room" note and the diagram's `(L+R)/2` label, with its text twin) and `output-chain/space.mdx` in English and Spanish; verify `npm run check` and the internals_wiki test pass
- [ ] 3.6 If B was chosen instead: write the mono tail as intended in ARCHITECTURE, the Internals page and the Space page (EN + ES), and change the spec delta to the "identical tails, by design" requirement; verify `openspec validate smooth-quality-switch-and-stereo-factory` passes

## 4. Close-out

- [ ] 4.1 Mark O9 and O10 FIXED (or decided) in `FABLE-CODE-REVIEW-2026-11-14.md` with date, change and proof, and remove them from `docs/BACKLOG.md`; verify by reading both files
- [ ] 4.2 Run `cargo fmt --check`, clippy with `-D warnings`, `cargo test --workspace --no-default-features`, `cargo test --lib`, `no_alloc` and the internals_wiki test; verify all pass
- [ ] 4.3 Update CLAUDE.md "Current state" and commit and push to `dev`; verify the push

## Workflow follow-up

- Archive with `/opsx:archive` after the maintainer has listened to the result.
