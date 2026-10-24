//                    ☦
//         ╔════════════════════╗
//         ║  Let the children  ║
//         ║  come to me.       ║
//         ╚════════════════════╝
//
//   Mark 10:14. Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

//! Thrash 0.6.0. One phrase on eight channels.
//!
//! The JUCE tracker stays 0.5.3. This slice does not open a `.dm2` song.

mod engine;
mod phrase;
mod screen;

pub use engine::{samples_per_row, Engine, BPM, ROWS_PER_BEAT};
pub use phrase::{dump, note_midi, note_name, usage, Edit, Phrase, CHANNELS, EMPTY, ROWS};
pub use screen::{cell_at, esc_at, layout, paint, Layout, Spot, BLACK, WHITE};

/// Product version of the TRUST remake. The JUCE app remains 0.5.3.
pub const VERSION: &str = "0.6.0";
