# Design

## Context

See proposal.md for why and the line counts. Constraints:

- The default release profile has 16 codegen units and no LTO (`Cargo.toml` sets only `[profile.test]`). Moving functions between modules can change how they're partitioned, and CLAUDE.md records that code layout alone has swung the plain benchmarks by 30 % with bit-identical output.
- `tests/internals_wiki.rs` (`is_test_file`, `:223-227`) exempts `tests.rs`, `*_tests.rs` and anything under a `tests/` directory from the "every source file is named" check; every other new file must be named by an Internals page.
- `crate::host_rate_and_latency_tests` is used from `src/kit/tests.rs:275, 366, 369`, so that module's path must not change.
- Test names are used as filters in CLAUDE.md, the docs and the ignored measurement tests (`preset_levels_are_sane`, `print_48k_exciter_render_hashes`, …).
- Child modules can read the parent's private fields, so `impl Voice` blocks in `src/voice/envelope.rs` need no visibility changes.

## Goals / Non-Goals

**Goals:**
- Smaller files with one concern each; tests next to, not inside, the code.
- Zero behaviour change, proven.

**Non-Goals:**
- Renaming, re-ordering logic or "while I'm here" fixes: those belong in the behaviour changes.
- Splitting `core.rs`'s non-test code. `SecondOrderMode` and `ModalResonator::process_sample` are the per-sample hot loop; keeping them in one file keeps their codegen-unit placement as it is (CLAUDE.md, "keep the per-sample loop's data small").
- Merging the mono and stereo paths (O7, `merge-mono-stereo-render-paths`).

## Decisions

### 1. Roots keep their paths

`src/gui/editor.rs`, `src/params.rs` and `src/voice/mod.rs` stay as module roots, with children in `src/gui/editor/`, `src/params/` and `src/voice/`. Every doc, spec and CLAUDE.md reference to those paths stays valid. Alternative: `editor/mod.rs`, `params/mod.rs`. Rejected for the churn in docs and specs.

### 2. The split plan

| From | To |
|---|---|
| `lib.rs` `:1544-4493` (9 test modules) | `src/tests/` one file per module: `mod.rs` (`tests`), `c5_measurements.rs`, `output_ceiling_tests.rs`, `host_state_tests.rs`, `host_rate_and_latency_tests.rs`, `midi_control_tests.rs`, `voice_mix_dc_tests.rs`, `note_expression_tests.rs`, `editor_keyboard_tests.rs`; declared in `lib.rs` with `#[cfg(test)] #[path = "tests/<file>.rs"] mod <name>;` so `crate::<name>` paths don't change |
| `voice/mod.rs` `:1995-4555` (5 test modules) | `src/voice/tests.rs`, `material_transform_tests.rs`, `metal_chain_tests.rs`, `adsr_release_tests.rs`, `contact_control_tests.rs` |
| `voice/mod.rs` `:475-500`, `:811-1001` | `src/voice/envelope.rs`: envelope phases, `configure_envelope`, `process_force_envelope`, `release_force_envelope` |
| `voice/mod.rs` `:1348-1475` | `src/voice/expression.rs`: pitch bend, tuning, address, choke, pressure, mod wheel, `expression_gain` |
| `voice/mod.rs` `:1632-1988` | `src/voice/render.rs`: rattle, contact, governor, `process_exciter`, `process_sample`, `process_sample_stereo` |
| `voice/mod.rs` rest | stays: `VoiceControls`, `Voice`, constants, `new`, note on/off, live controls (about 1 100 lines) |
| `params.rs` `:848-1535` | `src/params/enums.rs`: `ExciterType`, `ExciterFamily`, `QualityMode`, `PlayMode`, `Object` and their parameter constructors |
| `params.rs` `:1538-2800` | `src/params/constructors.rs`: the generated per-parameter constructors shared with `Preset::into_params` |
| `params.rs` `:3011-3159` | `src/params/tests.rs` |
| `params.rs` rest | stays: `CorrosionParams` (its `#[derive(Params)]` needs every field in one struct) and `Default` |
| `editor.rs` `:4401-6949` | `src/gui/editor/tests.rs` |
| `editor.rs` `:1016-1810` | `src/gui/editor/widgets.rs`: screws, knobs, faders, combo, badges, panels, headings |
| `editor.rs` `:1880-2238` | `src/gui/editor/header.rs` |
| `editor.rs` `:480-595`, `:2239-2711` | `src/gui/editor/browser.rs`: preset menu, import/export, loader, factory folders |
| `editor.rs` `:2712-3158` | `src/gui/editor/exciter_column.rs` (with the envelope panel) |
| `editor.rs` `:3159-3398` | `src/gui/editor/resonator_column.rs` |
| `editor.rs` `:3399-3884` | `src/gui/editor/processing_column.rs` (with the macro strip and footer) |
| `editor.rs` `:3885-4400` | `src/gui/editor/keyboard.rs`: keyboard strip, kit bar, key mapping |
| `editor.rs` rest | stays: `EditorUiState`, `create_editor`, `WebEditor`, layout scale, style, `render_ui` (about 1 100 lines) |
| `core.rs` `:1853-2678` | `src/dsp/resonators/core_tests.rs` |

Line ranges are as of 2026-11-28; the change re-reads them when it starts, since it runs after the behaviour changes.

### 3. Proofs

- **Same tests:** `cargo test --workspace --no-default-features -- --list` and `cargo test --lib -- --list` before and after, sorted and diffed: identical.
- **Same sound:** the render hashes from `print_48k_exciter_render_hashes` and the bank's `generated_bank_equals_the_files_on_disk`, against a clean worktree.
- **Same speed:** `scripts/bench_ratios.py 5` and the plain 16-voice and plugin-path benchmarks, base and change interleaved, medians of 5, with the benchmark output hashes compared so a layout swing isn't mistaken for a regression. If a hot function's placement moves the numbers beyond noise, mark it `#[inline]` as it effectively was, and record it.
- Each file moves in its own commit, so a regression bisects to one move.

### Ordering

Last among the review's changes: after every behaviour change that edits these files (`fix-specialty-adsr-release`, `tidy-review-nits`, `smooth-space-and-width`, `trim-post-chain-per-sample-work`, `m14-quality-reaches-held-notes`, `scope-gesture-modifiers`, `mseg-pingpong-loop`, …) and after `merge-mono-stereo-render-paths`, so it moves final code once and doesn't invalidate the file:line citations in their designs.

## Risks / Trade-offs

- [Codegen-unit placement changes the hot path's speed] → the benchmark proof above; `core.rs`'s hot loop doesn't move.
- [A test module's path changes and a cross-module helper breaks] → `#[path]` keeps paths; the `--list` diff catches renames.
- [Docs keep citing old line numbers or functions in the wrong file] → task group 3 greps every Internals page and doc for the moved files and functions.
- [The web editor breaks on a `cfg` the plugin build doesn't exercise] → `web/scripts/build.sh` in the close-out.

## Migration Plan

None.

## Measurements

To fill in: the line counts after the split, the `--list` diffs (empty), the render-hash comparison, and the benchmark medians with output hashes.
