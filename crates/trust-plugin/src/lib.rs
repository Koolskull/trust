//                    ☦
//      ╔════════════════════════════╗
//      ║  Whatever you do, do it    ║
//      ║  in the name of the Lord   ║
//      ║  Jesus.                    ║
//      ╚════════════════════════════╝
//
//   Colossians 3:17. Come, Holy Spirit.
//   Lord Jesus Christ, Son of God, have mercy on us.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

//! Thin processor trait. `trust-clap` is what a CLAP host calls; this trait
//! is what that call runs. MIDI bindings are a mapping layer. They are not
//! graph data.

use trust_core::{
    Event, PARAM_BITS, PARAM_CEILING_DB_MILLI, PARAM_GAIN_MILLI, PARAM_MIX_MILLI, PARAM_RATE_HZ,
};

/// Implemented by a DSP voice or effect. `process` must not allocate.
pub trait TrustProcessor: Send + Default + 'static {
    const ID: &'static str;
    const NAME: &'static str;
    const DESCRIPTION: &'static str;

    fn activate(&mut self, sample_rate: u32, max_frames: u32);
    fn deactivate(&mut self);
    fn set_param(&mut self, id: u32, value: i32);
    fn process(
        &mut self,
        input_l: &[f32],
        input_r: &[f32],
        output_l: &mut [f32],
        output_r: &mut [f32],
        events: &[Event],
    );
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MidiKind {
    Note,
    Cc,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MidiBinding {
    pub kind: MidiKind,
    pub number: u8,
    pub param: u32,
    pub min: i32,
    pub max: i32,
}

/// Built off the audio thread. The audio thread only reads it.
#[derive(Clone, Debug, Default)]
pub struct MidiMap {
    pub bindings: Vec<MidiBinding>,
}

impl MidiMap {
    pub fn krush() -> Self {
        Self {
            bindings: vec![
                MidiBinding {
                    kind: MidiKind::Cc,
                    number: 1,
                    param: PARAM_BITS,
                    min: 1,
                    max: 16,
                },
                MidiBinding {
                    kind: MidiKind::Cc,
                    number: 74,
                    param: PARAM_RATE_HZ,
                    min: 1000,
                    max: 48_000,
                },
                MidiBinding {
                    kind: MidiKind::Cc,
                    number: 7,
                    param: PARAM_GAIN_MILLI,
                    min: 0,
                    max: 1000,
                },
                MidiBinding {
                    kind: MidiKind::Cc,
                    number: 11,
                    param: PARAM_MIX_MILLI,
                    min: 0,
                    max: 1000,
                },
                MidiBinding {
                    kind: MidiKind::Cc,
                    number: 12,
                    param: PARAM_CEILING_DB_MILLI,
                    min: -24_000,
                    max: 0,
                },
                MidiBinding {
                    kind: MidiKind::Note,
                    number: 60,
                    param: PARAM_MIX_MILLI,
                    min: 0,
                    max: 1000,
                },
            ],
        }
    }

    /// Write integer events into `out`. Returns how many were written.
    /// Does not allocate.
    pub fn map_packed(&self, packed: u32, out: &mut [Event]) -> usize {
        let status = (packed & 0xff) as u8;
        let data1 = ((packed >> 8) & 0xff) as u8;
        let data2 = ((packed >> 16) & 0xff) as u8;
        let kind = match status & 0xf0 {
            0x90 if data2 > 0 => Some((MidiKind::Note, data2 as i32)),
            0x80 | 0x90 => Some((MidiKind::Note, 0)),
            0xb0 => Some((MidiKind::Cc, data2 as i32)),
            _ => None,
        };
        let Some((kind, raw)) = kind else {
            return 0;
        };
        let mut written = 0;
        for binding in &self.bindings {
            if written >= out.len() {
                break;
            }
            if binding.kind != kind || binding.number != data1 {
                continue;
            }
            let value = match kind {
                MidiKind::Note => {
                    if raw > 0 {
                        binding.max
                    } else {
                        binding.min
                    }
                }
                MidiKind::Cc => scale(raw, 0, 127, binding.min, binding.max),
            };
            out[written] = Event {
                sample_offset: 0,
                param: binding.param,
                value,
            };
            written += 1;
        }
        written
    }
}

pub fn pack_midi(msg: &[u8]) -> Option<u32> {
    if msg.is_empty() {
        return None;
    }
    let status = msg[0] as u32;
    let d1 = msg.get(1).copied().unwrap_or(0) as u32;
    let d2 = msg.get(2).copied().unwrap_or(0) as u32;
    Some(status | (d1 << 8) | (d2 << 16))
}

fn scale(x: i32, x0: i32, x1: i32, y0: i32, y1: i32) -> i32 {
    if x1 == x0 {
        return y0;
    }
    y0 + (y1 - y0) * (x - x0) / (x1 - x0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cc_and_note_map_to_integers_without_touching_a_graph() {
        let map = MidiMap::krush();
        let mut out = [Event {
            sample_offset: 9,
            param: 0,
            value: 0,
        }; 4];
        let n = map.map_packed(pack_midi(&[0xB0, 1, 0]).unwrap(), &mut out);
        assert_eq!(n, 1);
        assert_eq!(out[0].param, PARAM_BITS);
        assert_eq!(out[0].value, 1);
        assert_eq!(out[0].sample_offset, 0);

        let n = map.map_packed(pack_midi(&[0xB0, 1, 127]).unwrap(), &mut out);
        assert_eq!(out[0].value, 16);
        assert_eq!(n, 1);

        let _n = map.map_packed(pack_midi(&[0xB0, 1, 64]).unwrap(), &mut out);
        assert_eq!(out[0].value, 8);

        let n = map.map_packed(pack_midi(&[0x90, 60, 100]).unwrap(), &mut out);
        assert_eq!(n, 1);
        assert_eq!(out[0].param, PARAM_MIX_MILLI);
        assert_eq!(out[0].value, 1000);

        let n = map.map_packed(pack_midi(&[0x80, 60, 0]).unwrap(), &mut out);
        assert_eq!(out[0].value, 0);
        assert_eq!(n, 1);
    }
}
