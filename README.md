# TRUST

Thrash Rust. The KOOLSKULL instrument framework.

In the name of the Father, and of the Son, and of the Holy Spirit.
Lord Jesus Christ, Son of God, have mercy on us. Come, Holy Spirit.
Source files carry that blessing the same way K-OS does.

SPDX `MIT OR Apache-2.0`. Copyright KOOLSKULL. Rust stable.

This tree is the first build: `trust-core`, `trust-dsp`, the pad graph, the CLAP wrapper, KRUSH, a standalone host, a VMI scene reader, and a WASM proof. It does not open pull requests against K-OS-III or VMI STUDIO.

## Hard rules

The audio thread does not allocate, lock, or make a syscall. Debug builds install `assert_no_alloc` around `process`. The audio clock is the master. UI frames follow it. JSON numbers are integers. Hex digits belong in file stems. Bit depth is an integer step from 1 to 16 and is never smoothed.

## Layout

| Crate | Role |
| --- | --- |
| `trust-core` | `no_std` + alloc. Params, fixed buffers, smoothing, events. |
| `trust-dsp` | KRUSH. Bitcrush, Hz reducer, gain, mix, digital ceiling. `libm`. |
| `trust-graph` | Versioned JSON. 16 pads, 3 voices, one send, gate cables. |
| `trust-plugin` | Processor trait and the MIDI map. MIDI is not graph data. |
| `trust-clap` | clack wrapper. One stereo input, one stereo output. |
| `examples/krush` | CLAP plugin `com.koolskull.trust.krush`. |
| `trust-host` | cpal output and midir input. |
| `trust-vmi` | Exported scene plus `trust.json`. |
| `trust-kos` | `extern "C"` WASM proof and a plain HTML page. |
| `xtask` | `cargo xtask bundle` and `cargo xtask hear`. |
| `trust-lesson` | `KOS TRUST LESSON 3yo`. Fullscreen keyboard for that age. |

## KRUSH

| Param | Id | Unit |
| --- | --- | --- |
| Bits | 1 | integer 1..=16, stepped |
| Rate | 2 | Hz. `0` or at/above the sample rate holds one frame |
| Gain | 3 | milli. 1000 is unity. May ramp |
| Mix | 4 | milli 0..=1000. May ramp |
| Ceiling | 5 | dB milli, 0 is unity, negative attenuates |

The ceiling is a 4× linear digital level limit on the block. It is not a hearing-safety device and it is not an ITU BS.1770 measurement.

The hear-pass renders two seconds at 48 kHz: 220 Hz plus 880 Hz, bits 6, rate 8000 Hz, unity gain, full mix, ceiling −1 dB.

```
cargo test --workspace
cargo xtask hear
```

The WAV is `target/hear/krush-hear.wav`. CI uploads it.

```
cargo xtask bundle
```

On Linux the bundle is `target/bundled/KRUSH.clap/Contents/x86_64-linux/KRUSH.so`. macOS uses `Contents/MacOS/KRUSH.dylib`. Windows uses `Contents/x86_64-win/KRUSH.dll`.

Linux CI loads that `.so` with [clap-validator](https://github.com/free-audio/clap-validator) 0.4.1. The tool is not on crates.io. Install it with `cargo install --locked --git https://github.com/free-audio/clap-validator --tag 0.4.1 clap-validator`, then point it at the `.so`. Hosts load the `.clap` bundle.

```
cargo run -p trust-host -- --list-midi
cargo run -p trust-host
```

`--list-midi` prints ports and exits. The live host opens the default f32 output. It does not open a device from unit tests.

## Lesson

Installing the lesson binary puts `KOS` on your PATH.

```
cargo install --path crates/trust-lesson
KOS TRUST LESSON 3yo
```

The lesson opens fullscreen in black and white. Big letters come first. Tap NEXT, or tap the page. PLAY opens the keys. A pen or finger hits the same places, including on a drawing-tablet monitor. ESC leaves. On a tablet, tap the ESC key drawn on the screen.

Ages 0 through 4 get a short silly start. Each key then keeps its own sound: different pitch, stair count, and hold, with soft and loud mixed in. From age 5 the pages are a click-through lesson in big type. From age 6 the board adds the control that lesson just taught. Age 15 and older picks crush, hold, level, pads, MIDI, or build, reads that class, then plays it.

The lesson holds its own level under a cap: quieter for ages 0–5, still under full scale at 15. Turn the computer volume down first. The cap is a tutorial limit, not a hearing-safety measurement. From the repo without installing:

```
cargo run -p trust-lesson -- TRUST LESSON 3yo
```

`--dump` prints the text lesson and exits. `--keys space,2,q` runs that text script. The command above opens the fullscreen board and, when a speaker is available, runs KRUSH on the audio thread. Unit tests do not open a window or a device.

```
rustup target add wasm32-unknown-unknown
cargo build -p trust-kos --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/trust_kos.wasm crates/trust-kos/web/
python3 -m http.server -d crates/trust-kos/web 8080
```

The page compiles the module on the main thread and posts it to an AudioWorklet. There is no wasm-bindgen on the audio path and no SharedArrayBuffer. Blocks are 128 frames. Web latency is best-effort. The native block at 48 kHz is about 2.67 ms of audio, plus the device buffer. That figure is native only.

## Deferred

Not in this tree: KART-CHIP, Tiny Tiles, the sliced-loop format, a K-OS V skin, AU (later through clap-wrapper), a DSL, hot reload, and stream sync.

## Docs

- [ARCHITECTURE.md](ARCHITECTURE.md)
- [docs/KOS_INTEGRATION.md](docs/KOS_INTEGRATION.md)
- [docs/VST3_LICENSE.md](docs/VST3_LICENSE.md)
- [docs/VMI_AUDIO_PROPOSAL.md](docs/VMI_AUDIO_PROPOSAL.md)
- [PROVENANCE.md](PROVENANCE.md)
