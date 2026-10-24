//                    ☦
//          ╔═════════════════╗
//          ║  Let the        ║
//          ║  children come  ║
//          ║  to me.         ║
//          ╚═════════════════╝
//
//   Mark 10:14. Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

//! Fullscreen lesson for TRUST.
//!
//! `KOS TRUST LESSON 3yo` opens a black-and-white board for that age.
//! Every key plays. ESC leaves. Pen and touch hit the same keys.
//! Age 15 and older can switch crush, hold, level, pads, MIDI, and build.

mod app;
mod parse;
mod play;

pub use app::{
    level_cap, render, render_block, App, ClassId, Key, LevelCap, Screen, Sound, Speaker, Voice,
};
pub use parse::{parse_cli, usage_text, Cli, Launch, UsageError};
pub use play::{
    action_for, label_for, layout, mode_from_id, paint, sound_for, spot_at, Action, BoardMode,
    KeySpot, EXIT_ID,
};

/// Ages this old and older choose a class. Younger ages get one lesson.
pub const FULL_CLASS_AGE: u8 = 15;

/// Oldest age the command accepts.
pub const MAX_AGE: u8 = 120;
