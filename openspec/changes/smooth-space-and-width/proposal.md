# Proposal

## Why

Three audible faults sit in the Space and Spread stages of the post chain (`FABLE-CODE-REVIEW-2026-11-14.md` R1, R2, R3):
- **R1.** Only the selected Space processor runs; the others keep their buffers frozen. Switching Space Mode back to a mode replays the tail it held when you left it, even minutes later (Factory came back at about −25 dBFS from silence), and switching away cuts a ringing tail in one sample.
- **R2.** Space Amount has no smoother, unlike BODY, Drive, Output and the filter, so automating it zippers in steps of one host block.
- **R3.** Spread Width bypasses the stage below 0.001 and runs the proximity filters above it, so a 100 Hz mono tone drops 3.7 dB between Width 0 and 0.001, and re-entering from bypass starts from stale filter and delay state.

All three are clicks or jumps a user hears when moving or automating a control.

## What Changes

- A Space Mode change crossfades the outgoing and incoming wet signals over a short time (20 ms), and a processor that is no longer selected is cleared, so re-entering a mode always starts silent.
- Space Amount is smoothed per sample with the same 20 ms time constant as BODY, snapping on the first set after `new`/`reset`. Output is unchanged while the amount is static.
- Spread Width is continuous from 0: the spread's filters and delay always run, and the stage crossfades from the dry input to the processed output over Width 0–0.05. At Width ≥ 0.05 the output is unchanged (no factory preset uses a Width under 0.05).
- Wiki (EN + ES): Space (mode changes fade, Amount glides) and Dispersion (Spread Width fades in from 0) pages.
- Not BREAKING: no parameter changes meaning.

## Capabilities

### New Capabilities

(none)

### Modified Capabilities
- `space-effects`: adds that a Space Mode change neither cuts nor replays a tail, and that Space Amount moves without steps.
- `post-chain-routing`: adds that Spread Width is continuous from 0.

## Impact

- Code: `src/dsp/post_processing/post_chain.rs` (mode-change crossfade and clearing, Space Amount smoother), `src/dsp/post_processing/hrtf_spread.rs` (always-on filters, Width fade), possibly a bounded incremental clear in `src/dsp/post_processing/space.rs`.
- CPU: during a 20 ms mode crossfade two Space processors run; the spread's filters run at Width 0 (a handful of multiply-adds per sample).
- Presets: none uses Spread Width < 0.05; static Space Amount renders bit-identical, so the bank should not move. Checked by the bank tests.
- Ordering: `widen-hrtf-spread` (the mono remix and Spread Width's help) rewrites the processed spread path; this change only makes the path continuous at 0 and should land first, so that change builds on a stage whose state is always current.
- Docs: Internals `Body-Spread-and-Space.md` and `Post-Chain.md`, `docs/ARCHITECTURE.md`, user Wiki `output-chain/space.mdx` and `output-chain/dispersion.mdx` (EN + ES).
- Review: R1, R2, R3 marked FIXED; removed from `docs/BACKLOG.md`.
