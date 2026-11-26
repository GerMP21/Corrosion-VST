# Design: tidy-review-nits

## Context

Each item was re-checked on `dev` (2026-11-26); the review's line numbers still hold:

| Item | Where | Confirmed |
|---|---|---|
| O8 | `src/lib.rs:1077-1092` | Literal `match` arms 0..3 for `space_mode` (fallback Off) and `quality_mode` (fallback Normal). `SpaceMode` is in `space.rs:7`, `PostQualityMode` in `post_chain.rs:16`; the params enums already have `from_int` (`params.rs:907`, `:1222`, …). |
| N1 | `src/dsp/resonators/core.rs:1753-1763` | Nine arms, all `mode_sum`. |
| N2 | `core.rs:1238-1243` | `rebuild_in_place` writes `SizeScale::default()`, `RustAmount::default()`, … The only readers are tests that pin the defaults (`src/dsp/tests.rs:450-451`, `:497`, `:576`). `src/offline/mod.rs` reads `rust_amount` of a spec or artifact, not of the resonator. |
| N3 | `src/dsp/interaction.rs:109` | `pub fn calculate_mode_coefficients` allocates a `Vec`; one caller, a test at `:268`. P3.1 comments at `:17-21`, `:125-127`, `:214-216`. |
| N4 | `src/voice/mod.rs:1891-1893`, `:1943-1944` | Wrapper ignores `_width`; the comment talks about a `0` branch that no longer exists. |
| N5 | `voice/mod.rs:1203`, `:1213`, `:1224` | `Voice::set_rate` (`:1606-1625`) already calls `set_sample_rate` on all three when the rate changes; each exciter's `set_sample_rate` only stores the rate (e.g. `wire_brush.rs:83`). |
| N7 | `src/dsp/envelopes/mod.rs:5-11`, `:260`; `voice/mod.rs:957` | Doc split by `#[derive]`; `let dt = 1.0 / self.sample_rate;` per sample; `self.mseg.set_sample_rate(self.sample_rate)` per sample. The note-on path builds a fresh `MSEG` and sets its rate once (`voice/mod.rs:842-843`), which is the call to keep. |
| N8 | `src/lib.rs:1385-1386` | `URL` and `EMAIL` are `""`. Internals `Plugin-Lifecycle.md:44` and `:225` describe them as empty. |
| N10 | `hrtf_spread.rs:109`, `fem_body.rs:104` and `:112`, `lorenz_drive.rs:183`, `:190`, the filter's stage saturators | Clamps exist and are undocumented in ARCHITECTURE. |

## Goals / Non-Goals

**Goals:** readable code with no dead branches or misleading state; per-sample work only where it must be; the vendor URL set; the clamps written down. Every audio item bit-identical.

**Non-Goals:** removing or changing any clamp (N10 is documentation only); the file splits (O6) and the mono/stereo merge (O7); N6, N9, N11 (other changes).

## Decisions

1. **O8 as `from_int` on the DSP enums**, matching `Object::from_int` and the params enums, rather than `From<i32>`: the codebase already uses `from_int`, and an inherent function can document its fallback. The fallbacks stay exactly as they are (Off, Normal) so hosts sending out-of-range values behave as today. Unit tests pin every value and the fallback.
2. **N2: drop the fields** if, after the change, nothing but the three tests reads them. Storing the real values would add six writes per note-on for state no code uses. The tests that pinned the defaults are rewritten to check what matters (the bank), or removed if they only checked the fields. If a reader turns up during implementation, store the real values instead and note it *As built*.
3. **N7: cache `dt` in `MSEG::set_sample_rate`.** `1.0 / sample_rate` computed once gives the same `f32` as computing it every sample, so the envelope is bit-identical. The note-on path already sets the fresh MSEG's rate (`voice/mod.rs:843`); a rate change during a held note is handled by also calling `mseg.set_sample_rate` from `Voice::set_rate`'s rate-changed branch, not per sample.
4. **N8: `URL = "https://corrosion-vst.org/"`.** The site's only domain is `corrosion-vst.org` (CLAUDE.md). `EMAIL` is a maintainer decision: leave it empty (hosts show nothing, as today) or set a project address the maintainer gives. This design does not invent one; the default is empty.
5. **Proof of bit-identity:** `tests/tidy_render_hash.rs` (`print_tidy_render_hashes`, ignored) already renders damaged stereo friction and hit notes; extend it with a Specialty exciter, a Drone note (friction MSEG) and the post chain with Space on, then compare its hashes and `print_48k_exciter_render_hashes` (`src/voice/mod.rs:2949`) against a clean worktree of the base commit, gnu target.

## Risks / Trade-offs

- [A "tidy" item changes rounding (for example N7 reorders arithmetic)] → the hash comparison catches it; any item that can't be made bit-identical is dropped from this change and reported, not merged with a sound change.
- [Conflicts with `split-largest-source-files` / `merge-mono-stereo-render-paths`, which move the same code] → apply these changes one at a time; whichever lands second rebases. Prefer landing this one first: it is smaller.
- [A host caches vendor metadata] → harmless; the URL shows after a rescan.

## Migration Plan

None: no parameter, preset or project format changes.

## Measurements

To fill in during implementation: render hashes before and after (gnu), per scenario; `cargo test` counts.
