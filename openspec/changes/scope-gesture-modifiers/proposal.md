# Proposal: scope-gesture-modifiers

## Why

The Gesture Modifiers panel (Env Amount, Vel To Peak, Global Time, Vel To Level, Vel To Time, Curve) is drawn under every one of the 16 exciters, but its six controls only configure the friction envelope (MSEG): the Hit family's AD envelope and the Specialty family's ADSR ignore them. On a Hand Strike, the default patch, a player can move six faders and hear nothing. That breaks the `control-response` promise that every control on the panel acts, and the guide had to add "friction envelope" to every help text to explain it (`write-user-guide-content`).

## What Changes

Recommended option (the maintainer decides; design, Decision 1):
- The editor shows the Gesture Modifiers panel only when the selected exciter is a friction exciter (Bow, Stiff Point Scrape, Heavy Grinding, Corrugated Drag, Tension Rise), like the MSEG panel it belongs with.
- No sound changes; parameters, presets and projects are untouched. The host still lists the six parameters (they remain automatable and still act on friction notes).
- The exciter column of the Hit and Specialty exciters gets shorter; the editor layout tests and the guide screenshots follow.
- User Wiki (EN + ES: Gesture Modifiers page, editor page, Troubleshooting entry about faders that do nothing), Internals Editor and Envelopes pages updated.

Alternative (make them act on AD and ADSR): every Hit and Specialty note changes sound at today's default values, so every factory preset outside the friction family moves and the bank reseeds; not recommended.

## Capabilities

### New Capabilities
- (none)

### Modified Capabilities
- `editor-controls`: adds a requirement that the editor shows a control group only where its controls act, applied to the Gesture Modifiers.

## Impact

- Code: `src/gui/controls.rs` (`FIXED_CONTROLS` entries for the six parameters, `:1256-1297`, from `Always` to `Family(Friction)`), `src/gui/editor.rs` (`render_exciter_column`, the Gesture Modifiers block at `:2833-2895`, drawn only for the friction family). The web app runs the same editor code, so it follows.
- Tests: editor layout and help-coverage tests (GUI lane), `print_editor_content_height` if a height bound moves.
- Guide: screenshots retaken (the default patch is Hand Strike, so the `exciter` screenshot loses the panel).
- Relation to other changes: `env-amount-flat-envelope` changes what Env Amount does on friction notes; `add-release-snap` adds a friction-only fader to the MSEG panel with the same visibility rule. No ordering constraint, but whichever lands second retakes the screenshots once.
