# Proposal

## Why

Five source files hold almost half the crate, and three of them are mostly tests (review finding O6, `FABLE-CODE-REVIEW-2026-11-14.md`; `docs/BACKLOG.md`). Current sizes on `dev` (2026-11-28):

| File | Lines | Of which tests |
|---|---|---|
| `src/gui/editor.rs` | 6 949 | 2 549 (from `:4401`) |
| `src/voice/mod.rs` | 4 555 | 2 561 (five modules from `:1995`) |
| `src/lib.rs` | 4 493 | 2 950 (nine modules from `:1544`) |
| `src/params.rs` | 3 159 | 149 (from `:3011`) |
| `src/dsp/resonators/core.rs` | 2 678 | 826 (from `:1853`) |

Files this size are slow to read and review, and every change touching the voice, the plugin glue or the editor lands in the same file. `src/kit/` and `src/dsp/` already keep tests in a sibling `tests.rs`.

## What Changes

- Move every inline test module of the five files into sibling files, keeping module paths and test names unchanged (`#[path]` attributes where a module isn't a child).
- Split `editor.rs` by panel into children of `src/gui/editor/` (widgets, header, preset browser and import/export, exciter column, resonator column, processing column, keyboard and kit bar), keeping `src/gui/editor.rs` as the module root.
- Split `voice/mod.rs` into envelope, expression and render children of `src/voice/`.
- Split `params.rs` into children of `src/params/` (enums and their parameters, the generated constructors), keeping `src/params.rs` as the root, so `src/params.rs` stays a valid path for docs and specs.
- `core.rs` only loses its tests: its hot loop stays in one file (see design.md).
- Mechanical: no behaviour change, render hashes unchanged, the same test list in both lanes, CPU within noise.
- Not **BREAKING**.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None: a pure refactor, so `.openspec.yaml` sets `skip_specs: true`.

## Impact

- Code: the five files above and about 20 new files under `src/gui/editor/`, `src/voice/`, `src/params/`, `src/tests/` and `src/dsp/resonators/`.
- Web: `web/editor` compiles `src/gui` with `web-gui`; it must still build (`web/scripts/build.sh`).
- Docs: every Internals page that names these files or functions in them (Repository-Layout, Editor, Parameters, Voices-and-Polyphony, Envelopes-and-Modulation, Note-Expression, Testing, Modal-Resonator and the others `grep -rl` finds); each new non-test file named in a `## Source files` list. `docs/ARCHITECTURE.md` and CLAUDE.md where they describe file layout.
- Every other in-flight change cites line numbers in these files; this one runs last (design.md, Ordering).
- Review: O6 marked FIXED, removed from `docs/BACKLOG.md`.
