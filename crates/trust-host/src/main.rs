//                    ☦
//         ╔════════════════════╗
//         ║  My sheep hear     ║
//         ║  my voice.         ║
//         ╚════════════════════╝
//
//   John 10:27. Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit. The audio clock is the master.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

mod ring;

use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, SampleRate, StreamConfig};
use trust_core::Event;
use trust_dsp::Krush;
use trust_plugin::{pack_midi, MidiMap};

use crate::ring::MidiRing;

#[cfg(debug_assertions)]
#[global_allocator]
static TRUST_ALLOC: assert_no_alloc::AllocDisabler = assert_no_alloc::AllocDisabler;

const SCRATCH: usize = 8192;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("trust-host\n  --list-midi   list MIDI inputs and exit");
        return ExitCode::SUCCESS;
    }
    if args.iter().any(|a| a == "--list-midi") {
        return match list_midi() {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{err}");
                ExitCode::from(1)
            }
        };
    }
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn list_midi() -> Result<(), String> {
    let input = midir::MidiInput::new("trust-host").map_err(|e| e.to_string())?;
    let ports = input.ports();
    if ports.is_empty() {
        println!("no midi inputs");
        return Ok(());
    }
    for (i, port) in ports.iter().enumerate() {
        let name = input.port_name(port).unwrap_or_else(|_| "unnamed".into());
        println!("{i}: {name}");
    }
    Ok(())
}

fn run() -> Result<(), String> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("no default output device")?;
    let range = device
        .supported_output_configs()
        .map_err(|e| e.to_string())?
        .find(|r| r.sample_format() == SampleFormat::F32)
        .ok_or("output device has no f32 config")?;
    let supported = if range.min_sample_rate().0 <= 48_000 && range.max_sample_rate().0 >= 48_000 {
        range.with_sample_rate(SampleRate(48_000))
    } else {
        range.with_max_sample_rate()
    };
    let rate = supported.sample_rate().0;
    let channels = supported.channels() as usize;
    if channels == 0 {
        return Err("output device has no channels".into());
    }
    let config: StreamConfig = supported.into();
    let ring = Arc::new(MidiRing::new());
    let _midi = connect_midi(Arc::clone(&ring));

    let mut krush = Krush::new(rate);
    krush.set_bits(8);
    krush.set_rate_hz(12_000);
    krush.snap_gain(1000);
    krush.snap_mix(1000);
    krush.set_ceiling_db_milli(-1000);
    let map = MidiMap::krush();
    let mut in_l = vec![0.0f32; SCRATCH];
    let mut in_r = vec![0.0f32; SCRATCH];
    let mut out_l = vec![0.0f32; SCRATCH];
    let mut out_r = vec![0.0f32; SCRATCH];
    let ring_audio = Arc::clone(&ring);

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _| {
                render(
                    &mut krush,
                    &ring_audio,
                    &map,
                    &mut in_l,
                    &mut in_r,
                    &mut out_l,
                    &mut out_r,
                    data,
                    channels,
                );
            },
            |err| eprintln!("audio: {err}"),
            None,
        )
        .map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    println!("KRUSH  bits 8  rate 12000 Hz  ceiling -1 dB. Ctrl-C stops.");
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn connect_midi(ring: Arc<MidiRing>) -> Option<midir::MidiInputConnection<()>> {
    let input = midir::MidiInput::new("trust-host").ok()?;
    let port = input.ports().into_iter().next()?;
    let name = input.port_name(&port).unwrap_or_else(|_| "midi".into());
    println!("midi: {name}");
    input
        .connect(
            &port,
            "trust-host",
            move |_stamp, msg, _| {
                if let Some(packed) = pack_midi(msg) {
                    ring.push(packed);
                }
            },
            (),
        )
        .ok()
}

fn render(
    krush: &mut Krush,
    ring: &MidiRing,
    map: &MidiMap,
    in_l: &mut [f32],
    in_r: &mut [f32],
    out_l: &mut [f32],
    out_r: &mut [f32],
    data: &mut [f32],
    channels: usize,
) {
    let mut events = [Event {
        sample_offset: 0,
        param: 0,
        value: 0,
    }; 32];
    let mut count = 0;
    while count < events.len() {
        let Some(packed) = ring.pop() else {
            break;
        };
        count += map.map_packed(packed, &mut events[count..]);
    }
    let frames = data.len() / channels;
    let mut offset = 0;
    while offset < frames {
        let n = (frames - offset).min(in_l.len());
        for i in 0..n {
            let base = (offset + i) * channels;
            in_l[i] = data[base];
            in_r[i] = if channels > 1 {
                data[base + 1]
            } else {
                data[base]
            };
        }
        let ev = if offset == 0 { &events[..count] } else { &[] };
        let in_l = &in_l[..n];
        let in_r = &in_r[..n];
        let out_l = &mut out_l[..n];
        let out_r = &mut out_r[..n];
        assert_no_alloc::assert_no_alloc(|| {
            krush.process(in_l, in_r, out_l, out_r, ev);
        });
        for i in 0..n {
            let base = (offset + i) * channels;
            data[base] = out_l[i];
            if channels > 1 {
                data[base + 1] = out_r[i];
            }
        }
        offset += n;
    }
}
