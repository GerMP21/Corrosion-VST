# Tasks

## 1. Tests and the echo glide

- [ ] 1.1 Write the `space-effects` scenarios as tests in `space.rs` (Delay Time jump, synced division change through `sync_rate_to_delay_time`, Factory Size jump, Tank Size jump, read-speed cap, settles exactly, static bit-identity against stored hashes from the current code, no glide on load) and show the jump tests fail on current code; verify with `cargo test --no-default-features space`
- [ ] 1.2 Add the echo glide (50 ms one-pole on the delay in samples, capped at ±1 sample per sample, snapped when within 1e-3, primed on the first set and on reset and rate change), reading through the existing Hermite path while it moves; verify the echo tests pass and the existing echo tests (`Echo modulation glides`, `No modulation stays exact`, timing at every rate) pass
- [ ] 1.3 Render a 1 s Delay Time sweep and a 1/8 → 1/4 sync switch with the glide and with a 30 ms two-head crossfade (test-only), record both in design.md and ask the maintainer to confirm the glide by ear; verify the choice is recorded

## 2. The reverbs

- [ ] 2.1 Restructure the Factory combs and the Spring tank as full-capacity rings read at `write − delay`, with no glide yet; verify a render-hash comparison against a clean worktree (each Space mode, sizes 0.1/0.5/0.9, impulse and noise, 44.1/48/96 kHz) is identical and `cargo test --no-default-features space` passes
- [ ] 2.2 Add the glide to the comb and tank lengths, with linear interpolation while moving and the wet gain following the glided size; verify the Factory and Spring jump tests, "Factory combs stay distinct" and the spring timing and channel-independence tests pass
- [ ] 2.3 Run `cargo test --no-default-features --test no_alloc` and the `post_processing_chain` benchmark against the clean worktree (5 interleaved runs, median); verify no allocation and record the median in design.md

## 3. Documentation

- [ ] 3.1 Update `docs/ARCHITECTURE.md` (Space section) and `docs/internals/Body-Spread-and-Space.md` (rings with a read offset, the glide law and its cap, exact settling); verify `cargo test --no-default-features --test internals_wiki`
- [ ] 3.2 Update the Wiki's Space page in English and Spanish (`output-chain/space.mdx`, `es/output-chain/space.mdx`): Delay Time, Sync, Factory Size and Tank Size now glide like tape when moved, with a sound-design tip (automating Delay Time for pitch dives), in the guide's voice; verify `npm run check -- --complete` in `web/guide/`
- [ ] 3.3 Mark O5 FIXED in `FABLE-CODE-REVIEW-2026-11-14.md` and remove it from `docs/BACKLOG.md`; verify neither lists it as open

## 4. Close out

- [ ] 4.1 Run the bank tests (`cargo test --no-default-features presets::factory`); verify they pass with no preset moved
- [ ] 4.2 Run `cargo fmt --check`, clippy with `-D warnings`, `cargo test --workspace --no-default-features`, `no_alloc`, `internals_wiki`; verify all pass
- [ ] 4.3 Update CLAUDE.md "Current state" and commit and push to `dev`; verify the push

## Workflow follow-up

- Archive with `/opsx:archive glide-delay-length-controls`.
