# Architecture

In the name of the Father, and of the Son, and of the Holy Spirit.
Lord Jesus Christ, Son of God, have mercy on us. Come, Holy Spirit.

## Introduction

[README.md](README.md) is the tour: how to open the lesson, how to play Thrash 0.6.0, and what each crate is. This file is the contract those programs share.

The workspace version is `0.1.0`. `trust-tracker` is product `0.6.0` and does not inherit the workspace version. Thrash Tracker 0.5.3 is a separate JUCE application and still speaks `dm2.3`. This tree does not load a `.dm2` file.

| Surface | Where it lives | What the audio thread sees |
| --- | --- | --- |
| KRUSH | `trust-dsp`, wrapped by `trust-clap` and `trust-host` | One stereo `process` over the caller's slices. |
| Lesson | `trust-lesson`, binary `KOS` | `Voice::render` under `assert_no_alloc`, after `level_cap`. |
| Thrash 0.6.0 | `trust-tracker` | Eight sines summed into one `Krush::process`. The grid arrives through atomics. |
| Graph | `trust-graph` | A published bank. MIDI never travels inside the JSON. |
| WASM proof | `trust-kos` | `trust_process` over 128-frame static buffers. |

Native processing is requested in 128-frame blocks. A UI frame reads what the audio thread last published. The UI does not drive the clock.

## Clock

The audio device is the master clock. A UI frame reads the last published block. It does not drive the instrument. Native processing is requested in 128-frame blocks (`trust_core::BLOCK_FRAMES`). At 48 kHz that is about 2.67 ms of audio, plus whatever buffer the device or the CLAP host already has. There is no separate millisecond latency budget in this build. The WASM proof measures nothing and promises nothing about latency.

## Audio thread

`trust-core` and `trust-dsp` are `no_std` with `alloc`. Math goes through `libm`. `process` uses fixed smoother state and the caller's slices. The event queue is a fixed array. `events_to_vec` is for hosts and tests, off the audio thread.

Debug builds of KRUSH and `trust-host` install `assert_no_alloc::AllocDisabler` and wrap the DSP call in `assert_no_alloc`. Scratch for the CLAP wrapper and the standalone host is allocated in `activate` or before the stream starts. The CLAP wrapper copies host pointers into that scratch, runs the DSP, and writes back. It does not collect channel pairs into a `Vec` on the audio thread.

Bit depth is a `u8` applied on the sample that reads it. Gain and mix may ramp for `sample_rate / 200` samples. The sample-rate reducer holds `sample_rate / target_hz` frames, or one frame when the target is 0 or at least the sample rate. Denormals, NaN, and Inf flush to 0.

## JSON

Graph levels, caps, param values, and `trust.json` version are integers. Milli-gain uses 1000 as unity. Mix milli is 0..=1000. Ceiling is dB milli. Floats are rejected. Hex digits show up in `SSSXXX` file stems, not as JSON numbers.

## Graph and MIDI

`schema_version` 1 is a bank of 16 pads. Each pad declares 3 voices and one send (`amount_milli`). Cables are `kind: "gate"` only. A missing version, or version 0, migrates to that default bank. A newer version is rejected.

Voice allocation is a fixed list of 64. `note_on` steals the oldest voice of the object when the per-object cap is hit, and the oldest voice anywhere when the global cap is hit. The default caps are 3 and 32, so the 48 declared pad voices do not all sound at once.

A `midi` key anywhere in the graph is an error. Note and CC bindings live in `trust_plugin::MidiMap`. The standalone host pushes packed MIDI into an atomic ring from the midir callback and drains it on the audio callback.

## Lesson

`KOS TRUST LESSON <age>` is the `trust-lesson` binary. The words are case-insensitive. `3yo`, `3y`, and `3` are the same age. Accepted ages are 0 through `MAX_AGE` (120). Age 0 has its own pages in `AGE0`. The live command opens fullscreen. Big-letter pages come first. NEXT, or a tap on the page, moves forward. PLAY opens the keys. ESC, and the ESC key on the page, leave. A pen, a mouse, or a touch hits those same targets. Q does not leave the window.

Ages 0 through 4 (`READING_AGE` is 5) get a short silly start. `silly_sound` keeps a stable sound on each key: its own pitch, stair count, and hold, with quiet keys mixed in. From age 5 the pages teach that year in large type. From age 6, `knob_for` adds that year's control on the lesson board. Until age 15, `sound_for` stays on `BoardMode::Lesson`, so the crush, hold, level, pads, MIDI, and build classes remain the age-15 catalog. Age 15 and older (`FULL_CLASS_AGE`) picks one of those classes, reads it, then plays it. The page list for each year is in `crates/trust-lesson/src/book.rs`. The README table is the same map in plain language.

`--dump` and `--keys` stay headless. They still walk the guided text lesson one step at a time, and Q quits that script. Unit tests do not open a window or an audio device. When a speaker is available, the live board fills it from `Voice::render` under `assert_no_alloc`. `Voice::render` applies `level_cap` again, so a hot request cannot skip it.

| Age | Gain cap | Ceiling |
| --- | --- | --- |
| 0–5 | 350 milli | −12000 dB milli |
| 6–10 | 500 milli | −9000 dB milli |
| 11–14 | 700 milli | −6000 dB milli |
| 15 and older | 800 milli | −3000 dB milli |

The ceiling is a digital limit, not a hearing-safety claim.

## Thrash 0.6.0

`trust-tracker` is product version 0.6.0. The package version is the literal `"0.6.0"`, not `version.workspace`. The workspace stays 0.1.0. The JUCE app stays 0.5.3 and still speaks `dm2.3`.

This slice is one phrase: 16 rows (`ROWS`), 8 channels (`CHANNELS`), 120 bpm (`BPM`), 4 rows a beat (`ROWS_PER_BEAT`). One row is `sample_rate / 8` samples, so at 48 kHz the 16-row loop is two seconds. An empty cell is `EMPTY` (255) and is silent for that row. A sounding cell retriggers at the start of its row. The note does not hold into the next row.

Eight sines sum, divide by eight, then one KRUSH runs on the mix. `Engine::new` sets bits 8, rate 8000 Hz, gain 450 milli, mix 1000 milli, and ceiling −6000 dB milli. `render` pulls at most `BLOCK_FRAMES` (128) at a time and does not allocate. Pitch is armed only when a row starts, through `libm::powf`. Each sample uses `libm::sinf`.

The message thread owns the `Phrase`. The audio thread copies 128 note bytes from atomics onto the stack, renders under `assert_no_alloc`, and stores the play row. Space sets the restart flag before it sets playing, and the engine returns to row 0. The edit cursor does not follow that row. A pen, mouse, or touch calls `Phrase::select` and does not write a pitch. Note entry uses the tracker keyboard in `note_midi`: `Z` at octave 4 is MIDI 60. Octave clamps to 1..=7. The README has the full key table.

`--dump` prints `THRASH 0.6.0` and the grid, then exits. Unit tests do not open a window or a device. Debug builds of the binary install `AllocDisabler`. The guard is around `render` and `reset`, not around the whole process.

## CLAP

`trust-plugin::TrustProcessor` does not mention clack. `trust-clap` implements the clack `Plugin` and `PluginAudioProcessor` traits and the audio-ports extension (one stereo input, one stereo output). KRUSH exports `clack_export_entry!(SinglePluginEntry<TrustClap<KrushProcessor>>)`. The plugin id is `com.koolskull.trust.krush`.

clack and clap-sys are MIT OR Apache-2.0. This build does not depend on nih-plug.

## VMI

`trust-vmi` reads a folder that VMI STUDIO already exported. The committed fixture is `tests/fixtures/vmi_scene/` (the gamer scene). `trust.json` is the TRUST sidecar:

```json
{ "version": 1, "kind": "ui" }
```

`kind` is `ui` or `pack`. For `ui`, SSS is 000 rest, 001 hover, 002 pressed, 003 after, and XXX is the frame. For `pack`, SSS is the bank and XXX is the slice. Extensions are matched without case. Audio files are bare `SSSXXX.wav`, `.ogg`, or `.flac`. If more than one audio extension shares a stem, wav wins, then flac, then ogg. Object names come from `object.json` and the folder. The real gamer `object.json` stores `slots` as an array. `scene.json` stores `slots` as a map. Both load. Newer writer fields are optional.

## WASM

`trust-kos` is a `cdylib` and an `rlib`. The audio exports are `extern "C"`: `trust_init`, `trust_set_param`, `trust_process`, plus pointers to 128-frame static buffers. `crates/trust-kos/web/index.html` fetches the wasm, calls `WebAssembly.compile` on the main thread, loads `processor.js`, and `postMessage`s the `Module`. The worklet instantiates it. No SharedArrayBuffer. No wasm-bindgen.
