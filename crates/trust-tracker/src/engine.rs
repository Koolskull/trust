//                    ☦
//   Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

use trust_core::BLOCK_FRAMES;
use trust_dsp::Krush;

use crate::phrase::{CHANNELS, EMPTY, ROWS};

pub const BPM: u32 = 120;
pub const ROWS_PER_BEAT: u32 = 4;

/// Samples in one phrase row at `sample_rate`. 120 bpm and 4 rows a beat.
pub fn samples_per_row(sample_rate: u32) -> u32 {
    let rows_per_second = (BPM / 60) * ROWS_PER_BEAT;
    (sample_rate.max(1) / rows_per_second.max(1)).max(1)
}

/// Eight oscillators summed into one KRUSH. `render` does not allocate.
pub struct Engine {
    sample_rate: u32,
    into_row: u32,
    row: usize,
    phases: [u32; CHANNELS],
    steps: [u32; CHANNELS],
    krush: Krush,
}

impl Engine {
    pub fn new(sample_rate: u32) -> Self {
        let sample_rate = sample_rate.max(1);
        let mut krush = Krush::new(sample_rate);
        krush.set_bits(8);
        krush.set_rate_hz(8_000);
        krush.snap_gain(450);
        krush.snap_mix(1_000);
        krush.set_ceiling_db_milli(-6_000);
        Self {
            sample_rate,
            into_row: 0,
            row: 0,
            phases: [0; CHANNELS],
            steps: [0; CHANNELS],
            krush,
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn reset(&mut self) {
        self.into_row = 0;
        self.row = 0;
        self.phases = [0; CHANNELS];
        self.steps = [0; CHANNELS];
    }

    pub fn render(
        &mut self,
        playing: bool,
        notes: &[[u8; CHANNELS]; ROWS],
        out_l: &mut [f32],
        out_r: &mut [f32],
    ) {
        let n = out_l.len().min(out_r.len());
        if !playing {
            out_l[..n].fill(0.0);
            out_r[..n].fill(0.0);
            return;
        }
        let per = samples_per_row(self.sample_rate);
        let mut done = 0;
        while done < n {
            if self.into_row == 0 {
                self.arm_notes(&notes[self.row]);
            }
            let room = (per - self.into_row) as usize;
            let chunk = (n - done).min(room).min(BLOCK_FRAMES);
            self.span(
                &mut out_l[done..done + chunk],
                &mut out_r[done..done + chunk],
            );
            self.into_row += chunk as u32;
            done += chunk;
            if self.into_row >= per {
                self.into_row = 0;
                self.row = (self.row + 1) % ROWS;
            }
        }
    }

    fn arm_notes(&mut self, notes: &[u8; CHANNELS]) {
        for channel in 0..CHANNELS {
            let note = notes[channel];
            self.steps[channel] = if note == EMPTY {
                0
            } else {
                midi_step(note, self.sample_rate)
            };
            self.phases[channel] = 0;
        }
    }

    fn span(&mut self, out_l: &mut [f32], out_r: &mut [f32]) {
        let n = out_l.len().min(out_r.len()).min(BLOCK_FRAMES);
        let mut input_l = [0.0f32; BLOCK_FRAMES];
        let mut input_r = [0.0f32; BLOCK_FRAMES];
        let mut left = [0.0f32; BLOCK_FRAMES];
        let mut right = [0.0f32; BLOCK_FRAMES];
        let scale = 1.0 / CHANNELS as f32;
        for i in 0..n {
            let mut sample = 0.0f32;
            for channel in 0..CHANNELS {
                if self.steps[channel] == 0 {
                    continue;
                }
                sample += sine(self.phases[channel]);
                self.phases[channel] = self.phases[channel].wrapping_add(self.steps[channel]);
            }
            let mixed = sample * scale;
            input_l[i] = mixed;
            input_r[i] = mixed;
        }
        self.krush.process(
            &input_l[..n],
            &input_r[..n],
            &mut left[..n],
            &mut right[..n],
            &[],
        );
        out_l[..n].copy_from_slice(&left[..n]);
        out_r[..n].copy_from_slice(&right[..n]);
    }
}

fn midi_step(note: u8, sample_rate: u32) -> u32 {
    let hz = 440.0 * libm::powf(2.0, (f32::from(note) - 69.0) / 12.0);
    let hz = hz.max(1.0) as u64;
    ((hz << 32) / u64::from(sample_rate.max(1))) as u32
}

fn sine(phase: u32) -> f32 {
    libm::sinf(phase as f32 * (core::f32::consts::TAU / 4_294_967_296.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phrase::EMPTY;

    #[test]
    fn a_stopped_phrase_is_silent_and_a_note_loops() {
        let mut engine = Engine::new(48_000);
        let mut notes = [[EMPTY; CHANNELS]; ROWS];
        let mut left = [0.0f32; 256];
        let mut right = [0.0f32; 256];
        engine.render(false, &notes, &mut left, &mut right);
        assert!(left.iter().all(|sample| sample.abs() < 1.0e-6));
        notes[0][0] = 60;
        engine.reset();
        let per = samples_per_row(48_000) as usize;
        let mut block = vec![0.0f32; per];
        let mut other = vec![0.0f32; per];
        engine.render(true, &notes, &mut block, &mut other);
        assert_eq!(engine.row(), 1);
        let peak = block
            .iter()
            .fold(0.0f32, |peak, sample| peak.max(sample.abs()));
        assert!(peak > 0.005, "{peak}");
        assert!(peak < 1.0, "{peak}");
        let whole = per * ROWS;
        let mut song_l = vec![0.0f32; whole];
        let mut song_r = vec![0.0f32; whole];
        engine.reset();
        engine.render(true, &notes, &mut song_l, &mut song_r);
        assert_eq!(engine.row(), 0);
    }
}
