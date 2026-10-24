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

//! Terminal lesson for TRUST.
//!
//! `KOS TRUST LESSON 3yo` starts the lesson for that age. Ages 0 through 14
//! open one guided lesson. Age 15 and older opens the class list and the
//! student picks a subject.

mod app;
mod parse;

pub use app::{render, render_block, App, ClassId, Key, Screen, Sound, Speaker, Voice};
pub use parse::{parse_cli, usage_text, Cli, Launch, UsageError};

/// Ages this old and older choose a class. Younger ages get one lesson.
pub const FULL_CLASS_AGE: u8 = 15;

/// Oldest age the command accepts.
pub const MAX_AGE: u8 = 120;
