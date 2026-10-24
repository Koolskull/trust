//                    ☦
//   Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

//! Single-producer MIDI ring. Push and pop do not allocate or lock.

use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

const N: usize = 256;

pub struct MidiRing {
    buf: [AtomicU32; N],
    write: AtomicUsize,
    read: AtomicUsize,
}

impl MidiRing {
    pub fn new() -> Self {
        Self {
            buf: std::array::from_fn(|_| AtomicU32::new(0)),
            write: AtomicUsize::new(0),
            read: AtomicUsize::new(0),
        }
    }

    pub fn push(&self, packed: u32) -> bool {
        let write = self.write.load(Ordering::Relaxed);
        let next = (write + 1) % N;
        if next == self.read.load(Ordering::Acquire) {
            return false;
        }
        self.buf[write].store(packed, Ordering::Relaxed);
        self.write.store(next, Ordering::Release);
        true
    }

    pub fn pop(&self) -> Option<u32> {
        let read = self.read.load(Ordering::Relaxed);
        if read == self.write.load(Ordering::Acquire) {
            return None;
        }
        let value = self.buf[read].load(Ordering::Relaxed);
        self.read.store((read + 1) % N, Ordering::Release);
        Some(value)
    }
}

impl Default for MidiRing {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_keeps_order_and_does_not_open_a_device() {
        let ring = MidiRing::new();
        assert!(ring.push(1));
        assert!(ring.push(2));
        assert_eq!(ring.pop(), Some(1));
        assert_eq!(ring.pop(), Some(2));
        assert_eq!(ring.pop(), None);
    }
}
