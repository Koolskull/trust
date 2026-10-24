//                    ☦
//         ╔════════════════════╗
//         ║  He has made       ║
//         ║  everything        ║
//         ║  beautiful in its  ║
//         ║  time.             ║
//         ╚════════════════════╝
//
//   Ecclesiastes 3:11.
//   Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

//! KRUSH. Bit depth is an integer step from 1 to 16. The reducer takes a
//! rate in Hz. The ceiling is a digital level limit.

use std::path::{Path, PathBuf};

use clack_plugin::prelude::{clack_export_entry, SinglePluginEntry};
use trust_clap::TrustClap;
use trust_core::{
    Event, PARAM_BITS, PARAM_CEILING_DB_MILLI, PARAM_GAIN_MILLI, PARAM_MIX_MILLI, PARAM_RATE_HZ,
};
use trust_dsp::Krush;
use trust_plugin::TrustProcessor;

#[cfg(debug_assertions)]
#[global_allocator]
static TRUST_ALLOC: assert_no_alloc::AllocDisabler = assert_no_alloc::AllocDisabler;

pub struct KrushProcessor {
    krush: Krush,
}

impl Default for KrushProcessor {
    fn default() -> Self {
        let mut krush = Krush::new(48_000);
        krush.set_bits(8);
        krush.snap_gain(1000);
        krush.snap_mix(1000);
        krush.set_ceiling_db_milli(-1000);
        Self { krush }
    }
}

impl TrustProcessor for KrushProcessor {
    const ID: &'static str = "com.koolskull.trust.krush";
    const NAME: &'static str = "KRUSH";
    const DESCRIPTION: &'static str =
        "Stereo bitcrusher. Bit depth steps, sample-rate reduction in Hz, mix, gain, and a digital ceiling.";

    fn activate(&mut self, sample_rate: u32, _max_frames: u32) {
        self.krush.set_sample_rate(sample_rate);
        self.krush.snap_gain(1000);
        self.krush.snap_mix(1000);
    }

    fn deactivate(&mut self) {}

    fn set_param(&mut self, id: u32, value: i32) {
        match id {
            PARAM_BITS => self.krush.set_bits(value),
            PARAM_RATE_HZ => self.krush.set_rate_hz(value),
            PARAM_GAIN_MILLI => self.krush.set_gain_milli(value),
            PARAM_MIX_MILLI => self.krush.set_mix_milli(value),
            PARAM_CEILING_DB_MILLI => self.krush.set_ceiling_db_milli(value),
            _ => {}
        }
    }

    fn process(
        &mut self,
        input_l: &[f32],
        input_r: &[f32],
        output_l: &mut [f32],
        output_r: &mut [f32],
        events: &[Event],
    ) {
        self.krush
            .process(input_l, input_r, output_l, output_r, events);
    }
}

clack_export_entry!(SinglePluginEntry<TrustClap<KrushProcessor>>);

pub fn hear_wav_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/hear/krush-hear.wav")
}

/// Fixed input, fixed preset. The vec is allocated before the process loop.
pub fn render_hear() -> Vec<f32> {
    const SR: u32 = 48_000;
    const FRAMES: usize = 48_000 * 2;
    let mut input_l = vec![0.0f32; FRAMES];
    let mut input_r = vec![0.0f32; FRAMES];
    for i in 0..FRAMES {
        let t = i as f32 / SR as f32;
        let s = (2.0 * std::f32::consts::PI * 220.0 * t).sin() * 0.35
            + (2.0 * std::f32::consts::PI * 880.0 * t).sin() * 0.15;
        input_l[i] = s;
        input_r[i] = s * 0.8;
    }
    let mut out_l = vec![0.0f32; FRAMES];
    let mut out_r = vec![0.0f32; FRAMES];
    let mut krush = Krush::new(SR);
    krush.set_bits(6);
    krush.set_rate_hz(8_000);
    krush.snap_gain(1000);
    krush.snap_mix(1000);
    krush.set_ceiling_db_milli(-1000);
    let mut offset = 0;
    while offset < FRAMES {
        let n = (FRAMES - offset).min(trust_core::BLOCK_FRAMES);
        let end = offset + n;
        assert_no_alloc::assert_no_alloc(|| {
            krush.process(
                &input_l[offset..end],
                &input_r[offset..end],
                &mut out_l[offset..end],
                &mut out_r[offset..end],
                &[],
            );
        });
        offset = end;
    }
    let mut interleaved = vec![0.0f32; FRAMES * 2];
    for i in 0..FRAMES {
        interleaved[i * 2] = out_l[i];
        interleaved[i * 2 + 1] = out_r[i];
    }
    interleaved
}

pub fn write_wav(path: &Path, interleaved: &[f32], sample_rate: u32) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let frames = interleaved.len() / 2;
    let data_bytes = frames * 4;
    let mut bytes = Vec::with_capacity(44 + data_bytes);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + data_bytes as u32).to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    bytes.extend_from_slice(b"fmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.extend_from_slice(&sample_rate.to_le_bytes());
    bytes.extend_from_slice(&(sample_rate * 4).to_le_bytes());
    bytes.extend_from_slice(&4u16.to_le_bytes());
    bytes.extend_from_slice(&16u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&(data_bytes as u32).to_le_bytes());
    for sample in interleaved {
        let clamped = sample.clamp(-1.0, 1.0);
        let pcm = (clamped * 32767.0).round() as i16;
        bytes.extend_from_slice(&pcm.to_le_bytes());
    }
    std::fs::write(path, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hear_pass_writes_wav() {
        let samples = render_hear();
        let path = hear_wav_path();
        write_wav(&path, &samples, 48_000).unwrap();
        assert!(path.is_file());
        assert!(samples.iter().any(|s| s.abs() > 0.01));
    }
}
