# Tasks

## 1. Decision

- [ ] 1.1 Render A/B clips of the 9 friction presets that store Loop Mode 2 and of the default Bow patch with Ping-Pong 3 → 4 and 1 → 3 (today vs a prototype of the new loop), and get the maintainer's choice (implement, as specified, or remove, per design.md "If the maintainer chooses removal"); record the choice in design.md

## 2. Tests first

- [ ] 2.1 Add envelope tests for each scenario of the `mseg-envelope` delta (triangle 3 → 4, Start/End honoured 1 → 3, differs from Forward with only Forward stepping, degenerate region equals Off, release from a backward stage) and verify they fail on current code (`cargo test --no-default-features --lib envelopes`)
- [ ] 2.2 Confirm the Off and Forward rows of `loop_output_matches_golden` and `every_loop_configuration_is_bounded` still pass unchanged after 2.1 (they are the bit-identity proof for the other modes)

## 3. Implementation

- [ ] 3.1 Add the loop direction state to `MSEG` (reset on `trigger`), mirror `progress` for backward stages and walk the region in `advance_stage`, keeping the `MAX_TRANSITIONS` cap; verify 2.1 passes and `default_and_drone_loops_never_hit_transition_cap` passes
- [ ] 3.2 Re-pin the `pingpong` row of `loop_output_matches_golden` with the reason in its doc comment; verify the Off and Forward rows didn't change
- [ ] 3.3 Run `cargo test --no-default-features --test mseg_loop_safety` and `--test no_alloc`; both pass

## 4. Help, docs and Wiki

- [ ] 4.1 Update the `loop_mode` doc in `src/params.rs`, the ("loop", "Ping-Pong") help in `src/gui/controls.rs`, and `README.md` (both mentions); verify `grep -rn -i "pingpong.*partial\|for now the same" src README.md` finds nothing
- [ ] 4.2 Update `docs/ARCHITECTURE.md` and the Internals pages `Envelopes-and-Modulation.md` (Looping table) and `Parameters.md`; verify `cargo test --no-default-features --test internals_wiki` passes
- [ ] 4.3 Update the Wiki's `exciters/envelopes.mdx` (EN and ES, wiki style) with what Ping-Pong does and a tip; run `cargo run --bin export_guide_data`, fix `parameters.es.json`, and verify `npm run check -- --complete` in `web/guide/` passes
- [ ] 4.4 Regenerate the guide clips (`scripts/build-guide-clips.sh`: the `loop_mode` clips move) and verify `guide_data_equals_the_files_on_disk` passes

## 5. Presets

- [ ] 5.1 Run the bank tests (`presets::factory::tests`); reseed with `cargo run --release --bin seed_presets`; record which presets moved and by how much (`curated_measurements_table` and the seeder's levels) in design.md `## Measurements`; verify `generated_bank_equals_the_files_on_disk` passes

## 6. Close-out

- [ ] 6.1 `cargo fmt --check`, `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`, `cargo test --workspace --no-default-features`, `cargo test --lib` all pass
- [ ] 6.2 Remove the MSEG PingPong item from `docs/BACKLOG.md`, update CLAUDE.md "Current state", commit and push to `dev`

## Workflow follow-up

- Archive with `/opsx:archive` after the maintainer listens to the reseeded presets.
