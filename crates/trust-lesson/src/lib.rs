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
//! `KOS TRUST LESSON 3yo` opens big letters, then a black-and-white board.
//! From age 5 the letters are a lesson you click through.
//! Younger ages get silly, varied keys. Older ages add controls.
//! ESC leaves. Pen and touch hit the same keys.

mod app;
mod book;
mod parse;
mod play;

pub use app::{
    level_cap, render, render_block, App, ClassId, Key, LevelCap, Screen, Sound, Speaker, Voice,
};
pub use book::{lesson_cards, Card, READING_AGE};
pub use parse::{parse_cli, usage_text, Cli, Launch, UsageError};
pub use play::{
    action_for, book_spots, digit_choice, knob_for, label_for, layout, layout_for, mode_from_id,
    paint, paint_book, played_sound, press_choice, selected_choices, silly_name, sound_for,
    spot_at, Action, BoardMode, KeySpot, Knob, BACK_ID, EXIT_ID, LESSON_ID, NEXT_ID,
};

/// Ages this old and older choose a class. Younger ages get one lesson.
pub const FULL_CLASS_AGE: u8 = 15;

/// Oldest age the command accepts.
pub const MAX_AGE: u8 = 120;
