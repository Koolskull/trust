//                    ☦
//   Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

use crate::VERSION;

pub const CHANNELS: usize = 8;
pub const ROWS: usize = 16;
/// No note in the cell.
pub const EMPTY: u8 = 255;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edit {
    Left,
    Right,
    Up,
    Down,
    Clear,
    Play,
    OctaveDown,
    OctaveUp,
    Note(u8),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Phrase {
    notes: [[u8; CHANNELS]; ROWS],
    channel: usize,
    row: usize,
    octave: u8,
    playing: bool,
}

impl Phrase {
    pub fn new() -> Self {
        Self {
            notes: [[EMPTY; CHANNELS]; ROWS],
            channel: 0,
            row: 0,
            octave: 4,
            playing: false,
        }
    }

    pub fn note(&self, channel: usize, row: usize) -> u8 {
        self.notes[row][channel]
    }

    pub fn channel(&self) -> usize {
        self.channel
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn octave(&self) -> u8 {
        self.octave
    }

    pub fn playing(&self) -> bool {
        self.playing
    }

    pub fn notes(&self) -> &[[u8; CHANNELS]; ROWS] {
        &self.notes
    }

    /// Move the edit cursor. A pen tap uses this and does not write a pitch.
    pub fn select(&mut self, channel: usize, row: usize) {
        if channel < CHANNELS {
            self.channel = channel;
        }
        if row < ROWS {
            self.row = row;
        }
    }

    /// Returns true when playback should restart at row 0.
    pub fn edit(&mut self, edit: Edit) -> bool {
        match edit {
            Edit::Left => self.channel = (self.channel + CHANNELS - 1) % CHANNELS,
            Edit::Right => self.channel = (self.channel + 1) % CHANNELS,
            Edit::Up => self.row = (self.row + ROWS - 1) % ROWS,
            Edit::Down => self.row = (self.row + 1) % ROWS,
            Edit::Clear => {
                self.notes[self.row][self.channel] = EMPTY;
                self.row = (self.row + 1) % ROWS;
            }
            Edit::Play => {
                self.playing = !self.playing;
                return self.playing;
            }
            Edit::OctaveDown => self.octave = self.octave.saturating_sub(1).max(1),
            Edit::OctaveUp => self.octave = (self.octave + 1).min(7),
            Edit::Note(note) => {
                self.notes[self.row][self.channel] = note.min(127);
                self.row = (self.row + 1) % ROWS;
            }
        }
        false
    }
}

impl Default for Phrase {
    fn default() -> Self {
        Self::new()
    }
}

/// Tracker keyboard. `z` is C of `octave`. `q` is C of the next octave.
pub fn note_midi(token: &str, octave: u8) -> Option<u8> {
    let (semi, upper) = match token {
        "z" => (0, false),
        "s" => (1, false),
        "x" => (2, false),
        "d" => (3, false),
        "c" => (4, false),
        "v" => (5, false),
        "g" => (6, false),
        "b" => (7, false),
        "h" => (8, false),
        "n" => (9, false),
        "j" => (10, false),
        "m" => (11, false),
        "q" => (0, true),
        "2" => (1, true),
        "w" => (2, true),
        "3" => (3, true),
        "e" => (4, true),
        "r" => (5, true),
        "5" => (6, true),
        "t" => (7, true),
        "6" => (8, true),
        "y" => (9, true),
        "7" => (10, true),
        "u" => (11, true),
        _ => return None,
    };
    let oct = u16::from(octave).saturating_add(u16::from(upper));
    let midi = (oct + 1) * 12 + semi;
    u8::try_from(midi).ok().filter(|note| *note <= 127)
}

pub fn note_name(note: u8) -> String {
    if note == EMPTY || note > 127 {
        return "---".to_string();
    }
    const NAMES: [&str; 12] = [
        "C-", "C#", "D-", "D#", "E-", "F-", "F#", "G-", "G#", "A-", "A#", "B-",
    ];
    let name = NAMES[usize::from(note) % 12];
    let oct = i32::from(note / 12) - 1;
    format!("{name}{oct}")
}

pub fn dump(phrase: &Phrase) -> String {
    let mut out = format!("THRASH {VERSION}\n");
    out.push_str("phrase  16 rows  8 channels  120 bpm\n");
    out.push_str(if phrase.playing { "PLAY\n" } else { "STOP\n" });
    out.push_str(&format!("octave {}\n\n", phrase.octave));
    out.push_str("    1   2   3   4   5   6   7   8\n");
    for row in 0..ROWS {
        let mark = if row == phrase.row { '>' } else { ' ' };
        out.push_str(&format!("{mark}{row:02}"));
        for channel in 0..CHANNELS {
            out.push(' ');
            out.push_str(&note_name(phrase.note(channel, row)));
        }
        out.push('\n');
    }
    out
}

pub fn usage() -> &'static str {
    "\
trust-tracker

Thrash 0.6.0. One phrase. Eight channels. One oscillator through KRUSH.
The JUCE tracker stays 0.5.3. This screen does not open a .dm2 song.

Arrows move. Z writes C of the octave. Q writes the next octave.
Space starts and stops. [ and ] change octave. Backspace clears.
A pen tap selects a cell. ESC leaves.

  trust-tracker
  trust-tracker --dump
"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_0_6_0() {
        assert_eq!(VERSION, "0.6.0");
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.6.0");
    }

    #[test]
    fn z_writes_c4_and_steps_the_row() {
        let mut phrase = Phrase::new();
        let note = note_midi("z", phrase.octave()).unwrap();
        assert_eq!(note, 60);
        assert_eq!(note_name(note), "C-4");
        phrase.edit(Edit::Note(note));
        assert_eq!(phrase.note(0, 0), 60);
        assert_eq!(phrase.row(), 1);
        assert_eq!(note_midi("q", 4), Some(72));
        assert_eq!(note_midi("s", 4), Some(61));
    }

    #[test]
    fn arrows_wrap_and_space_restarts_only_when_starting() {
        let mut phrase = Phrase::new();
        phrase.edit(Edit::Left);
        assert_eq!(phrase.channel(), CHANNELS - 1);
        phrase.edit(Edit::Up);
        assert_eq!(phrase.row(), ROWS - 1);
        assert!(phrase.edit(Edit::Play));
        assert!(phrase.playing());
        assert!(!phrase.edit(Edit::Play));
        assert!(!phrase.playing());
        phrase.select(3, 4);
        assert_eq!((phrase.channel(), phrase.row()), (3, 4));
        assert_eq!(phrase.note(3, 4), EMPTY);
    }
}
