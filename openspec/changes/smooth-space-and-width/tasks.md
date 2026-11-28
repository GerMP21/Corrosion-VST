# Tasks

## 1. Space Mode changes (R1)

- [ ] 1.1 Add post-chain tests for the three `space-effects` scenarios of "Changing Space Mode neither cuts nor replays a tail" (Factory re-entered after silence, Spring switched away, Echo not replaying); verify the re-entry and Echo tests fail on current `dev` (Factory reads about 0.053 RMS) and the switch-away test fails on its step bound.
- [ ] 1.2 Implement the 20 ms crossfade between outgoing and incoming modes and the clear of a processor once it has faded out (design decisions 1–2), rate-converted per the sample-rate rule; verify 1.1 passes and `cargo test --no-default-features --test no_alloc` passes with a mode change inside the checked region (extend the test if it doesn't cover one).
- [ ] 1.3 Measure the worst block time of a mode change with a one-shot clear and with the sliced clear; keep the cheaper adequate one and record both, with an *As built* note, in design.md Measurements.

## 2. Space Amount smoother (R2)

- [ ] 2.1 Add tests for "Space Amount moves without steps" (step between blocks glides; first set snaps); verify the glide test fails on current `dev`.
- [ ] 2.2 Add the smoother (target, primed flag, BODY's 20 ms coefficient) and reset the primed flag in `reset`; verify 2.1 passes and a static-amount render hash matches a clean worktree of `dev` (record it).

## 3. Spread Width from 0 (R3)

- [ ] 3.1 Add `hrtf_spread` tests for the three scenarios of "Spread Width is continuous from 0" (the review's 100 Hz table, leaving 0 after a level change, bit-identical at ≥ 0.05); verify the first two fail on current `dev` (−3.7 dB at 0.001).
- [ ] 3.2 Run the delay and filters at every Width and fade dry → processed over 0–0.05 (design decision 4); verify 3.1 passes and the "Hard-left input" test of `post-chain-routing` still passes.

## 4. Presets and docs

- [ ] 4.1 Run the bank tests (`presets::factory::tests`) and verify they pass; record in Measurements that the bank's render hashes with static controls are unchanged (one-off comparison against a clean worktree).
- [ ] 4.2 Update Internals `docs/internals/Body-Spread-and-Space.md` (mode crossfade and clear, Space Amount smoothed, Width fade; drop the "Unsmoothed controls" line for Space Amount) and `Post-Chain.md`, and `docs/ARCHITECTURE.md`; verify `cargo test --no-default-features --test internals_wiki` passes.
- [ ] 4.3 Update the user Wiki `output-chain/space.mdx` (switching modes fades; Amount glides under automation) and `output-chain/dispersion.mdx` (Width fades in from 0), English and Spanish, in the guide's voice; if a ⓘ help text changes, run `cargo run --bin export_guide_data` and update `parameters.es.json`. Verify `npm run check -- --complete` in `web/guide/` passes.
- [ ] 4.4 Mark R1, R2 and R3 FIXED in `FABLE-CODE-REVIEW-2026-11-14.md` with the date, this change and the proving tests; remove them from `docs/BACKLOG.md`. Verify by reading both.

## 5. Close-out

- [ ] 5.1 Run `cargo fmt --check`, clippy with `-D warnings`, `cargo test --workspace --no-default-features`, `no_alloc` and `internals_wiki`; verify all pass.
- [ ] 5.2 Update CLAUDE.md "Current state" (date, row, what's next, note for `widen-hrtf-spread`), commit on `dev` and push; verify `git status` is clean.

## Workflow follow-up

- Archive with `/opsx:archive`; `widen-hrtf-spread` then builds on the always-running spread stage.
