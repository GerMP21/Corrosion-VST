# Tasks

## 1. Bow Roughness ripple

- [ ] 1.1 Measure today's ripple at 44.1, 48 and 96 kHz (default Bow, Roughness 1) and the default bow speed; render A/B clips of the default patch and every Bow factory preset with the recommended `K · v` ripple and the two alternatives; get the maintainer's choice and record it with the numbers in design.md
- [ ] 1.2 Add the `sample-rate-independence` scenarios as tests (same ripple at three rates; below Nyquist at maximum speed) and verify they fail on current code
- [ ] 1.3 Implement the chosen ripple in `src/dsp/exciters/bow.rs`; verify 1.2 passes, `tests/bow_exciter.rs` and `every_sound_shaping_control_changes_its_note` pass
- [ ] 1.4 Run the bank tests, reseed (`cargo run --release --bin seed_presets`), record what moved; update `docs/internals/Friction-Exciters.md`, `docs/ARCHITECTURE.md` and the Wiki's Bow page (EN + ES) if it describes Roughness; regenerate guide clips (`scripts/build-guide-clips.sh`); verify `generated_bank_equals_the_files_on_disk` and `npm run check -- --complete` pass

## 2. Brush Wire Stiffness

- [ ] 2.1 Measure a single brush impulse's level and spectrum at stiffness 0, 0.3, 1 today; render A/B clips of the Wire Brush presets with the corrected, level-compensated high-pass; get the maintainer's choice (tone control or re-described level control) and record it
- [ ] 2.2 Add the "Brush Wire Stiffness changes the brush's tone" scenario as a test and verify it fails on current code
- [ ] 2.3 Implement in `src/dsp/exciters/wire_brush.rs`; verify 2.2 and `every_sound_shaping_control_changes_its_note` pass, and that the ⓘ text ("Higher sounds brighter and tighter") now matches the measured direction
- [ ] 2.4 Bank tests, reseed, record what moved; update the ⓘ text if needed (`src/gui/controls.rs`), `cargo run --bin export_guide_data` + `parameters.es.json`, the Wiki's Wire Brush page (EN + ES), `docs/internals/Hit-Exciters.md`, `docs/ARCHITECTURE.md`; regenerate guide clips; verify the guide checks pass

## 3. Position Wander range

- [ ] 3.1 Confirm the choice with the maintainer (shrink to 0–0.5, recommended; rescale; or widen), since every option is BREAKING or changes saved sounds; record it
- [ ] 3.2 Add the `strike-position` scenarios as tests (90 % vs 100 % differ; stored 0.8 loads as 0.5 bit-identically, for a preset and for host state) and verify the first fails on current code
- [ ] 3.3 Change `position_wander_param` to 0–0.5 (`src/params.rs`), keep `presets/factory` bit-identical (`generated_bank_equals_the_files_on_disk`), and verify 3.2 and the control-response tests pass
- [ ] 3.4 Add the **BREAKING** automation note to the Wiki (EN + ES) and Troubleshooting; `export_guide_data` + `parameters.es.json`; update `docs/internals/Interaction-Bus-and-Contact.md`, `docs/internals/Parameters.md`, `docs/ARCHITECTURE.md`; verify the guide and internals checks pass

## 4. Web app diode dot

- [ ] 4.1 Add the `diode-stage` scenarios as a test on `WebEngine` (a loud block reports ≥ 6 dB, a quiet one < 0.1 dB) and verify the loud case fails on current code
- [ ] 4.2 Publish the meter at the end of `process_block` and drop the duplicate in `Plugin::process`; verify 4.1 passes, the plugin's diode-dot tests pass and a render hash of the default patch is unchanged
- [ ] 4.3 Check in Chromium (`web/scripts/smoke.cjs` or by hand) that the dot lights on a loud preset; update `docs/internals/Web-App.md` and `docs/internals/Output-Protection.md`; verify `internals_wiki` passes

## 5. Lookahead limiter switch

- [ ] 5.1 Add the `output-limiting` scenarios as tests (no stale samples after Hard → Lookahead; no dropout on the switch) and verify the first fails on current code
- [ ] 5.2 Clear the lookahead lines on every mode change and crossfade from the Hard path over the delay; verify 5.1, `tests/limiter.rs`, `no_alloc` and `scripts/check_clicks.py` on a rendered switch pass
- [ ] 5.3 Update `docs/internals/Output-Protection.md`, `docs/ARCHITECTURE.md`; verify `internals_wiki` passes

## 6. Empty kit in Kit mode

- [ ] 6.1 Confirm the choice with the maintainer (round-trip as Kit, recommended; or document today's behaviour); record it
- [ ] 6.2 Add the "Empty kit round trip" scenario as a test, plus a host-state round trip of an empty Kit project (to prove the host path already works), and verify the preset test fails on current code
- [ ] 6.3 Write `kit` in Kit mode even when empty (`src/presets/mod.rs`); verify 6.2, the old-Kit migration tests and `tests/preset_roundtrip.rs` pass, and the factory bank is unchanged
- [ ] 6.4 Update the Wiki's Kit page (EN + ES) if it mentions empty kits, `docs/internals/Kit-Mode.md`, `docs/internals/Presets-and-Migrations.md`; verify the checks pass

## 7. Drop Envelope Spread

- [ ] 7.1 Add the "bottom of the range acts" scenario as a test and verify it fails on current code
- [ ] 7.2 Clamp to 40–1000 ms in `src/dsp/exciters/metal_chain.rs`; verify 7.1 passes and the factory bank is bit-identical
- [ ] 7.3 Update `docs/internals/Hit-Exciters.md` (or the page that describes Metal Chain) and the Wiki if it states the range; verify the checks pass

## 8. Stale comments and docs

- [ ] 8.1 Rebuild the list: every discrepancy the Internals pages note ("still says", "despite its name", "the comment says") plus the backlog's examples, minus those owned by `tidy-review-nits` and `widen-hrtf-spread`; record it with file and line in design.md `## Measurements`
- [ ] 8.2 Fix each listed comment, doc comment, `docs/ARCHITECTURE.md`, `README.md` and `CLAUDE.md` line; tick them off in the list; verify `cargo doc --no-deps --no-default-features` adds no warnings
- [ ] 8.3 After `trim-post-chain-per-sample-work` has landed, rename `wdf_filter.rs` → `ladder_filter.rs` and `WdfLadderFilter` → `LadderFilter` in its own commit; verify render hashes of the default patch are unchanged and `internals_wiki` passes with the Source files links updated
- [ ] 8.4 Remove the "despite its name"/"still says" notes from the Internals pages whose discrepancy is now gone; verify `internals_wiki` passes

## 9. Close-out

- [ ] 9.1 `cargo fmt --check`, clippy `-D warnings`, `cargo test --workspace --no-default-features`, `cargo test --lib`, `no_alloc`, `internals_wiki`, `npm run check -- --complete` all pass
- [ ] 9.2 Remove the "Found while writing the Internals wiki" items from `docs/BACKLOG.md` (except Spread Width's help), update CLAUDE.md "Current state" (clips regenerated, presets moved), commit and push to `dev`

## Workflow follow-up

- Archive with `/opsx:archive` once every group is done; groups can be applied and committed one at a time.
