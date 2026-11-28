# Tasks

## 1. Pad level (R7)

- [ ] 1.1 Add `src/kit/tests.rs` tests: a kit with `"level_db": 800` loads with a finite gain equal to +72 dB and re-serializes as 72; every factory drum kit's pad gains are unchanged (compare with the gains computed from the stored files). Verify the first fails on current `dev` (gain is `inf`).
- [ ] 1.2 Clamp `level_db` on deserialize, in `from_params` and in `build` (design decision 1); verify 1.1 passes.

## 2. Pad build errors (N9)

- [ ] 2.1 Add tests: a kit with pads on 36 and 38 where 38 holds `1e40` loads without panicking, 36 plays, 38 is silent; the editor's `load_pad` on that pad does nothing and doesn't panic. Verify the load test panics on current `dev`.
- [ ] 2.2 Make `params()` return a `Result` and skip non-finite stored values; make `build()` return `Option<Pad>`; `KitTable::load` writes `None` and logs the key; update the editor and web call sites (design decision 2). Verify 2.1 passes and `cargo test --lib` (GUI lane) passes.

## 3. File size bound (N11)

- [ ] 3.1 Add `presets` tests: importing a valid preset plus a 5 MB `.corrosion-preset` imports one and skips one; the user-folder scan skips the large file; a preset with a 128-pad kit imports. Verify the skip test fails on current `dev` (the file is read and fails to parse only after reading; assert via a sparse file or by checking that the error kind is the size error).
- [ ] 3.2 Add `MAX_PRESET_FILE_BYTES` and the metadata check plus a bounded read in `Preset::load` and `import_preset_files` (design decision 3); verify 3.1 passes.

## 4. Docs

- [ ] 4.1 Update Internals `docs/internals/Kit-Mode.md` and `Presets-and-Migrations.md` (pad level bound, a bad pad leaves its key empty, the 4 MB limit) and any page `grep -rl 'kit/mod.rs\|presets/mod.rs' docs/internals/` shows describing these paths; update `docs/ARCHITECTURE.md`. Verify `cargo test --no-default-features --test internals_wiki` passes.
- [ ] 4.2 Add a Troubleshooting line (EN + ES) for "an imported file was skipped" mentioning files over 4 MB, in the guide's voice; verify `npm run check -- --complete` in `web/guide/` passes.
- [ ] 4.3 Mark R7, N9 and N11 FIXED in `FABLE-CODE-REVIEW-2026-11-14.md` (date, change, tests) and update its Security section's list of exceptions; remove the three from `docs/BACKLOG.md`. Verify by reading both.

## 5. Close-out

- [ ] 5.1 Run `cargo fmt --check`, clippy with `-D warnings`, `cargo test --workspace --no-default-features`, `cargo test --lib`, `no_alloc` and `internals_wiki`; verify all pass.
- [ ] 5.2 Update CLAUDE.md "Current state", commit on `dev` and push; verify `git status` is clean.

## Workflow follow-up

- Archive with `/opsx:archive`.
