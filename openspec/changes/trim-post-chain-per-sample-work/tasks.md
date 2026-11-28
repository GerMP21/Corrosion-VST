# Tasks

## 1. Baseline and O1: configure on change

- [ ] 1.1 Audit every post-chain stage and both limiters for state that only `set_sample_rate` refreshes (the filter's tolerance-0 coefficient is one), list them in design.md Measurements; verify each listed item has a test in 1.3 or a reason it can't matter
- [ ] 1.2 Add the benchmark twins `post_processing_chain_steady` and `post_processing_chain_body_zero` (chain built outside the timed loop) and `voice_manager_sixteen_voices_wander`, record a clean-worktree baseline (5 runs, median) in design.md; verify `cargo bench --no-default-features --bench performance -- post_processing_chain` runs them
- [ ] 1.3 Write the `realtime-safety` and `post-chain-filter` "Tolerance back to 0" scenarios as tests (bit-identical steady render with and without per-block reconfiguration via a test-only switch; rate change 48 → 96 kHz; tolerance 1 → 0 restores the exact cutoff) and show the tolerance test fails on a build with only the guard; verify with `cargo test --no-default-features post_chain` and `wdf`
- [ ] 1.4 Cache the configured rate in `Corrosion` (`Option<u32>`, cleared in `initialize` and `reset`) and call the post chain's and limiters' `set_sample_rate` only on change; make `WdfLadderFilter::set_parameters` refresh its coefficients when the tolerance changes; verify the 1.3 tests pass and a render-hash comparison against the clean worktree (legacy `sheet_lightning`, the 16 exciters' defaults, automation of tolerance and Space mode) is identical
- [ ] 1.5 Update `docs/internals/Post-Chain.md` and `Filter-and-Drive.md` (when the chain is configured; the filter's tolerance refresh) and `docs/ARCHITECTURE.md`; verify `cargo test --no-default-features --test internals_wiki`

## 2. O2: the body

- [ ] 2.1 Write the `post-chain-routing` scenarios as tests (BODY raised from 0 starts from rest; BODY 2 bit-identical through a test-only switch to the old per-sample arithmetic; BODY 0 equals no body) and show the first fails on current code; verify with `cargo test --no-default-features fem_body post_chain`
- [ ] 2.2 Precompute `a = 2 − ω² − decay`, `b = 1 − decay`, `material_gain` and `1/(i+1)` per mode in `update_mode_frequencies`/`update_decay`, keeping today's multiplication order; verify the BODY 2 bit-identity test passes
- [ ] 2.3 Skip and reset the body while BODY and its smoothed value are both 0; verify the 2.1 tests pass and `tests/body.rs` passes
- [ ] 2.4 Render a clip of BODY automated from 0 to 2 on a held Pipe note, before and after, and record the verdict by ear in design.md; if it is worse, keep the body running at BODY 0 (the 2.2 savings stay); verify the decision is recorded
- [ ] 2.5 Update `docs/internals/Body-Spread-and-Space.md` and `docs/ARCHITECTURE.md` (body costs nothing at BODY 0, starts from rest); if users can hear the restart, add a line to the BODY text in the Wiki in English and Spanish; verify `internals_wiki` and, if touched, `npm run check -- --complete`

## 3. O3: filter drift at a control rate

- [ ] 3.1 Write the `post-chain-filter` "Control-rate drift tracks the per-sample drift" and "No tolerance is unchanged" scenarios as tests, with a test-only switch for the refresh interval, and show the interval switch changes nothing on current code (reference); verify with `cargo test --no-default-features wdf`
- [ ] 3.2 Refresh the drifted coefficients every 16 samples while cutoff and resonance are settled, advancing the drift every sample; verify the 3.1 tests pass and the existing tolerance tests (`drift_spread_db`, reset-from-silence) pass
- [ ] 3.3 Record the 50 ms level differences and the twin's median gain in design.md, and update `Filter-and-Drive.md`; verify `internals_wiki`

## 4. O4: strike-position weights

- [ ] 4.1 Write the `strike-position` scenarios as tests (weights within 1e-5 of `sin(nπp)` in f64 at every sample of a wandering Render-quality Pipe note, with p swept over 0..1 including the ends; still position bit-identical; wandering output difference ≥ 80 dB under the RMS against a test-only switch to the old path) and show the accuracy test passes on current code (a reference); verify with `cargo test --no-default-features interaction strike`
- [ ] 4.2 Compute the moving position's weights with the f64 recurrence, keeping the cache and the fundamental-lock minimum; verify the 4.1 tests and `cargo test --no-default-features --test no_alloc` pass
- [ ] 4.3 Record the worst weight error and the wander twin's median gain in design.md, and update `docs/internals/Interaction-Bus-and-Contact.md`; verify `internals_wiki`

## 5. Integration and close-out

- [ ] 5.1 Run the plugin-path benchmarks and the twins against the baseline, five interleaved runs, and record the medians with matching output hashes in design.md; update `docs/internals/Benchmarks-and-Performance.md` for the new twins; verify the `realtime-safety` "no slower" scenario holds
- [ ] 5.2 Run the bank tests (`cargo test --no-default-features presets::factory`); verify they pass with no preset moved
- [ ] 5.3 Mark O1–O4 FIXED in `FABLE-CODE-REVIEW-2026-11-14.md` and remove them from `docs/BACKLOG.md`; verify neither lists them as open
- [ ] 5.4 Run `cargo fmt --check`, clippy with `-D warnings`, `cargo test --workspace --no-default-features`, `no_alloc`, `internals_wiki`, `cargo doc --no-deps --no-default-features`; verify all pass
- [ ] 5.5 Update CLAUDE.md "Current state" and commit and push to `dev`; verify the push

## Workflow follow-up

- Archive with `/opsx:archive trim-post-chain-per-sample-work`.
