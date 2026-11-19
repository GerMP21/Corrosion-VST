# Glossary

Terms the other pages use, as this code uses them. Where a term belongs to one part of the system, the page that explains it is linked.

## Engine

- **Mode**: one damped resonance of an object, given by its frequency, decay and gain (`ModalModeSpec`). It runs per sample as a `SecondOrderMode`. See [Modal Resonator](Modal-Resonator).
- **Mode bank, modal bank**: the list of modes one note rings. `ModalResonator::rebuild_in_place` builds it at note-on, in buffers allocated once for `MAX_MODES` (96).
- **Coupled form**: how a mode is computed, as a complex one-pole filter `z = p·z + g·x`. Retuning rotates its state without adding energy.
- **Generator**: an object's `ResonatorAlgorithm`, which computes the mode list from the key, Size and the character controls. See [Objects](Objects).
- **Profile**: an object's static `ModalProfile` table. Today it only sets the object's base mode count.
- **Character controls**: the per-object parameters (`CharacterParams`), such as Pipe Diameter or Metal Thinness.
- **Strike level**: a bank's estimated peak under a unit impulse. Every bank is normalized to `STRIKE_LEVEL_REF` at note-on, so objects and sizes start at comparable levels.
- **Band fade**: the raised-cosine weight that fades a mode out at the top of the audible band, `min(20 kHz, 0.45·fs)`.
- **Prune**: dropping modes that can't sound (at or above 80 kHz, or with a decay under 0.1 ms) when the bank is built.
- **Ring bound**: the resonator's tracked upper bound on its future output, Σ weight·|mode state|.
- **Drive governor**: the per-voice gain on the exciter's force that keeps the ring bound under full scale. See [Exciters](Exciters).
- **Pitch factor**: the product of pitch bend, object dynamics and Heat's warble that the modes are retuned by each sample.
- **Tension drop, buckling warp**: the per-sample dynamics of Taut Cable (pitch rises with ring amplitude) and Sheet Metal (pitch rises with the square of the low-mode displacement).
- **Thermal warble**: Heat's slow pitch wander, two LFOs at 0.37 and 0.83 Hz, about ±8 cents at Heat 1. See [Material Transformations](Material-Transformations).
- **Rattle**: amplitude-gated noise a voice adds when Damage is above 0.
- **Exciter**: the per-voice model that pushes on the object (strike, bow, scrape, jet…). There are 16 models. See [Exciters](Exciters).
- **Hit, Friction, Specialty**: the three exciter families (`ExciterFamily`). A family picks the force envelope: a one-shot AR, the MSEG, or the ADSR. See [Envelopes and Modulation](Envelopes-and-Modulation).
- **Interaction bus**: the two-way link between an exciter and its object. The exciter's force goes in; the object's displacement and velocity come back. See [Interaction Bus and Contact](Interaction-Bus-and-Contact).
- **MSEG**: the multi-stage force envelope the friction exciters use. Its stages can loop.
- **Force envelope**: the AR, ADSR or MSEG level that multiplies an exciter's force.
- **Drive level**: the per-exciter force multiplier (`EXCITER_DRIVE_LEVEL`) that brings all 16 models to a common level.
- **Strike position**: the normalized point (0–1) where the exciter touches the object. It weights mode *n* by `sin((n+1)πP)`.
- **Contact displacement**: the position-weighted sum of the modes' outputs at the strike point, `Σ cᵢ·yᵢ`.
- **Implicit contact, `ContactPrediction`**: solving an exciter's force together with the resonator's response in the same sample (`F = h(F)`). This sample's contact displacement is predicted as `free + compliance·F` and solved with an Illinois root finder.
- **Hertzian, Hunt–Crossley, Kelvin–Voigt contact**: contact laws. Hertzian force grows as penetration^1.5. Hunt–Crossley adds damping proportional to penetration. Kelvin–Voigt is a spring and a damper in parallel. See [Hit Exciters](Hit-Exciters).
- **Stribeck curve**: a friction coefficient that falls from static to dynamic friction as the slip speed rises. See [Friction Exciters](Friction-Exciters).
- **Release snap**: the loud ring when a one-signed friction force is released and the deflected object springs back.

## Voices and notes

- **Voice**: one sounding note: a `Voice` with its exciter, resonator and force envelope, one of 16 in the `VoiceManager` pool. See [Voices and Polyphony](Voices-and-Polyphony).
- **Held note**: a voice whose key is down, or which the sustain pedal holds. **Pedal-held**: the key is up but the pedal keeps the voice active.
- **Tail**: a released voice that keeps rendering while its resonator decays, until its peak stays under 1e-4 for 100 ms.
- **Peak hold**: a voice's decaying peak (×0.999 per sample at 48 kHz), used to pick which voice to steal and to end tails.
- **Voice stealing**: reusing a sounding voice for a new note when all 16 are busy, chosen by tier (`steal_index`).
- **Voice ID**: the host's per-note identifier (CLAP or VST3 note ID). **`NoteAddress`**: a note event's optional voice ID plus its MIDI channel. See [Note Expression](Note-Expression).
- **PolyTuning**: a per-note pitch offset in semitones, added to the channel pitch bend.
- **Expression gain**: the post-clamp voice gain `1 + 0.5·max(channel, poly pressure) + 0.3·mod wheel`.
- **Choke**: a host event that fades one note to silence in 2 ms and frees its voice.
- **All Sound Off, panic**: CC120: a 5 ms output fade, then every voice and the post chain are cleared.
- **Tonal, Kit, Drone**: the Note Mode (`play_mode` 0, 1, 2). Tonal plays the key's pitch. Kit plays each key's own pad. Drone sustains continuous exciters while a key is held.
- **Pad**: one key's sound in Kit mode (`KitPad`, then `Pad`): every voice-level parameter, a tune and a level. See [Kit Mode](Kit-Mode).
- **`KitTable`**: the audio thread's 128-slot table of ready-to-play pads, guarded by a seqlock.
- **`NoteSetup`**: what a note-on reads from the parameters (profile, size, Rust, Damage, exciter, `VoiceControls`). Live notes and pads share it.
- **`VoiceControls`**: the snapshot of parameters a voice copies at note-on.
- **Live control**: a parameter re-sent to held voices every block (Damping, Brightness, strike position, coupling, wander, Pressure…). See [Parameters](Parameters).
- **Macro bias**: the offsets and factors the four macros (0.5 is neutral) apply to voice and post-chain values (`resolve_macro_bias`).

## The plugin shell

- **`process_block`**: the plugin's whole audio path below the host API. `process()`, the offline renderer and the web engine all call it. See [Plugin Lifecycle](Plugin-Lifecycle).
- **`begin_block`**: the per-block setup that reads the parameters once and configures the post chain, the limiters and the live controls.
- **`filter_state`**: NIH-plug's hook that rewrites saved host state before it is restored. Saved DAW projects are migrated there. See [Presets and Migrations](Presets-and-Migrations).
- **Host state (`PluginState`)**: what NIH-plug saves in a DAW project: parameter values by ID, plus `#[persist]` fields such as the kit.
- **Gesture**: a host parameter edit bracketed by begin-set and end-set, so the host records it and can undo it.
- **`ParamSetter`, `GuiContext`**: NIH-plug's interfaces an editor uses to begin, set and end parameter changes on the host.
- **`significant_display`**: the 5-significant-digit formatter that keeps parameter text round-trips stable for clap-validator.

## Post chain and output

- **Post chain**: the processing after the voice mix: filter, drive, FEM body, spread, Space and the diode stage. See [Post Chain](Post-Chain).
- **Voice-mix DC blocker**: the 10 Hz second-order Butterworth high-pass on the summed voices (`StereoDcBlocker`, `f64` state).
- **Master drive**: `apply_drive`, the asymmetric waveshaper with a dry/shaped crossfade after the voice mix ("Legacy Drive").
- **Quality mode**: Eco, Normal, High or Render. At note-on it scales the mode count by 0.5, 1, 1.5 or 2. It also sets the diode clipper's oversampling to 1×, 4×, 8× or 16×. Render is the mode every guide clip and measurement uses.
- **Oversampling**: running a stage at a multiple of the host rate, so its harmonics above Nyquist can be filtered out before coming back down.
- **Aliasing**: harmonics above Nyquist that fold back into the audible band as inharmonic tones.
- **Half-band filter**: a low-pass whose transition is centred on a quarter of its sample rate, used for 2× up- and down-sampling. Here they are elliptic, polyphase all-pass designs. See [Output Protection](Output-Protection).
- **Group delay**: how many samples a filter delays a frequency. The half-bands delay low frequencies by 4–5 host samples, which the plugin doesn't report.
- **Diode clipper, diode stage**: the oversampled output stage with a tanh knee per polarity, set by Diode Softness and Diode Asymmetry.
- **Knee**: where a clipper's curve leaves the linear region.
- **Analog Ceiling**: the output's peak ceiling (default 0.9661, about −0.3 dBFS), shared by the diode stage (÷ 1.3) and both limiter modes.
- **Look-ahead limiter**: a limiter that delays the signal (48 samples here) so it can lower the gain before a peak arrives. Its delay is reported as latency.
- **DC blocker**: a high-pass at a few hertz that removes a constant offset.
- **One-pole smoother**: `x += (target − x)·k` per sample, with `k = 1 − e^(−1/(τ·fs))`.
- **TPT, zero-delay feedback**: the filter's discretization. Trapezoidal integrators keep the analog structure, and the feedback loop is solved within the same sample. See [Filter and Drive](Filter-and-Drive).
- **Bilinear prewarp**: choosing `g = tan(π·fc/fs)` so a digital filter's cutoff lands exactly on `fc`.
- **Self-oscillation**: a resonant filter ringing on its own with no input.
- **Lorenz attractor**: a three-variable chaotic system (σ 10, ρ 28, β 8/3), used as a free-running modulator of the drive.
- **Mid/side**: the sum and difference of a stereo pair, `(L + R)/2` and `(L − R)/2`. **Crossfeed**: mixing part of each channel into the other.
- **Schroeder reverb**: parallel feedback comb filters followed by series all-pass diffusers. **Comb filter**: a delay line fed back on itself. See [Body, Spread and Space](Body-Spread-and-Space).
- **Hermite interpolation**: 4-point cubic interpolation for reading a delay line between samples.

## Rates and real time

- **48 kHz reference**: the rate per-sample constants are written at. They are converted with `rate::ratio = 48 000 / fs`. See [Sample Rate Rule](Sample-Rate-Rule).
- **Event splitter**: `rate::EventSplitter`, which spreads a short event across two samples to keep its area and timing at any rate.
- **Note seed**: `note_seed(note, velocity, start_frame)`, the per-note noise seed, salted per consumer.
- **Seqlock**: a lock-free scheme. The writer makes a sequence counter odd while it writes, and readers retry if the counter changed. Readers never wait. See [Real-Time Safety](Real-Time-Safety).
- **SPSC ring**: a single-producer, single-consumer lock-free queue. The editor's note ring is one (`EditorNotes`, 256 `AtomicU32` slots).
- **`NoteMask`**: a set of 128 MIDI notes stored as two `u64` words.
- **Flush-to-zero (FTZ)**: a CPU mode that turns denormal floats into zero, which NIH-plug enables during `process()`.

## Presets and the factory bank

- **Floor render**: a held middle C at velocity 100, 2 s at 48 kHz, through the plugin's output path. It is used to level and check every factory preset. See [Factory Bank](Factory-Bank).
- **Quality floor**: the checks every factory preset must pass (`FloorReport::check`): finite samples, RMS within ±6 dB of `TARGET_RMS`, at most 1 % of samples at the ceiling at velocity 100 and 127, and the full-velocity RMS no more than 12 dB under the target.
- **`TARGET_RMS`**: 0.13 (−17.7 dBFS), the RMS every factory preset's floor render is levelled to.
- **Seed try**: the index mixed into an experimental preset's seed. The seeder raises it to re-draw a preset that fails.
- **Spectral centroid, spectral flatness**: the power-weighted mean frequency of a spectrum, and the ratio of its geometric to arithmetic mean power (near 1 for noise, near 0 for tones). The bank's character checks use them.

## Front ends

- **egui**: the immediate-mode Rust GUI library the editor is written in. It redraws every widget each frame from the parameters. See [Editor](Editor).
- **baseview, `nih_plug_egui`**: the windowing library, and NIH-plug's adapter that runs an egui editor inside a plugin host's window.
- **eframe**: egui's application framework. The web build uses it to draw the editor into a `<canvas>`. See [Web App](Web-App).
- **Cap height**: the height of capital letters per em. `UI_EM_SCALE` matches it between the editor's font and the one it replaced.
- **OFL**: the SIL Open Font License, the license of Inter and IBM Plex Sans.
- **AudioWorklet**: the browser API that runs audio code on a real-time thread, in **quanta** of 128 frames (2.67 ms at 48 kHz).
- **wasm-bindgen**: the tool and crate that generate the JavaScript glue for a Rust wasm module. The editor uses it; the engine doesn't.
- **Starlight, Pagefind**: Astro's documentation theme, which builds the user Wiki, and the static search index it builds. See [Site and User Wiki Tooling](Site-and-User-Wiki-Tooling).
- **Opus**: the audio format of the user Wiki's clips, encoded at 48 kbps.

## Testing and process

- **Golden fixture**: a committed file of expected results that a test compares against, such as the mode-bank hashes. See [Testing](Testing).
- **Drift test**: a test that regenerates committed generated files in memory and fails when they differ from the files on disk.
- **Render hash**: a hash over the bits of every rendered sample, used to prove two builds produce bit-identical output. Hashes differ between the gnu and musl targets.
- **FNV-1a**: the simple 64-bit hash behind the render hashes and the clip plan's fingerprint.
- **Twin benchmark**: a benchmark identical to another except for one feature, compared by ratio within one run. See [Benchmarks and Performance](Benchmarks-and-Performance).
- **Ad-hoc signing**: macOS code signing with no developer identity (`codesign --sign -`). Apple Silicon can load the code, but it isn't notarized. See [Building and Bundling](Building-and-Bundling).
- **OpenSpec change**: a planning folder (proposal, design, specs, tasks) under `openspec/changes/`, archived when done. **Capability**: a named area of requirements in `openspec/specs/<capability>/spec.md`. See [Contributing and Keeping This Wiki Current](Contributing-and-Keeping-This-Wiki-Current).
