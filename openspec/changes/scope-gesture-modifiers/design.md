# Design: scope-gesture-modifiers

## Context

See proposal.md for why. Current code (`dev`, 2026-11-26):
- `render_exciter_column` draws, in order, the Model Type selector, the global knobs, the model's slot panel, the family's envelope panel (`render_envelope`, `src/gui/editor.rs:2901`), then the Gesture Modifiers panel unconditionally (`:2833-2895`).
- `FIXED_CONTROLS` lists the six faders with `Shown::Always` (`src/gui/controls.rs:1256-1297`); the friction MSEG faders already use `Shown::Family(Friction)` (`:1317-1338`), resolved at `:1629`.
- Only `configure_envelope`'s Friction arm reads the six values (`src/voice/mod.rs:841-877`); the Hit arm (`:822-830`) and Specialty arm (`:831-840`) take Attack/Decay/Sustain/Release only.
- The guide and the help texts say "friction envelope" and warn that the panel does nothing elsewhere (`web/guide/src/content/docs/editor.mdx:72`, `exciters/gesture-modifiers.mdx:10`); Internals says the same (`docs/internals/Envelopes-and-Modulation.md:120`).
- The web app sizes its canvas to the editor's measured content height each frame (`src/gui/editor.rs:682-701`), so a shorter exciter column can shrink the page when the tallest column is the exciter's.

## Goals / Non-Goals

**Goals:**
- No control on screen that can't change the note in front of the player.

**Non-Goals:**
- Making the AD or ADSR envelopes velocity- or curve-sensitive (Decision 1's alternative).
- Moving the panel inside the MSEG panel or restyling it.

## Decisions

### 1. Hide, or make them act (decide)

- **Recommended: hide for non-friction exciters.** Change the six entries to `Shown::Family(Friction)` and draw the block only when `exciter.family() == ExciterFamily::Friction`. No audio code changes, so every render is unchanged; no preset moves.
- **Alternative: act on AD and ADSR.** Env Amount, the velocity curves, Global Time and Curve would shape the one-shot and ADSR envelopes too. At today's defaults (Vel To Level 1, Vel To Time 0.15, Vel To Peak 0.5) that changes every Hit and Specialty note, so all 160-odd non-friction factory presets and the 10 kits move, the bank reseeds and every guide clip regenerates. Under this alternative the spec delta is instead an ADDED requirement in `mseg-envelope`'s neighbour (a new `force-envelopes` capability) stating how each modifier acts on each family, with bit-identity at a chosen neutral value, plus preset and host migrations setting that neutral value for old state.

Task 1 shows the maintainer the editor before and after (screenshots of Hand Strike, Pneumatic Jet and Bow) and records the choice.

### 2. Help texts

Under the recommendation the texts can drop "friction" in favour of "the envelope", since the panel now only appears with the MSEG. Kept as they are unless the maintainer prefers the shorter form; either way `npm run check` and the help-coverage test (two sentences at most) must pass.

### 3. Layout

The exciter column for Hit and Specialty models ends after its envelope panel. The editor's layout tests assert positions inside panels and the window's fit; they are re-run, and `print_editor_content_height` is recorded before and after for Hand Strike, Pneumatic Jet and Bow. If the editor's minimum height came from the exciter column, the window's base height (`BASE_LAYOUT_HEIGHT`) does not change: the plugin window keeps its size and the column simply has space below it.

## Risks / Trade-offs

- [A user who automates a Gesture Modifier on a Hit preset loses the fader] → the parameter still exists in the host; it never acted on that preset, so nothing audible is lost.
- [The web page's height jumps when switching families] → measured with the web app; if the jump is visible, the page keeps the largest height seen (a one-line change in the web shell), reported before doing it.

## Migration Plan

None: no parameter, preset or state change.
