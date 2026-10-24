# TRUST

Thrash Rust. The KOOLSKULL instrument framework.

In the name of the Father, and of the Son, and of the Holy Spirit.
Lord Jesus Christ, Son of God, have mercy on us. Come, Holy Spirit.
Source files carry that blessing the same way K-OS does.

SPDX `MIT OR Apache-2.0`. Copyright KOOLSKULL. Rust stable, edition 2021, `rust-version` 1.85.
Repository: https://github.com/Koolskull/trust

## Introduction

TRUST is two things at once. It is a small set of programs you can run today, and it is the Rust framework those programs are built on.

If you want to hear it, start with one of these:

- `KOS TRUST LESSON 3yo` opens a fullscreen black-and-white lesson. The age in the command picks the lesson. A pen, a finger, or a mouse works, including on a drawing-tablet monitor.
- `trust-tracker` opens Thrash 0.6.0: one 16-row phrase, eight channels, and one oscillator through the KRUSH effect. Arrow keys and a pen move. The tracker keys write notes. Space plays.

If you are here to work on the instrument, the workspace version is `0.1.0`. Thrash 0.6.0 is the only crate with its own product version. The audio device is the clock. The audio thread does not allocate, lock, or make a syscall. JSON numbers are integers. Bit depth is a step from 1 to 16 and is never smoothed. The ceiling is a digital level limit on the block. It is a tutorial and mix limit, not hearing protection, and not an ITU BS.1770 meter.

Thrash Tracker 0.5.3 is the shipping desktop tracker, in its own repository: https://github.com/Koolskull/thrashtracker. It still owns `.dm2` songs (`dm2.3`). This repository does not open those songs yet. It does not open pull requests against K-OS-III or VMI STUDIO.

[ARCHITECTURE.md](ARCHITECTURE.md) is the contract behind the tour on this page.

## Play it

Install Rust from https://rustup.rs and clone this repository. On Linux, the audio examples also need the ALSA headers:

```
sudo apt-get install pkg-config libasound2-dev
```

Turn the computer volume down before any command that opens a speaker.

### The lesson

```
cargo install --path crates/trust-lesson
KOS TRUST LESSON 3yo
```

From the repository, without installing:

```
cargo run -p trust-lesson -- TRUST LESSON 3yo
```

The words are case-insensitive. `3yo`, `3y`, and `3` are the same age. Ages run from 0 through 120. Age 0 has its own two pages, Surprise and Tap.

Big letters come first. Tap NEXT, or tap the page. PLAY opens the keys. ESC leaves. On a tablet, tap the ESC key drawn on the screen. Q does not leave the window. In the headless text script, Q quits.

Each key keeps its own sound for that age. Younger lessons are sillier and more varied: honk, chirp, robot, bubble, and the rest, with soft and loud mixed in. From age 5 the pages are a lesson you click through. From age 6 the board adds the control that lesson just taught. At 15 and older you pick the class.

| Age | The pages | Then the keys |
| --- | --- | --- |
| 0–1 | Surprise. Every key is a different silly sound. | Mash the board and find a favorite. |
| 2 | Some keys are quiet. Some look bigger. | Hear which ones are soft. |
| 3 | Honk, chirp, robot, zap. | The differences are on purpose. |
| 4 | Low keys and high keys. | Pitch jumps. It does not slide. |
| 5 | Press, and the sound starts. Let go, and it stops. | First click-through lesson. |
| 6 | KRUSH cuts a sound into stairs. | 1 is smoother, 2 is the middle, 3 is blocky. |
| 7 | Bit depth is that stair count, from 1 to 16. | Choose 16, 8, 4, or 2. |
| 8 | Hold keeps one piece of the sound. | 1 is fast. 2 is slow. |
| 9 | The hold is a rate in Hz. | Short, medium, or long. |
| 10 | Dry is the plain tone. Wet is KRUSH. | A little, half, or all crush. |
| 11 | Mix is a number from 0 to 1000. 1000 is all wet. | A little crush, or a lot. |
| 12 | The ceiling is a digital lid on the block. | Lid off, or lid on. On is quieter. |
| 13 | A pad, 3 voices, one send, a gate. | Open, close, send. |
| 14 | The same ideas, named the way the instrument names them. | Open, close, send, then bits 6. |
| 15+ | You choose crush, hold, level, pads, MIDI, or build. | Read that class, then play it. |

The lesson holds its own level under a cap, quieter for ages 0–5 and still under full scale at 15. The cap is a tutorial limit. Turn the computer volume down first.

```
KOS TRUST LESSON 6yo
KOS TRUST LESSON 15yo
KOS TRUST LESSON 7yo --dump
KOS TRUST LESSON 7yo --keys space,2,q
```

`--dump` prints the text lesson and exits. `--keys` runs that text script and exits. The live command opens the fullscreen board and, when a speaker is available, runs KRUSH on the audio thread. Unit tests do not open a window or a device.

### Thrash 0.6.0

```
cargo install --path crates/trust-tracker
trust-tracker
```

From the repository:

```
cargo run -p trust-tracker
cargo run -p trust-tracker -- --dump
cargo run -p trust-tracker -- --help
```

The binary name is `trust-tracker`. The window title is `THRASH 0.6.0`. The screen is fullscreen, black and white. `--dump` and `--help` print and exit. An unknown flag prints the help and exits. Unit tests do not open a window or a speaker.

You are editing one phrase. It has 16 rows and 8 channels. Tempo is 120 bpm and 4 rows a beat, so the phrase is two seconds long and then it loops. An empty cell is silent for that row. A note sounds for its own row, then the next row starts again or stays silent.

The play row is white, with black note text. Your cursor is the inner border. The cursor stays where you left it while the play row moves.

| Key | What it does |
| --- | --- |
| Arrows | Move. The edges wrap. |
| `Z S X D C V G B H N J M` | Write C through B of the current octave, then step down. |
| `Q 2 W 3 E R 5 T 6 Y 7 U` | Write C through B of the next octave, then step down. |
| `[` and `]` | Octave down and up. The range is 1 through 7. The start is 4, so `Z` is C-4. |
| Space | Start and stop. Starting begins again at row 0. |
| Backspace or Delete | Clear the cell, then step down. |
| Pen, mouse, or touch | Select a cell. A tap does not write a pitch. |
| ESC | Leave. The ESC key drawn on the screen does the same. |

`trust-tracker --dump` prints a stopped phrase. The first line is `THRASH 0.6.0`. A cursor row starts with `>`. An empty cell prints `---`.

The sound is eight sines, summed and divided by eight, then one KRUSH on that mix. The crush is fixed for this slice: 8 bits, 8 kHz hold, gain 450 milli, full mix, ceiling −6 dB. That ceiling is a digital level limit.

This screen is the first slice of the remake. Song, chain, the seven instrument kinds, the kit, the effect pads, and `.dm2` files stay in Thrash Tracker 0.5.3 until a later slice.

## Build it

```
cargo test --workspace
cargo xtask hear
```

The hear-pass renders two seconds at 48 kHz: 220 Hz plus 880 Hz, bits 6, rate 8000 Hz, unity gain, full mix, ceiling −1 dB. The WAV is `target/hear/krush-hear.wav`. CI uploads it.

```
cargo xtask bundle
```

On Linux the bundle is `target/bundled/KRUSH.clap/Contents/x86_64-linux/KRUSH.so`. macOS uses `Contents/MacOS/KRUSH.dylib`. Windows uses `Contents/x86_64-win/KRUSH.dll`. Hosts load the `.clap` bundle. Linux CI checks the `.so`.

[clap-validator](https://github.com/free-audio/clap-validator) 0.4.1 is not on crates.io. Install it with:

```
cargo install --locked --git https://github.com/free-audio/clap-validator --tag 0.4.1 clap-validator
clap-validator validate --only-failed target/bundled/KRUSH.clap/Contents/x86_64-linux/KRUSH.so
```

The standalone host lists MIDI ports, or opens the default f32 output:

```
cargo run -p trust-host -- --list-midi
cargo run -p trust-host
```

`--list-midi` prints ports and exits. Unit tests do not open a device.

The WASM proof:

```
rustup target add wasm32-unknown-unknown
cargo build -p trust-kos --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/trust_kos.wasm crates/trust-kos/web/
python3 -m http.server -d crates/trust-kos/web 8080
```

The page compiles the module on the main thread and posts it to an AudioWorklet. There is no wasm-bindgen on the audio path and no SharedArrayBuffer. Blocks are 128 frames. Web latency is best-effort. The native block at 48 kHz is about 2.67 ms of audio, plus the device buffer. That figure is native only.

`rust-toolchain.toml` pins stable Rust with rustfmt, clippy, and `wasm32-unknown-unknown`. CI runs on Ubuntu, macOS, and Windows: format check, `cargo test --workspace`, the KRUSH benchmark compiled with `--no-run`, and `cargo xtask hear`. Linux also builds the WASM proof and validates the CLAP bundle.

## The crates

| Crate | Version | Role |
| --- | --- | --- |
| `trust-core` | 0.1.0 | `no_std` + alloc. Params, fixed buffers, smoothing, events. `BLOCK_FRAMES` is 128. |
| `trust-dsp` | 0.1.0 | KRUSH. Bitcrush, Hz reducer, gain, mix, digital ceiling. Math goes through `libm`. |
| `trust-graph` | 0.1.0 | Versioned JSON. 16 pads, 3 voices, one send, gate cables. |
| `trust-plugin` | 0.1.0 | Processor trait and the MIDI map. MIDI is not graph data. |
| `trust-clap` | 0.1.0 | clack wrapper. One stereo input, one stereo output. |
| `examples/krush` | 0.1.0 | CLAP plugin `com.koolskull.trust.krush`. |
| `trust-host` | 0.1.0 | cpal output and midir input. |
| `trust-vmi` | 0.1.0 | An exported VMI STUDIO scene plus `trust.json`. |
| `trust-kos` | 0.1.0 | `extern "C"` WASM proof and a plain HTML page. |
| `xtask` | 0.1.0 | `cargo xtask bundle` and `cargo xtask hear`. |
| `trust-lesson` | 0.1.0 | `KOS TRUST LESSON 3yo`. Fullscreen lesson for that age. |
| `trust-tracker` | 0.6.0 | Thrash 0.6.0. One phrase, eight channels, one oscillator through KRUSH. |

## KRUSH

The processing order is hold, then crush, then gain and mix, then the ceiling trim. Denormals, NaN, and Inf flush to 0.

| Param | Id | Unit |
| --- | --- | --- |
| Bits | 1 | integer 1..=16, stepped, never smoothed |
| Rate | 2 | Hz. `0` or at/above the sample rate holds one frame |
| Gain | 3 | milli. 1000 is unity. May ramp for `sample_rate / 200` samples |
| Mix | 4 | milli 0..=1000. May ramp the same way |
| Ceiling | 5 | dB milli. 0 is unity. Negative attenuates |

The ceiling is a 4× linear digital level limit on the block.

## Rules

The audio thread does not allocate, lock, or make a syscall. Debug builds install `assert_no_alloc` around `process`. Scratch is allocated before the stream starts, or in CLAP `activate`. The audio clock is the master. A UI frame reads the last published block.

Graph levels, caps, param values, and `trust.json` version are integers. Milli-gain uses 1000 as unity. Floats in that JSON are rejected. Hex digits belong in `SSSXXX` file stems, not in JSON numbers.

A `midi` key anywhere in the graph is an error. Note and CC bindings live in `trust_plugin::MidiMap`.

## Elsewhere

Thrash Tracker 0.5.3 remains the desktop tracker for songs, chains, phrases of any length, the seven instrument kinds (SMP, OSC, VST, FM, OS4, SPR, KIT), the kit, and the effect pages. Format `dm2.3` stays there.

Not in this tree yet: KART-CHIP, Tiny Tiles, the sliced-loop format, a K-OS V skin, AU, a DSL, hot reload, and stream sync. The sliced-loop format has no public name here. A name has to clear a trademark check before it ships. See [PROVENANCE.md](PROVENANCE.md).

Registering TRUST inside K-OS is documented in [docs/KOS_INTEGRATION.md](docs/KOS_INTEGRATION.md). This repository does not apply that edit.

## Docs

- [ARCHITECTURE.md](ARCHITECTURE.md) is the build contract: clock, audio thread, JSON, graph, lesson, Thrash 0.6.0, CLAP, VMI, WASM.
- [docs/KOS_INTEGRATION.md](docs/KOS_INTEGRATION.md)
- [docs/VST3_LICENSE.md](docs/VST3_LICENSE.md)
- [docs/VMI_AUDIO_PROPOSAL.md](docs/VMI_AUDIO_PROPOSAL.md)
- [PROVENANCE.md](PROVENANCE.md)
