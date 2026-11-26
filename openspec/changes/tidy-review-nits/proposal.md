# Proposal: tidy-review-nits

## Why

The 2026-11-14 code review (`FABLE-CODE-REVIEW-2026-11-14.md`) left one Optional item (O8) and a set of Nits that make the code harder to read or debug without changing what anyone hears: a dead `match`, state that reads wrong in a debugger, a test-only public API, stale comments, per-sample work that only needs doing when the rate changes, empty vendor metadata, and safety clamps nobody has written down. Grouping them keeps the behaviour changes elsewhere small and reviewable.

## What Changes

- **O8:** move the integer-to-enum mapping of Space mode and Quality mode out of `apply_post_chain_params` (`src/lib.rs`) into `from_int` constructors beside `SpaceMode` and `PostQualityMode`, keeping today's fallbacks (Off and Normal).
- **N1:** replace the dead `match self.profile` (every arm returns `mode_sum`) in `src/dsp/resonators/core.rs` with `mode_sum`.
- **N2:** stop `rebuild_in_place` writing default values into `size_scale`, `rust_amount` and their siblings. Drop the fields if nothing outside tests reads them; otherwise store the values the bank was built with.
- **N3:** make `InteractionState::calculate_mode_coefficients` (public, allocating, test-only) `#[cfg(test)]`, and drop the "Removed 2026-05-28 (P3.1)" comments in `src/dsp/interaction.rs`.
- **N4:** inline the pass-through `process_exciter_stereo(_width)` wrapper in `src/voice/mod.rs` and rewrite the stale "never fires today" comment on the stereo high-pass boost.
- **N5:** remove the redundant `set_sample_rate(self.sample_rate)` calls on the wire brush, metal pipe and metal chain at note-on; `Voice::set_rate` already keeps them at the host rate.
- **N7:** join the MSEG's split doc comment (the `#[derive]` sits between its two halves), cache the envelope's `dt` when the rate changes instead of dividing every sample, and stop the voice calling `mseg.set_sample_rate` every sample.
- **N8:** set the plugin's `URL` to `https://corrosion-vst.org/`. `EMAIL` stays empty unless the maintainer names a project address (decision below).
- **N10:** document the stages' silent safety clamps (HRTF spread ±1.5, body ±10 per mode and ±1 overall, drive ±1, the filter's saturators) in `docs/ARCHITECTURE.md` and the Internals wiki, so nobody mistakes them for the diode stage.

N6 (the unreachable ADSR Sustain branch) belongs to `fix-specialty-adsr-release`, and N9 and N11 to `harden-preset-file-input`. O6 (splitting the large files) and O7 (merging the mono and stereo paths) are `split-largest-source-files` and `merge-mono-stereo-render-paths`; this change lands first or after them, never interleaved, because all three touch `voice/mod.rs` and `core.rs`.

No sound changes: every audio item must keep the render hashes bit-identical.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None. No requirement changes: the audio items are refactors proven bit-identical, N8 is vendor metadata no spec covers, and N10 is documentation. `.openspec.yaml` sets `skip_specs: true`.

## Impact

- Code: `src/lib.rs` (O8, N8), `src/dsp/post_processing/space.rs` and `post_chain.rs` (O8), `src/dsp/resonators/core.rs` and `src/dsp/tests.rs` (N1, N2), `src/dsp/interaction.rs` (N3), `src/voice/mod.rs` (N4, N5, N7), `src/dsp/envelopes/mod.rs` (N7).
- Hosts: the vendor URL now shows `https://corrosion-vst.org/` (CLAP `url`, VST3 vendor URL).
- Docs: `docs/ARCHITECTURE.md`, Internals pages `Plugin-Lifecycle`, `Post-Chain`, `Body-Spread-and-Space`, `Filter-and-Drive`, `Envelopes-and-Modulation`, `Interaction-Bus-and-Contact`, `Modal-Resonator`, `Voices-and-Polyphony`, and the review and backlog. No user Wiki change: nothing users hear or see moves.
