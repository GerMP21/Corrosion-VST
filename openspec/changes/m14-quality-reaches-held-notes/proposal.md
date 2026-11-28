# Proposal: m14-quality-reaches-held-notes

## Why

M14 of the 2026-07-07 review (`docs/archive/FABLE-CODE-REVIEW-2026-07-07.md`, left open in `docs/BACKLOG.md`): Quality has two parts that change at different times. The diode stage's oversampling switches at once; a voice's mode count (0.5× Eco to 2× Render) is fixed at note-on. `m10-m18-tidy-dead-code-and-docs` documented this, and the Wiki's Quality page says it. What is left is a decision: is "held notes keep their density" fine, or should a Quality change reach them?

It matters in one situation: a user drops to **Eco because the CPU is overloading** while long drones or ringing Pipe and I-Beam notes are held. Today those notes keep costing their full bank until they end, which can be seconds, or forever in Drone mode, so the switch doesn't relieve the overload it was meant to.

## What Changes

Decision for the maintainer (task group 1 demonstrates both):
- **A (recommended): lowering Quality thins held notes.** On a switch to a lower quality, every held or releasing voice fades out the modes beyond the new mode count over 20 ms and stops computing them, so CPU drops at once and nothing clicks. Raising Quality doesn't touch held notes (new modes can't be excited mid-note without a fake attack); the next notes get the richer bank, as today.
- **B: keep it documented.** No code change; the behaviour is already written in the README, ARCHITECTURE, the Internals wiki and the Wiki's Quality page. A test pins it so it stays deliberate.

Under A, sound changes only at the moment of a downward Quality switch on held notes; renders at a fixed quality are bit-identical. Not BREAKING.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `modal-bank`: under A, adds "Lowering Quality thins held notes without a click" and modifies "Retuned notes keep every mode they can reach", whose "a mode kept by the bank stays for the whole note" would otherwise forbid dropping modes. Under B, the delta becomes one ADDED requirement stating that a held note keeps its mode count until it ends.

## Impact

- Code (A): `src/voice/manager.rs` and `src/voice/mod.rs` (detecting a quality change and telling the voices), `src/dsp/resonators/core.rs` (a per-voice active-mode limit with a short fade), `src/lib.rs` (passing the live quality's scale to the voice manager).
- Real-time: the trim must not allocate (it shortens the processed range, it doesn't rebuild); `no_alloc` gains a Quality switch with voices held.
- Docs: README, `docs/ARCHITECTURE.md` (Quality modes), Internals `Modal-Resonator` ("Mode count and quality") and `Voices-and-Polyphony`, Wiki `quality.mdx` (EN + ES).
- Related: `smooth-quality-switch-and-stereo-factory` (O9) makes the clipper half of a Quality switch click-free; apply this change after it so both halves share one switch path.
