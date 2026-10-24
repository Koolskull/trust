//                    ☦
//          ╔═════════════════╗
//          ║  In the name of ║
//          ║  the Father,    ║
//          ║  the Son, and   ║
//          ║  the Holy Spirit║
//          ╚═════════════════╝
//
//   Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

//! Audio-thread primitives for TRUST.
//!
//! `no_std` + `alloc`. Nothing in this crate takes a lock or performs a
//! syscall. Smoothers and the event queue are fixed storage. Allocate only
//! through [`events_to_vec`], and only off the audio thread.

#![no_std]

extern crate alloc;

use alloc::vec::Vec;

/// Block size used by the native host request and the WASM proof.
pub const BLOCK_FRAMES: usize = 128;

/// Fixed stereo block. Stack storage. `clear` does not allocate.
#[derive(Clone, Debug)]
pub struct StereoBlock<const N: usize> {
    pub left: [f32; N],
    pub right: [f32; N],
}

impl<const N: usize> StereoBlock<N> {
    pub const fn new() -> Self {
        Self {
            left: [0.0; N],
            right: [0.0; N],
        }
    }

    pub fn clear(&mut self) {
        self.left.fill(0.0);
        self.right.fill(0.0);
    }
}

impl<const N: usize> Default for StereoBlock<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// KRUSH parameter ids. Values are integers. Gain, mix, and the ceiling use
/// milli-units. Bit depth and rate are the integer itself.
pub const PARAM_BITS: u32 = 1;
pub const PARAM_RATE_HZ: u32 = 2;
pub const PARAM_GAIN_MILLI: u32 = 3;
pub const PARAM_MIX_MILLI: u32 = 4;
pub const PARAM_CEILING_DB_MILLI: u32 = 5;

/// Linear ramp. Gain and mix use this. Bit depth must not.
#[derive(Clone, Copy, Debug)]
pub struct Smoother {
    current: f32,
    target: f32,
    step: f32,
    left: u32,
}

impl Smoother {
    pub const fn new(value: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            left: 0,
        }
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn target(&self) -> f32 {
        self.target
    }

    /// Jump. Used when a value must not glide (callers still must not route
    /// bit depth through a smoother).
    pub fn snap(&mut self, value: f32) {
        self.current = value;
        self.target = value;
        self.step = 0.0;
        self.left = 0;
    }

    pub fn set_target(&mut self, target: f32, ramp_samples: u32) {
        self.target = target;
        if ramp_samples == 0 || (target - self.current).abs() == 0.0 {
            self.snap(target);
            return;
        }
        self.step = (target - self.current) / ramp_samples as f32;
        self.left = ramp_samples;
    }

    pub fn next(&mut self) -> f32 {
        if self.left > 0 {
            self.current += self.step;
            self.left -= 1;
            if self.left == 0 {
                self.current = self.target;
            }
        }
        self.current
    }
}

/// One sample-accurate change. `value` is an integer (milli-units or a raw step).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Event {
    pub sample_offset: u32,
    pub param: u32,
    pub value: i32,
}

/// Fixed queue. Push fails when full instead of allocating.
#[derive(Clone, Debug)]
pub struct EventQueue<const N: usize> {
    items: [Event; N],
    len: usize,
}

impl<const N: usize> EventQueue<N> {
    pub const fn new() -> Self {
        Self {
            items: [Event {
                sample_offset: 0,
                param: 0,
                value: 0,
            }; N],
            len: 0,
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, event: Event) -> Result<(), Event> {
        if self.len >= N {
            return Err(event);
        }
        self.items[self.len] = event;
        self.len += 1;
        Ok(())
    }

    pub fn as_slice(&self) -> &[Event] {
        &self.items[..self.len]
    }
}

impl<const N: usize> Default for EventQueue<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Owned copy for hosts and tests. Not for the audio thread.
pub fn events_to_vec(events: &[Event]) -> Vec<Event> {
    events.to_vec()
}

/// Subnormal, NaN, and Inf become 0. Zero stays zero.
#[inline]
pub fn flush_denormal(x: f32) -> f32 {
    if !x.is_finite() {
        return 0.0;
    }
    let exp = (x.to_bits() >> 23) & 0xff;
    if exp == 0 { 0.0 } else { x }
}

pub fn milli_to_unit(milli: i32) -> f32 {
    milli as f32 / 1000.0
}

/// `db_milli` of -1000 is -1.0 dB. 0 is unity.
pub fn db_milli_to_lin(db_milli: i32) -> f32 {
    let db = db_milli as f32 / 1000.0;
    libm::expf(db / 20.0 * core::f32::consts::LN_10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoother_reaches_target_and_snap_does_not_glide() {
        let mut s = Smoother::new(0.0);
        s.set_target(1.0, 4);
        let mut last = 0.0;
        for _ in 0..4 {
            last = s.next();
        }
        assert!((last - 1.0).abs() < 1e-5);
        s.snap(0.25);
        assert_eq!(s.next(), 0.25);
    }

    #[test]
    fn event_queue_does_not_grow_past_capacity() {
        let mut q = EventQueue::<2>::new();
        assert!(q
            .push(Event {
                sample_offset: 0,
                param: PARAM_BITS,
                value: 4
            })
            .is_ok());
        assert!(q
            .push(Event {
                sample_offset: 1,
                param: PARAM_BITS,
                value: 8
            })
            .is_ok());
        assert!(q
            .push(Event {
                sample_offset: 2,
                param: PARAM_BITS,
                value: 12
            })
            .is_err());
        assert_eq!(events_to_vec(q.as_slice()).len(), 2);
    }

    #[test]
    fn stereo_block_clear_is_silent() {
        let mut block = StereoBlock::<BLOCK_FRAMES>::new();
        block.left[0] = 0.5;
        block.right[3] = -0.25;
        block.clear();
        assert!(block.left.iter().all(|s| *s == 0.0));
        assert!(block.right.iter().all(|s| *s == 0.0));
    }

    #[test]
    fn flush_clears_subnormals() {
        assert_eq!(flush_denormal(0.0), 0.0);
        assert_eq!(flush_denormal(1.0e-40), 0.0);
        assert_eq!(flush_denormal(f32::NAN), 0.0);
        assert_eq!(flush_denormal(0.5), 0.5);
    }
}
