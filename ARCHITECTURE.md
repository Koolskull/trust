# Architecture

In the name of the Father, and of the Son, and of the Holy Spirit.
Lord Jesus Christ, Son of God, have mercy on us. Come, Holy Spirit.

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

`KOS TRUST LESSON <age>` is the `trust-lesson` binary. The words are case-insensitive. `3yo`, `3y`, and `3` are the same age. The live command opens fullscreen. Big-letter pages come first. NEXT, or a tap on the page, moves forward. PLAY opens the keys. ESC, and the ESC key on the page, leave. A pen, a mouse, or a touch hits those same targets. Ages 0 through 4 get a short silly start, then each key keeps its own pitch, stair count, and hold. From age 5 (`READING_AGE`) the pages teach that year, and from age 6 the board adds that year's control. Age 15 and older (`FULL_CLASS_AGE`) picks crush, hold, level, pads, MIDI, or the build, reads that class, then plays it. `--dump` and `--keys` stay headless. They still walk the guided text lesson one step at a time, and Q quits that script. Unit tests do not open a window or an audio device. When a speaker is available, the live board fills it from `Voice::render` under `assert_no_alloc`. `Voice::render` applies `level_cap` again, so a hot request cannot skip it. Ages 0–5 cap at 350 milli and −12000 dB milli. Ages 15 and older cap at 800 milli and −3000 dB milli. The ceiling is a digital limit, not a hearing-safety claim.

## Thrash 0.6.0

`trust-tracker` is product version 0.6.0. The workspace stays 0.1.0. The JUCE app stays 0.5.3 and still speaks `dm2.3`. This slice is one phrase: 16 rows, 8 channels, 120 bpm, 4 rows a beat. Empty cells are silent for that row. Eight sines sum, divide by eight, then one KRUSH runs on the mix. The audio thread copies the grid through atomics, renders under `assert_no_alloc`, and publishes the play row. The edit cursor does not follow that row. A pen selects a cell. Keys write pitches. Space starts at row 0. Unit tests do not open a window or a device.

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
