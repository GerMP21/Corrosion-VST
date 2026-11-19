# Corrosion Internals

How Corrosion works inside, for developers. Corrosion is a physical-modelling instrument plugin (VST3 and CLAP) written in Rust on NIH-plug: a MIDI note excites a bank of resonant modes through one of 16 exciter models, and the sound passes through a post chain of filter, drive, body, spread, Space, a diode stage and a limiter. The same engine and editor also run in the browser as the web app.

## How to read this wiki

- **New to the code?** Start with [Repository layout](Repository-Layout), then [Signal chain](Signal-Chain): together they show where everything lives and how a note becomes sound. Every other page goes deeper into one stage.
- **Looking for a file?** Every page ends with a **Source files** list, so searching the wiki for a path finds the pages that explain it.
- **Unfamiliar term?** See the [Glossary](Glossary).
- **Want the history behind a decision?** `docs/ARCHITECTURE.md` in the repository is the as-built record, with measurements and the changes that shaped the code; `openspec/changes/archive/` holds each change's proposal, design and numbers. This wiki explains how things work today.
- **Playing the instrument rather than changing it?** The [Corrosion Wiki](https://corrosion-vst.org/wiki/) is the user guide.

## Page map

### Start

- [Repository layout](Repository-Layout): the two Cargo workspaces, features, targets, and what every directory holds.
- [Signal chain](Signal-Chain): a note's path from MIDI to the limiter, and the per-sample voice loop.

### Plugin

- [Plugin lifecycle](Plugin-Lifecycle): NIH-plug wiring: construction, `initialize`, `process`, MIDI and CCs, state migrations.
- [Parameters](Parameters): how parameters are declared, named, displayed and smoothed, and which act live.
- [Voices and polyphony](Voices-and-Polyphony): the 16-voice pool: note-on, release, tails, stealing, sustain and panic.
- [Note expression](Note-Expression): voice IDs, PolyTuning, pressure, choke.
- [Kit mode](Kit-Mode): per-key pads and the lock-free `KitTable`.

### Engine

- [Modal resonator](Modal-Resonator): the mode bank, its per-sample loop and how it is built and retuned.
- [Objects](Objects): the nine objects and how each computes its modes.
- [Exciters](Exciters): the machinery all 16 exciters share: dispatch, drive levels, the governor.
- [Hit exciters](Hit-Exciters): the seven struck models.
- [Friction exciters](Friction-Exciters): the five friction models and the release snap.
- [Specialty exciters](Specialty-Exciters): Pneumatic Jet, Electromagnetic Hum, Tension Snap, Particle Rain.
- [Interaction bus and contact](Interaction-Bus-and-Contact): the two-way exciter–object loop and implicit contact.
- [Envelopes and modulation](Envelopes-and-Modulation): AR, ADSR, MSEG, gesture modifiers, position envelope and wander.
- [Material transformations](Material-Transformations): Rust, Thickness, Heat, Sludge and Damage.
- [DSP utilities](DSP-Utilities): the DC blocker, noise and the smaller helpers.
- [Sample-rate rule](Sample-Rate-Rule): why constants are written at 48 kHz and how they are converted.

### Post chain

- [Post chain](Post-Chain): every stage after the voice mix, quality modes and latency.
- [Filter and drive](Filter-and-Drive): the ladder filter, the master drive and the Lorenz drive.
- [Body, spread and Space](Body-Spread-and-Space): the FEM body, the stereo spread and the three Space modes.
- [Output protection](Output-Protection): half-band oversampling, the diode stage and the limiters.

### State

- [Presets and migrations](Presets-and-Migrations): the preset format, both migration paths, the browser, import and export.
- [Factory bank](Factory-Bank): the 214 factory presets built in code, the seeder and its levelling.
- [Randomizer](Randomizer): the four randomizer modes and their safety rules.

### Front ends

- [Editor](Editor): the egui editor: layout, controls, help, fonts and the keyboard.
- [Web app](Web-App): the browser build: two wasm modules, the AudioWorklet and the bridge.
- [Site and user Wiki tooling](Site-and-User-Wiki-Tooling): the site, the user Wiki's build, generated data, clips and screenshots.

### Quality

- [Real-time safety](Real-Time-Safety): the audio-thread contract and how the tests enforce it.
- [Testing](Testing): the test lanes, integration tests, fixtures and offline renders.
- [Benchmarks and performance](Benchmarks-and-Performance): benchmarks, how to measure honestly, and the CPU rules.
- [Building and bundling](Building-and-Bundling): targets, bundles, validators, vendored crates and the notices.
- [CI and releases](CI-and-Releases): every CI job, the nightly release and the wiki publish.

### Process

- [Contributing and keeping this wiki current](Contributing-and-Keeping-This-Wiki-Current): branches, the OpenSpec workflow and the rule that keeps this wiki true.
- [Glossary](Glossary): the terms these pages use.

## Keeping it true

These pages live in `docs/internals/` in the repository and are published here from `main`; edits made on GitHub are overwritten. Every change to the code updates the pages that describe it, in the same change, and a test checks that every source file is named by a page and that every link resolves. See [Contributing and keeping this wiki current](Contributing-and-Keeping-This-Wiki-Current).
