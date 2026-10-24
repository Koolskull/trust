//                    ☦
//         ╔════════════════════╗
//         ║  ST. CECILIA       ║
//         ║  pray with us      ║
//         ╚════════════════════╝
//
//   "Sing to the Lord a new song." — Psalm 96:1
//   Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

//! KRUSH. Bit depth steps. The sample-rate reducer takes a rate in Hz.
//! The ceiling is a digital level limit, not a hearing-safety claim.

#![no_std]

use trust_core::{
    db_milli_to_lin, flush_denormal, milli_to_unit, Event, Smoother, PARAM_BITS,
    PARAM_CEILING_DB_MILLI, PARAM_GAIN_MILLI, PARAM_MIX_MILLI, PARAM_RATE_HZ,
};

/// Quantize to `bits` in 1..=16. `bits` is the step count, not a glide.
pub fn crush_sample(sample: f32, bits: u8) -> f32 {
    let bits = bits.clamp(1, 16);
    let steps = (1u32 << (bits - 1)) as f32;
    let x = sample.clamp(-1.0, 1.0);
    let q = libm::roundf(x * steps);
    (q / steps).clamp(-1.0, 1.0)
}

/// How many input frames share one output value. A target at or above the
/// sample rate holds for one frame (no reduction).
pub fn hold_frames(sample_rate: u32, target_hz: u32) -> u32 {
    if sample_rate == 0 || target_hz == 0 || target_hz >= sample_rate {
        1
    } else {
        (sample_rate / target_hz).max(1)
    }
}

#[derive(Clone, Debug)]
struct Reducer {
    hold: u32,
    count: u32,
    last_l: f32,
    last_r: f32,
    primed: bool,
}

impl Reducer {
    const fn new() -> Self {
        Self {
            hold: 1,
            count: 0,
            last_l: 0.0,
            last_r: 0.0,
            primed: false,
        }
    }

    fn set_hold(&mut self, hold: u32) {
        self.hold = hold.max(1);
    }

    fn next(&mut self, l: f32, r: f32) -> (f32, f32) {
        if !self.primed || self.count == 0 {
            self.last_l = l;
            self.last_r = r;
            self.primed = true;
            self.count = self.hold;
        }
        self.count -= 1;
        (self.last_l, self.last_r)
    }
}

/// 4x linear peak between samples, including the boundary from `prev`.
pub fn true_peak_linear(prev: f32, buf: &[f32]) -> f32 {
    let mut peak = prev.abs();
    let mut last = prev;
    for &s in buf {
        peak = peak.max(s.abs());
        for k in 1..4 {
            let t = k as f32 * 0.25;
            let v = last + (s - last) * t;
            peak = peak.max(v.abs());
        }
        last = s;
    }
    peak
}

/// Stereo KRUSH. Construct off the audio thread. `process` does not allocate.
#[derive(Clone, Debug)]
pub struct Krush {
    sample_rate: u32,
    bits: u8,
    rate_hz: u32,
    gain: Smoother,
    mix: Smoother,
    ceiling_db_milli: i32,
    reducer: Reducer,
    prev_l: f32,
    prev_r: f32,
}

impl Krush {
    pub const fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            bits: 16,
            rate_hz: sample_rate,
            gain: Smoother::new(1.0),
            mix: Smoother::new(1.0),
            ceiling_db_milli: 0,
            reducer: Reducer::new(),
            prev_l: 0.0,
            prev_r: 0.0,
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate.max(1);
        self.reducer
            .set_hold(hold_frames(self.sample_rate, self.rate_hz));
    }

    pub fn bits(&self) -> u8 {
        self.bits
    }

    pub fn rate_hz(&self) -> u32 {
        self.rate_hz
    }

    /// Integer step. Applied on the next sample that reads `self.bits`. No ramp.
    pub fn set_bits(&mut self, bits: i32) {
        self.bits = bits.clamp(1, 16) as u8;
    }

    pub fn set_rate_hz(&mut self, hz: i32) {
        self.rate_hz = hz.max(0) as u32;
        self.reducer
            .set_hold(hold_frames(self.sample_rate, self.rate_hz));
    }

    pub fn set_gain_milli(&mut self, milli: i32) {
        let ramp = self.sample_rate / 200;
        self.gain.set_target(milli_to_unit(milli), ramp.max(1));
    }

    pub fn set_mix_milli(&mut self, milli: i32) {
        let ramp = self.sample_rate / 200;
        let m = milli_to_unit(milli.clamp(0, 1000));
        self.mix.set_target(m, ramp.max(1));
    }

    pub fn set_ceiling_db_milli(&mut self, db_milli: i32) {
        self.ceiling_db_milli = db_milli.min(0);
    }

    /// Jump gain. No ramp. Off or on the audio thread; it does not allocate.
    pub fn snap_gain(&mut self, milli: i32) {
        self.gain.snap(milli_to_unit(milli));
    }

    /// Jump mix. Clamped to 0..=1000 milli. No ramp.
    pub fn snap_mix(&mut self, milli: i32) {
        self.mix.snap(milli_to_unit(milli.clamp(0, 1000)));
    }

    fn apply_param(&mut self, param: u32, value: i32) {
        match param {
            PARAM_BITS => self.set_bits(value),
            PARAM_RATE_HZ => self.set_rate_hz(value),
            PARAM_GAIN_MILLI => self.set_gain_milli(value),
            PARAM_MIX_MILLI => self.set_mix_milli(value),
            PARAM_CEILING_DB_MILLI => self.set_ceiling_db_milli(value),
            _ => {}
        }
    }

    /// `events` must be ordered by `sample_offset`. Offsets past the block are ignored.
    pub fn process(
        &mut self,
        input_l: &[f32],
        input_r: &[f32],
        output_l: &mut [f32],
        output_r: &mut [f32],
        events: &[Event],
    ) {
        let n = input_l
            .len()
            .min(input_r.len())
            .min(output_l.len())
            .min(output_r.len());
        let mut ev = 0usize;
        let mut i = 0usize;
        while i < n {
            while ev < events.len() && events[ev].sample_offset as usize <= i {
                self.apply_param(events[ev].param, events[ev].value);
                ev += 1;
            }
            let end = if ev < events.len() {
                (events[ev].sample_offset as usize).min(n)
            } else {
                n
            };
            if end <= i {
                // A duplicate offset was already applied. Advance one sample
                // so a zero-length span cannot spin.
                self.render_span(input_l, input_r, output_l, output_r, i, i + 1);
                i += 1;
                continue;
            }
            self.render_span(input_l, input_r, output_l, output_r, i, end);
            i = end;
        }
    }

    fn render_span(
        &mut self,
        input_l: &[f32],
        input_r: &[f32],
        output_l: &mut [f32],
        output_r: &mut [f32],
        start: usize,
        end: usize,
    ) {
        let bits = self.bits;
        let ceiling = db_milli_to_lin(self.ceiling_db_milli);
        for i in start..end {
            let dry_l = input_l[i];
            let dry_r = input_r[i];
            let (held_l, held_r) = self.reducer.next(dry_l, dry_r);
            let wet_l = crush_sample(held_l, bits);
            let wet_r = crush_sample(held_r, bits);
            let g = self.gain.next();
            let m = self.mix.next().clamp(0.0, 1.0);
            let mixed_l = dry_l * (1.0 - m) + wet_l * g * m;
            let mixed_r = dry_r * (1.0 - m) + wet_r * g * m;
            output_l[i] = mixed_l;
            output_r[i] = mixed_r;
        }
        let peak = true_peak_linear(self.prev_l, &output_l[start..end])
            .max(true_peak_linear(self.prev_r, &output_r[start..end]));
        if peak > ceiling && peak > 0.0 {
            let trim = ceiling / peak;
            for i in start..end {
                output_l[i] *= trim;
                output_r[i] *= trim;
            }
        }
        for i in start..end {
            output_l[i] = flush_denormal(output_l[i]);
            output_r[i] = flush_denormal(output_r[i]);
        }
        if end > start {
            self.prev_l = output_l[end - 1];
            self.prev_r = output_r[end - 1];
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;

    fn sine(n: usize, freq: f32, sr: f32) -> (std::vec::Vec<f32>, std::vec::Vec<f32>) {
        // Vec is the test harness, not the audio thread.
        extern crate std;
        use std::vec::Vec;
        let mut l = Vec::with_capacity(n);
        let mut r = Vec::with_capacity(n);
        for i in 0..n {
            let s = libm::sinf(2.0 * core::f32::consts::PI * freq * i as f32 / sr);
            l.push(s * 0.25);
            r.push(s * 0.25);
        }
        (l, r)
    }

    #[test]
    fn null_when_mix_is_dry() {
        let (l, r) = sine(256, 440.0, 48000.0);
        let mut out_l = std::vec![0.0; l.len()];
        let mut out_r = std::vec![0.0; r.len()];
        let mut k = Krush::new(48000);
        k.set_mix_milli(0);
        k.mix.snap(0.0);
        k.set_bits(3);
        k.set_rate_hz(1000);
        k.process(&l, &r, &mut out_l, &mut out_r, &[]);
        for i in 0..l.len() {
            assert_eq!(out_l[i], l[i], "left {i}");
            assert_eq!(out_r[i], r[i], "right {i}");
        }
    }

    #[test]
    fn bit_depth_steps_without_smoothing() {
        let n = 32;
        let l = std::vec![0.4; n];
        let r = std::vec![0.4; n];
        let mut out_l = std::vec![0.0; n];
        let mut out_r = std::vec![0.0; n];
        let mut k = Krush::new(48000);
        k.set_bits(4);
        k.set_mix_milli(1000);
        k.mix.snap(1.0);
        k.gain.snap(1.0);
        k.set_rate_hz(48000);
        let events = [Event {
            sample_offset: 10,
            param: PARAM_BITS,
            value: 8,
        }];
        k.process(&l, &r, &mut out_l, &mut out_r, &events);
        let q4 = crush_sample(0.4, 4);
        let q8 = crush_sample(0.4, 8);
        assert!((out_l[9] - q4).abs() < 1e-6);
        assert!((out_l[10] - q8).abs() < 1e-6);
        assert!((q4 - q8).abs() > 1e-4);
    }

    #[test]
    fn reducer_holds_and_full_rate_does_not() {
        let (l, r) = sine(480, 12000.0, 48000.0);
        let mut out_l = std::vec![0.0; l.len()];
        let mut out_r = std::vec![0.0; r.len()];
        let mut k = Krush::new(48000);
        k.mix.snap(1.0);
        k.gain.snap(1.0);
        k.set_bits(16);
        k.set_rate_hz(3000);
        k.process(&l, &r, &mut out_l, &mut out_r, &[]);
        let hold = hold_frames(48000, 3000) as usize;
        assert!(hold > 1);
        let mut plateau = false;
        for i in 1..hold.min(out_l.len()) {
            if out_l[i] == out_l[0] {
                plateau = true;
            }
        }
        assert!(plateau);
        let mut differed = false;
        for i in 0..l.len() {
            if (out_l[i] - crush_sample(l[i], 16)).abs() > 1e-4 {
                differed = true;
                break;
            }
        }
        assert!(differed, "reduction must move a high sine");

        let mut full_l = std::vec![0.0; l.len()];
        let mut full_r = std::vec![0.0; r.len()];
        let mut full = Krush::new(48000);
        full.mix.snap(1.0);
        full.gain.snap(1.0);
        full.set_bits(16);
        full.set_rate_hz(48000);
        full.process(&l, &r, &mut full_l, &mut full_r, &[]);
        for i in 0..l.len() {
            let expect = crush_sample(l[i], 16);
            assert!((full_l[i] - expect).abs() < 1e-5);
        }
    }

    #[test]
    fn denormal_input_flushes() {
        let l = std::vec![1.0e-40; 64];
        let r = std::vec![1.0e-40; 64];
        let mut out_l = std::vec![1.0; 64];
        let mut out_r = std::vec![1.0; 64];
        let mut k = Krush::new(48000);
        k.mix.snap(1.0);
        k.gain.snap(1.0);
        k.set_bits(16);
        k.set_rate_hz(48000);
        k.process(&l, &r, &mut out_l, &mut out_r, &[]);
        assert!(out_l.iter().all(|s| *s == 0.0));
        assert!(out_r.iter().all(|s| *s == 0.0));
    }

    #[test]
    fn ceiling_pulls_a_hot_signal_down() {
        let l = std::vec![1.0; 128];
        let r = std::vec![1.0; 128];
        let mut out_l = std::vec![0.0; 128];
        let mut out_r = std::vec![0.0; 128];
        let mut k = Krush::new(48000);
        k.mix.snap(1.0);
        k.gain.snap(1.0);
        k.set_bits(16);
        k.set_rate_hz(48000);
        k.set_ceiling_db_milli(-1000);
        k.process(&l, &r, &mut out_l, &mut out_r, &[]);
        let limit = db_milli_to_lin(-1000) + 1e-4;
        let peak = true_peak_linear(0.0, &out_l).max(true_peak_linear(0.0, &out_r));
        assert!(peak <= limit, "peak {peak} limit {limit}");
        assert!(peak > 0.5);
    }
}
