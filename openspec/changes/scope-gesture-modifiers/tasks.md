# Tasks

## 1. Decide (maintainer)

- [ ] 1.1 Take editor screenshots of Hand Strike, Pneumatic Jet and Bow with the panel hidden for non-friction exciters (a local build of the change's first commit, or the web app) next to today's; verify the maintainer's choice of Decision 1 is recorded in design.md (*As built*), and if they choose the alternative, stop and rewrite this change's specs and tasks before continuing

## 2. Tests first

- [ ] 2.1 Write the `editor-controls` scenarios as GUI-lane tests: the panel and its six faders drawn for each friction exciter, none for each of the other eleven, and an automated Curve value shown after switching to Bow; verify the "not drawn" test fails on current code
- [ ] 2.2 Record `print_editor_content_height` for Hand Strike, Pneumatic Jet and Bow before the change in design.md `## Measurements`

## 3. Implement

- [ ] 3.1 Change the six `FIXED_CONTROLS` entries to `Shown::Family(Friction)` and draw the Gesture Modifiers block only for the friction family in `render_exciter_column`; verify 2.1 passes and the help-coverage and layout tests pass (`cargo test --lib`)
- [ ] 3.2 Record `print_editor_content_height` after the change; verify the plugin window's size is unchanged and note the column heights
- [ ] 3.3 Check the web app with the change (switch Hand Strike ↔ Bow); verify the page height doesn't jump visibly, or report per design's risk before changing the web shell
- [ ] 3.4 Verify no audio path changed: `print_48k_exciter_render_hashes` equals a clean worktree's

## 4. Guide, Wiki and docs

- [ ] 4.1 Retake the guide screenshots (`node web/scripts/screenshots.cjs`) and fix the callouts and alt text of the exciter screenshot in English and Spanish; verify the images and `npm run check -- --complete` pass
- [ ] 4.2 Update the user Wiki in English and Spanish: Gesture Modifiers page (the panel now appears with the friction exciters only), editor page (item 5 of the exciter column), and Troubleshooting if an entry says the panel does nothing; verify `npm run check -- --complete` passes
- [ ] 4.3 Run `cargo run --bin export_guide_data` if any help text changed (Decision 2) and update the Spanish entries; verify `guide_data_equals_the_files_on_disk` passes
- [ ] 4.4 Update the Internals pages (`docs/internals/Editor.md` column list and control table, `Envelopes-and-Modulation.md` "Trap" paragraph, `Parameters.md` if it mentions the panel) and `docs/ARCHITECTURE.md` if it describes the panel; verify `cargo test --no-default-features --test internals_wiki` passes
- [ ] 4.5 Remove the Gesture Modifiers item from `docs/BACKLOG.md`; verify it is gone

## 5. Close-out

- [ ] 5.1 Run fmt, clippy `-D warnings`, `cargo test --workspace --no-default-features`, `cargo test --lib`, and `cargo doc`; verify all pass and record the counts (the GUI clippy lane adds no new `falling back to f32` warnings)
- [ ] 5.2 Update "Current state" in `CLAUDE.md`, commit on `dev` and push; verify `git log origin/dev` shows it

## Workflow follow-up

- Archive with `/opsx:archive`; the `editor-controls` delta syncs into `openspec/specs/`.
