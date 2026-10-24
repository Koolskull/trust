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

use crate::book::{Card, READING_AGE};
use crate::{level_cap, Sound, FULL_CLASS_AGE};

/// On-screen and physical Escape. Tapping this key leaves the lesson.
pub const EXIT_ID: u16 = 1;

/// Step back one lesson page.
pub const BACK_ID: u16 = 801;

/// Step forward, or start the keys on the last page.
pub const NEXT_ID: u16 = 802;

/// Leave the keys and return to the lesson pages.
pub const LESSON_ID: u16 = 803;

const MODE_BASE: u16 = 900;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Exit,
    Play(u16),
    Mode(BoardMode),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardMode {
    Lesson,
    Crush,
    Hold,
    Level,
    Pads,
    Midi,
    Build,
}

impl BoardMode {
    pub fn name(self) -> &'static str {
        match self {
            BoardMode::Lesson => "play",
            BoardMode::Crush => "crush",
            BoardMode::Hold => "hold",
            BoardMode::Level => "level",
            BoardMode::Pads => "pads",
            BoardMode::Midi => "midi",
            BoardMode::Build => "build",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeySpot {
    pub id: u16,
    pub label: &'static str,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

struct Def {
    id: u16,
    label: &'static str,
    row: u8,
    weight: u8,
}

const KEYS: &[Def] = &[
    Def {
        id: EXIT_ID,
        label: "ESC",
        row: 0,
        weight: 3,
    },
    Def {
        id: 2,
        label: "F1",
        row: 0,
        weight: 2,
    },
    Def {
        id: 3,
        label: "F2",
        row: 0,
        weight: 2,
    },
    Def {
        id: 4,
        label: "F3",
        row: 0,
        weight: 2,
    },
    Def {
        id: 5,
        label: "F4",
        row: 0,
        weight: 2,
    },
    Def {
        id: 6,
        label: "F5",
        row: 0,
        weight: 2,
    },
    Def {
        id: 7,
        label: "F6",
        row: 0,
        weight: 2,
    },
    Def {
        id: 8,
        label: "F7",
        row: 0,
        weight: 2,
    },
    Def {
        id: 9,
        label: "F8",
        row: 0,
        weight: 2,
    },
    Def {
        id: 10,
        label: "F9",
        row: 0,
        weight: 2,
    },
    Def {
        id: 11,
        label: "F10",
        row: 0,
        weight: 2,
    },
    Def {
        id: 12,
        label: "F11",
        row: 0,
        weight: 2,
    },
    Def {
        id: 13,
        label: "F12",
        row: 0,
        weight: 2,
    },
    Def {
        id: 14,
        label: "`",
        row: 1,
        weight: 2,
    },
    Def {
        id: 15,
        label: "1",
        row: 1,
        weight: 2,
    },
    Def {
        id: 16,
        label: "2",
        row: 1,
        weight: 2,
    },
    Def {
        id: 17,
        label: "3",
        row: 1,
        weight: 2,
    },
    Def {
        id: 18,
        label: "4",
        row: 1,
        weight: 2,
    },
    Def {
        id: 19,
        label: "5",
        row: 1,
        weight: 2,
    },
    Def {
        id: 20,
        label: "6",
        row: 1,
        weight: 2,
    },
    Def {
        id: 21,
        label: "7",
        row: 1,
        weight: 2,
    },
    Def {
        id: 22,
        label: "8",
        row: 1,
        weight: 2,
    },
    Def {
        id: 23,
        label: "9",
        row: 1,
        weight: 2,
    },
    Def {
        id: 24,
        label: "0",
        row: 1,
        weight: 2,
    },
    Def {
        id: 25,
        label: "-",
        row: 1,
        weight: 2,
    },
    Def {
        id: 26,
        label: "=",
        row: 1,
        weight: 2,
    },
    Def {
        id: 27,
        label: "BACK",
        row: 1,
        weight: 3,
    },
    Def {
        id: 28,
        label: "TAB",
        row: 2,
        weight: 3,
    },
    Def {
        id: 29,
        label: "Q",
        row: 2,
        weight: 2,
    },
    Def {
        id: 30,
        label: "W",
        row: 2,
        weight: 2,
    },
    Def {
        id: 31,
        label: "E",
        row: 2,
        weight: 2,
    },
    Def {
        id: 32,
        label: "R",
        row: 2,
        weight: 2,
    },
    Def {
        id: 33,
        label: "T",
        row: 2,
        weight: 2,
    },
    Def {
        id: 34,
        label: "Y",
        row: 2,
        weight: 2,
    },
    Def {
        id: 35,
        label: "U",
        row: 2,
        weight: 2,
    },
    Def {
        id: 36,
        label: "I",
        row: 2,
        weight: 2,
    },
    Def {
        id: 37,
        label: "O",
        row: 2,
        weight: 2,
    },
    Def {
        id: 38,
        label: "P",
        row: 2,
        weight: 2,
    },
    Def {
        id: 39,
        label: "[",
        row: 2,
        weight: 2,
    },
    Def {
        id: 40,
        label: "]",
        row: 2,
        weight: 2,
    },
    Def {
        id: 41,
        label: "\\",
        row: 2,
        weight: 2,
    },
    Def {
        id: 42,
        label: "CAPS",
        row: 3,
        weight: 3,
    },
    Def {
        id: 43,
        label: "A",
        row: 3,
        weight: 2,
    },
    Def {
        id: 44,
        label: "S",
        row: 3,
        weight: 2,
    },
    Def {
        id: 45,
        label: "D",
        row: 3,
        weight: 2,
    },
    Def {
        id: 46,
        label: "F",
        row: 3,
        weight: 2,
    },
    Def {
        id: 47,
        label: "G",
        row: 3,
        weight: 2,
    },
    Def {
        id: 48,
        label: "H",
        row: 3,
        weight: 2,
    },
    Def {
        id: 49,
        label: "J",
        row: 3,
        weight: 2,
    },
    Def {
        id: 50,
        label: "K",
        row: 3,
        weight: 2,
    },
    Def {
        id: 51,
        label: "L",
        row: 3,
        weight: 2,
    },
    Def {
        id: 52,
        label: ";",
        row: 3,
        weight: 2,
    },
    Def {
        id: 53,
        label: "'",
        row: 3,
        weight: 2,
    },
    Def {
        id: 54,
        label: "ENTER",
        row: 3,
        weight: 3,
    },
    Def {
        id: 55,
        label: "SHIFT",
        row: 4,
        weight: 4,
    },
    Def {
        id: 56,
        label: "Z",
        row: 4,
        weight: 2,
    },
    Def {
        id: 57,
        label: "X",
        row: 4,
        weight: 2,
    },
    Def {
        id: 58,
        label: "C",
        row: 4,
        weight: 2,
    },
    Def {
        id: 59,
        label: "V",
        row: 4,
        weight: 2,
    },
    Def {
        id: 60,
        label: "B",
        row: 4,
        weight: 2,
    },
    Def {
        id: 61,
        label: "N",
        row: 4,
        weight: 2,
    },
    Def {
        id: 62,
        label: "M",
        row: 4,
        weight: 2,
    },
    Def {
        id: 63,
        label: ",",
        row: 4,
        weight: 2,
    },
    Def {
        id: 64,
        label: ".",
        row: 4,
        weight: 2,
    },
    Def {
        id: 65,
        label: "/",
        row: 4,
        weight: 2,
    },
    Def {
        id: 55,
        label: "SHIFT",
        row: 4,
        weight: 4,
    },
    Def {
        id: 67,
        label: "CTRL",
        row: 5,
        weight: 3,
    },
    Def {
        id: 68,
        label: "ALT",
        row: 5,
        weight: 3,
    },
    Def {
        id: 69,
        label: "SPACE",
        row: 5,
        weight: 10,
    },
    Def {
        id: 68,
        label: "ALT",
        row: 5,
        weight: 3,
    },
    Def {
        id: 67,
        label: "CTRL",
        row: 5,
        weight: 3,
    },
    Def {
        id: 72,
        label: "LEFT",
        row: 5,
        weight: 2,
    },
    Def {
        id: 73,
        label: "DOWN",
        row: 5,
        weight: 2,
    },
    Def {
        id: 74,
        label: "UP",
        row: 5,
        weight: 2,
    },
    Def {
        id: 75,
        label: "RIGHT",
        row: 5,
        weight: 2,
    },
];

const MODES: &[BoardMode] = &[
    BoardMode::Crush,
    BoardMode::Hold,
    BoardMode::Level,
    BoardMode::Pads,
    BoardMode::Midi,
    BoardMode::Build,
];

pub fn action_for(token: &str) -> Action {
    let token = token.trim().to_ascii_lowercase();
    if token.is_empty() {
        return Action::Play(5_000);
    }
    if token == "escape" || token == "esc" {
        return Action::Exit;
    }
    let alias = match token.as_str() {
        "backspace" | "back" => "back",
        "space" | " " => "space",
        "return" | "enter" => "enter",
        "capslock" | "caps" => "caps",
        "arrowleft" | "left" => "left",
        "arrowright" | "right" => "right",
        "arrowup" | "up" => "up",
        "arrowdown" | "down" => "down",
        "grave" | "backquote" => "`",
        "minus" => "-",
        "equal" => "=",
        "bracketleft" => "[",
        "bracketright" => "]",
        "backslash" => "\\",
        "semicolon" => ";",
        "quote" | "apostrophe" => "'",
        "comma" => ",",
        "period" => ".",
        "slash" => "/",
        "tab" => "tab",
        "shiftleft" | "shiftright" | "shift" => "shift",
        "controlleft" | "controlright" | "ctrl" | "control" => "ctrl",
        "altleft" | "altright" | "alt" => "alt",
        "super" | "meta" | "win" => "alt",
        other => other,
    };
    if let Some(id) = id_for_label(alias) {
        if id == EXIT_ID {
            return Action::Exit;
        }
        return Action::Play(id);
    }
    if let Some(rest) = token.strip_prefix("f") {
        if let Ok(n) = rest.parse::<u16>() {
            if (1..=12).contains(&n) {
                return Action::Play(1 + n);
            }
        }
    }
    if let Some(mode) = mode_from_name(&token) {
        return Action::Mode(mode);
    }
    Action::Play(5_000 + (hash_token(&token) % 400))
}

pub fn mode_from_id(id: u16) -> Option<BoardMode> {
    MODES.iter().copied().find(|mode| mode_id(*mode) == id)
}

fn mode_id(mode: BoardMode) -> u16 {
    MODE_BASE
        + match mode {
            BoardMode::Crush => 1,
            BoardMode::Hold => 2,
            BoardMode::Level => 3,
            BoardMode::Pads => 4,
            BoardMode::Midi => 5,
            BoardMode::Build => 6,
            BoardMode::Lesson => 0,
        }
}

fn mode_from_name(name: &str) -> Option<BoardMode> {
    Some(match name {
        "crush" => BoardMode::Crush,
        "hold" => BoardMode::Hold,
        "level" => BoardMode::Level,
        "pads" => BoardMode::Pads,
        "midi" => BoardMode::Midi,
        "build" => BoardMode::Build,
        _ => return None,
    })
}

pub fn id_for_label(label: &str) -> Option<u16> {
    let label = label.trim();
    KEYS.iter()
        .find(|key| key.label.eq_ignore_ascii_case(label))
        .map(|key| key.id)
}

pub fn label_for(id: u16) -> &'static str {
    if let Some(label) = choice_label(id) {
        return label;
    }
    if let Some(mode) = mode_from_id(id) {
        return mode.name();
    }
    KEYS.iter()
        .find(|key| key.id == id)
        .map(|key| key.label)
        .unwrap_or("KEY")
}

pub fn layout(width: i32, height: i32, age: u8) -> Vec<KeySpot> {
    layout_for(width, height, age, BoardMode::Lesson)
}

pub fn layout_for(width: i32, height: i32, age: u8, mode: BoardMode) -> Vec<KeySpot> {
    let width = width.max(320);
    let height = height.max(240);
    let mut spots = Vec::new();
    spots.push(KeySpot {
        id: LESSON_ID,
        label: "LESSON",
        x: width - 150,
        y: 4,
        w: 142,
        h: 32,
    });
    let mut y = 40;
    if age >= FULL_CLASS_AGE {
        let gap = 8;
        let w = (width - gap * (MODES.len() as i32 + 1)) / MODES.len() as i32;
        let h = 52;
        for (index, class) in MODES.iter().copied().enumerate() {
            spots.push(KeySpot {
                id: mode_id(class),
                label: class.name(),
                x: gap + index as i32 * (w + gap),
                y,
                w,
                h,
            });
        }
        y += h + 8;
    }
    y = add_choice_row(&mut spots, width, y, choices(age, mode));
    let rows = 6i32;
    let gap = 6;
    let board_h = (height - y - 8).max(rows * 24);
    let row_h = (board_h - gap * (rows - 1)) / rows;
    for row in 0..rows {
        let items: Vec<&Def> = KEYS.iter().filter(|key| key.row == row as u8).collect();
        let weight: i32 = items.iter().map(|key| i32::from(key.weight)).sum();
        let inner = width - gap * (items.len() as i32 + 1);
        let mut x = gap;
        for key in items {
            let w = (inner * i32::from(key.weight) / weight.max(1)).max(8);
            spots.push(KeySpot {
                id: key.id,
                label: key.label,
                x,
                y,
                w,
                h: row_h.max(8),
            });
            x += w + gap;
        }
        y += row_h + gap;
    }
    spots
}

pub fn spot_at(spots: &[KeySpot], x: i32, y: i32) -> Option<u16> {
    spots
        .iter()
        .find(|spot| x >= spot.x && y >= spot.y && x < spot.x + spot.w && y < spot.y + spot.h)
        .map(|spot| spot.id)
}

const PENTA: [u32; 15] = [
    196, 220, 247, 262, 294, 330, 392, 440, 494, 523, 587, 659, 784, 880, 988,
];

pub fn sound_for(age: u8, id: u16, mode: BoardMode) -> Sound {
    if age < READING_AGE {
        return silly_sound(age, id);
    }
    let mut sound = Sound::quiet();
    sound.playing = true;
    let index = usize::from(id);
    let mut freq = PENTA[index % PENTA.len()];
    if age <= 5 {
        freq = PENTA[index % 8];
    }
    sound.freq_hz = freq + (id as u32 % 9);
    let cap = level_cap(age);
    sound.ceiling_db_milli = cap.ceiling_db_milli;
    sound.gain_milli = cap.gain_milli * (55 + (id as i32 % 40)) / 100;
    let mode = if age < FULL_CLASS_AGE {
        BoardMode::Lesson
    } else {
        mode
    };
    match mode {
        BoardMode::Lesson => {
            sound.bits = if age <= 5 {
                12
            } else if age <= 10 {
                (4 + (id % 8) as u8).clamp(4, 12)
            } else {
                (2 + (id % 12) as u8).clamp(2, 14)
            };
            sound.rate_hz = if age <= 7 {
                48_000
            } else if id % 2 == 0 {
                48_000
            } else {
                6_000
            };
            sound.mix_milli = 1000;
        }
        BoardMode::Crush => {
            sound.bits = (1 + (id % 16) as u8).clamp(1, 16);
            sound.rate_hz = 48_000;
        }
        BoardMode::Hold => {
            sound.bits = 16;
            sound.rate_hz = [2_000, 4_000, 8_000, 12_000, 48_000][index % 5];
        }
        BoardMode::Level => {
            sound.bits = 10;
            sound.mix_milli = [0, 250, 500, 750, 1000][index % 5];
            sound.gain_milli = cap.gain_milli * (40 + (id as i32 % 50)) / 100;
        }
        BoardMode::Pads => {
            sound.bits = 8;
            sound.mix_milli = if id % 3 == 0 { 0 } else { 1000 };
            sound.rate_hz = if id % 4 == 0 { 4_000 } else { 48_000 };
        }
        BoardMode::Midi => {
            sound.bits = if id % 2 == 0 { 4 } else { 12 };
            sound.mix_milli = if id % 5 == 0 { 0 } else { 1000 };
        }
        BoardMode::Build => {
            sound.bits = 8;
            sound.rate_hz = 8_000;
        }
    }
    sound.capped(age)
}

const SMOOTH: u16 = 910;
const MID: u16 = 911;
const BLOCK: u16 = 912;
const B16: u16 = 913;
const B8: u16 = 914;
const B4: u16 = 915;
const B2: u16 = 916;
const FAST: u16 = 917;
const SLOW: u16 = 918;
const SHORT: u16 = 919;
const MED: u16 = 920;
const LONG: u16 = 921;
const DRY: u16 = 922;
const HALF: u16 = 923;
const WET: u16 = 924;
const LITTLE: u16 = 925;
const LOT: u16 = 926;
const LID_OFF: u16 = 927;
const LID_ON: u16 = 928;
const OPEN: u16 = 929;
const CLOSE: u16 = 930;
const SEND: u16 = 931;
const BITS6: u16 = 932;
const B12: u16 = 933;
const B1: u16 = 934;
const QUIET: u16 = 935;
const UNITY: u16 = 936;
const GATE: u16 = 937;
const UNGATE: u16 = 938;
const HOLD12: u16 = 939;
const HOLD3: u16 = 940;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Choice {
    id: u16,
    label: &'static str,
}

/// The controls an older lesson is holding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Knob {
    pub bits: u8,
    pub rate_hz: u32,
    pub mix_milli: i32,
    pub lid_on: bool,
    pub gate_open: bool,
    pub send_milli: i32,
    pub quiet_gain: bool,
}

impl Knob {
    fn middle() -> Self {
        Self {
            bits: 8,
            rate_hz: 48_000,
            mix_milli: 1_000,
            lid_on: false,
            gate_open: true,
            send_milli: 0,
            quiet_gain: false,
        }
    }
}

pub fn knob_for(age: u8, mode: BoardMode) -> Knob {
    let mut knob = Knob::middle();
    if age >= FULL_CLASS_AGE {
        match mode {
            BoardMode::Crush => knob.bits = 8,
            BoardMode::Hold => knob.rate_hz = 12_000,
            BoardMode::Level => knob.mix_milli = 1_000,
            BoardMode::Midi => knob.bits = 4,
            _ => {}
        }
        return knob;
    }
    match age {
        6 => knob.bits = 6,
        7 => knob.bits = 16,
        9 => knob.rate_hz = 8_000,
        10 => {
            knob.mix_milli = 500;
            knob.bits = 4;
        }
        11 => {
            knob.mix_milli = 250;
            knob.bits = 8;
        }
        _ => {}
    }
    knob
}

pub fn silly_name(id: u16) -> &'static str {
    const NAMES: &[&str] = &[
        "HONK", "CHIRP", "ROBOT", "BUBBLE", "LASER", "DRUM", "WOBBLE", "BEEP", "GROWL", "ZAP",
        "BOING", "SQUEAK", "QUACK", "BUZZ", "POP", "MEOW",
    ];
    NAMES[usize::from(id) % NAMES.len()]
}

pub fn played_sound(age: u8, id: u16, mode: BoardMode, knob: &Knob) -> Sound {
    let mut sound = sound_for(age, id, mode);
    if age < READING_AGE {
        return sound;
    }
    tune(&mut sound, age, mode, knob);
    sound.capped(age)
}

pub fn press_choice(age: u8, mode: BoardMode, mut knob: Knob, id: u16) -> Option<Knob> {
    if !choices(age, mode).iter().any(|choice| choice.id == id) {
        return None;
    }
    match id {
        SMOOTH => knob.bits = 12,
        MID => knob.bits = 6,
        BLOCK => knob.bits = 3,
        B16 => knob.bits = 16,
        B12 => knob.bits = 12,
        B8 => knob.bits = 8,
        B4 => knob.bits = 4,
        B2 => knob.bits = 2,
        B1 => knob.bits = 1,
        FAST | SHORT => knob.rate_hz = 48_000,
        SLOW => knob.rate_hz = 2_500,
        MED => knob.rate_hz = 8_000,
        HOLD12 => knob.rate_hz = 12_000,
        LONG => knob.rate_hz = 2_000,
        HOLD3 => knob.rate_hz = 3_000,
        DRY => knob.mix_milli = 0,
        UNGATE => knob.gate_open = false,
        HALF => knob.mix_milli = 500,
        WET => knob.mix_milli = 1_000,
        LITTLE => {
            knob.mix_milli = 250;
            knob.bits = 8;
        }
        LOT => {
            knob.mix_milli = 1_000;
            knob.bits = 4;
        }
        LID_OFF => knob.lid_on = false,
        LID_ON => knob.lid_on = true,
        OPEN | GATE => knob.gate_open = true,
        CLOSE => knob.gate_open = false,
        SEND => knob.send_milli = (knob.send_milli + 250).min(1_000),
        BITS6 => knob.bits = 6,
        QUIET => knob.quiet_gain = true,
        UNITY => knob.quiet_gain = false,
        _ => return None,
    }
    Some(knob)
}

pub fn digit_choice(age: u8, mode: BoardMode, key_id: u16) -> Option<u16> {
    let index = match label_for(key_id) {
        "1" => 0,
        "2" => 1,
        "3" => 2,
        "4" => 3,
        "5" => 4,
        "6" => 5,
        _ => return None,
    };
    choices(age, mode).get(index).map(|choice| choice.id)
}

pub fn selected_choices(age: u8, mode: BoardMode, knob: &Knob) -> Vec<u16> {
    choices(age, mode)
        .iter()
        .filter(|choice| choice_lit(choice.id, knob))
        .map(|choice| choice.id)
        .collect()
}

pub fn book_spots(
    width: i32,
    height: i32,
    page: usize,
    pages: usize,
    show_back: bool,
    chooser: bool,
) -> Vec<KeySpot> {
    let width = width.max(320);
    let height = height.max(240);
    let mut spots = Vec::new();
    spots.push(KeySpot {
        id: EXIT_ID,
        label: "ESC",
        x: 8,
        y: 8,
        w: 120,
        h: 56,
    });
    let bar_h = 72;
    let y = height - bar_h - 12;
    if show_back {
        spots.push(KeySpot {
            id: BACK_ID,
            label: "BACK",
            x: 8,
            y,
            w: 160,
            h: bar_h,
        });
    }
    if chooser {
        let gap = 8;
        let x0 = if show_back { 176 } else { 8 };
        let count = MODES.len() as i32;
        let inner = (width - x0 - gap * (count + 1)).max(count * 8);
        let w = (inner / count).max(8);
        for (index, class) in MODES.iter().copied().enumerate() {
            spots.push(KeySpot {
                id: mode_id(class),
                label: class.name(),
                x: x0 + gap + index as i32 * (w + gap),
                y,
                w,
                h: bar_h,
            });
        }
    } else {
        let label = if pages == 0 || page + 1 >= pages {
            "PLAY"
        } else {
            "NEXT"
        };
        spots.push(KeySpot {
            id: NEXT_ID,
            label,
            x: width - 220,
            y,
            w: 208,
            h: bar_h,
        });
    }
    spots
}

pub fn paint_book(
    pixels: &mut [u32],
    width: i32,
    height: i32,
    card: &Card,
    spots: &[KeySpot],
    page: usize,
    pages: usize,
    age: u8,
) {
    if width <= 0 || height <= 0 {
        return;
    }
    let stride = width as usize;
    for pixel in pixels.iter_mut().take(stride * height as usize) {
        *pixel = 0;
    }
    let count = format!("{} / {}", page + 1, pages.max(1));
    draw_text(
        pixels,
        stride,
        width - 16 - text_width(&count, 2),
        18,
        2,
        &count,
        false,
    );
    draw_text(pixels, stride, 140, 18, 2, "volume down first", false);
    let longest = card
        .lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(1)
        .max(card.title.chars().count());
    let title_want = if age < READING_AGE { 12 } else { 8 };
    let title_scale = fit_scale(width, card.title.chars().count().max(1), title_want);
    let body_want = if age < READING_AGE { 7 } else { 5 };
    let body_scale = fit_scale(width, longest, body_want).min(title_scale);
    let mut y = 84;
    draw_centered(pixels, stride, width, y, title_scale, card.title);
    y += 8 * title_scale + 18;
    for line in card.lines {
        draw_centered(pixels, stride, width, y, body_scale, line);
        y += 8 * body_scale + 10;
    }
    for spot in spots {
        stroke_rect(
            pixels, stride, width, height, spot.x, spot.y, spot.w, spot.h, false,
        );
        let scale = (spot.h / 12).clamp(2, 5);
        let text_w = text_width(spot.label, scale);
        let text_h = 7 * scale;
        let tx = spot.x + (spot.w - text_w).max(0) / 2;
        let ty = spot.y + (spot.h - text_h).max(0) / 2;
        draw_text(pixels, stride, tx, ty, scale, spot.label, false);
    }
}

fn silly_sound(age: u8, id: u16) -> Sound {
    let hash = u16::from(age)
        .wrapping_mul(131)
        .wrapping_add(id.wrapping_mul(17))
        .rotate_left(3)
        .wrapping_add(id);
    let bits_set = [2u8, 3, 4, 5, 6, 8, 3, 5];
    let rates = [350u32, 700, 1_100, 1_800, 2_600, 4_000, 7_000, 12_000];
    let lows = [55u32, 65, 82, 98, 110, 130, 146, 164];
    let highs = [523u32, 659, 784, 880, 1046, 1318, 1568, 1760, 2093];
    let wild = [
        73u32, 110, 164, 220, 330, 392, 494, 587, 698, 880, 1174, 1480, 1760, 2093, 2637,
    ];
    let freq_base = if age == 4 {
        if id % 2 == 0 {
            lows[usize::from(hash) % lows.len()]
        } else {
            highs[usize::from(hash) % highs.len()]
        }
    } else {
        wild[usize::from(hash) % wild.len()]
    };
    let bits = if age <= 1 {
        bits_set[usize::from(hash) % 6]
    } else {
        bits_set[usize::from(hash) % bits_set.len()]
    };
    let rate = rates[(usize::from(hash) / 5) % rates.len()];
    let cap = level_cap(age);
    let gain = if age == 2 {
        if id % 2 == 0 {
            (cap.gain_milli * 28 / 100).max(1)
        } else {
            cap.gain_milli
        }
    } else {
        cap.gain_milli * (32 + i32::from(hash % 69)) / 100
    };
    Sound {
        playing: true,
        bits,
        rate_hz: rate,
        gain_milli: gain,
        mix_milli: 1_000,
        ceiling_db_milli: cap.ceiling_db_milli,
        freq_hz: freq_base + u32::from(id % 23),
    }
    .capped(age)
}

fn tune(sound: &mut Sound, age: u8, mode: BoardMode, knob: &Knob) {
    if age >= FULL_CLASS_AGE {
        match mode {
            BoardMode::Crush => {
                sound.bits = knob.bits.clamp(1, 16);
                sound.rate_hz = 48_000;
                sound.mix_milli = 1_000;
            }
            BoardMode::Hold => {
                sound.bits = 16;
                sound.rate_hz = knob.rate_hz.max(1);
                sound.mix_milli = 1_000;
            }
            BoardMode::Level => {
                let cap = level_cap(age);
                sound.bits = 12;
                sound.rate_hz = 48_000;
                sound.mix_milli = knob.mix_milli.clamp(0, 1_000);
                sound.gain_milli = if knob.quiet_gain {
                    cap.gain_milli * 30 / 100
                } else {
                    cap.gain_milli
                };
                if knob.lid_on {
                    sound.ceiling_db_milli = -18_000;
                }
            }
            BoardMode::Pads => {
                sound.playing = knob.gate_open && sound.playing;
                sound.bits = 8;
                sound.mix_milli = 1_000;
                sound.rate_hz = rate_from_send(knob.send_milli);
            }
            BoardMode::Midi => {
                sound.bits = knob.bits.clamp(1, 16);
                sound.rate_hz = 48_000;
                sound.mix_milli = if knob.gate_open { 1_000 } else { 0 };
            }
            BoardMode::Build => {
                sound.bits = 8;
                sound.rate_hz = 8_000;
                sound.mix_milli = 1_000;
            }
            BoardMode::Lesson => {}
        }
        return;
    }
    match age {
        5 => {
            sound.bits = 12;
            sound.rate_hz = 48_000;
            sound.mix_milli = 1_000;
        }
        6 | 7 => {
            sound.bits = knob.bits.clamp(1, 16);
            sound.rate_hz = 48_000;
            sound.mix_milli = 1_000;
        }
        8 | 9 => {
            sound.bits = 14;
            sound.rate_hz = knob.rate_hz.max(1);
            sound.mix_milli = 1_000;
        }
        10 => {
            sound.bits = 4;
            sound.rate_hz = 48_000;
            sound.mix_milli = knob.mix_milli.clamp(0, 1_000);
        }
        11 => {
            sound.bits = knob.bits.clamp(1, 16);
            sound.rate_hz = 48_000;
            sound.mix_milli = knob.mix_milli.clamp(0, 1_000);
        }
        12 => {
            sound.bits = 8;
            sound.rate_hz = 48_000;
            sound.mix_milli = 1_000;
            if knob.lid_on {
                sound.ceiling_db_milli = -18_000;
            }
        }
        _ => {
            sound.playing = knob.gate_open && sound.playing;
            sound.bits = knob.bits.clamp(1, 16);
            sound.mix_milli = 1_000;
            sound.rate_hz = rate_from_send(knob.send_milli);
        }
    }
}

fn rate_from_send(send_milli: i32) -> u32 {
    if send_milli <= 0 {
        48_000
    } else {
        (12_000 - (send_milli as u32 / 250) * 2_000).max(2_000)
    }
}

fn choices(age: u8, mode: BoardMode) -> &'static [Choice] {
    if age >= FULL_CLASS_AGE {
        return match mode {
            BoardMode::Crush => CRUSH_ROW,
            BoardMode::Hold => HOLD_ROW,
            BoardMode::Level => LEVEL_ROW,
            BoardMode::Pads => PAD_ROW,
            BoardMode::Midi => MIDI_ROW,
            BoardMode::Build | BoardMode::Lesson => &[],
        };
    }
    match age {
        6 => SIX_ROW,
        7 => SEVEN_ROW,
        8 => EIGHT_ROW,
        9 => NINE_ROW,
        10 => TEN_ROW,
        11 => ELEVEN_ROW,
        12 => TWELVE_ROW,
        13 => THIRTEEN_ROW,
        14 => FOURTEEN_ROW,
        _ => &[],
    }
}

fn choice_label(id: u16) -> Option<&'static str> {
    match id {
        LESSON_ID => Some("LESSON"),
        BACK_ID => Some("BACK"),
        NEXT_ID => Some("NEXT"),
        SMOOTH => Some("SMOOTH"),
        MID => Some("MID"),
        BLOCK => Some("BLOCK"),
        B16 => Some("16"),
        B12 => Some("12"),
        B8 => Some("8"),
        B4 => Some("4"),
        B2 => Some("2"),
        B1 => Some("1"),
        FAST => Some("FAST"),
        SLOW => Some("SLOW"),
        SHORT => Some("SHORT"),
        MED => Some("MED"),
        LONG => Some("LONG"),
        DRY => Some("DRY"),
        HALF => Some("HALF"),
        WET => Some("WET"),
        LITTLE => Some("LITTLE"),
        LOT => Some("LOT"),
        LID_OFF => Some("LID OFF"),
        LID_ON => Some("LID ON"),
        OPEN => Some("OPEN"),
        CLOSE => Some("CLOSE"),
        SEND => Some("SEND"),
        BITS6 => Some("BITS 6"),
        QUIET => Some("QUIET"),
        UNITY => Some("FULL"),
        GATE => Some("GATE"),
        UNGATE => Some("DRY"),
        HOLD12 => Some("12000"),
        HOLD3 => Some("3000"),
        _ => None,
    }
}

fn choice_lit(id: u16, knob: &Knob) -> bool {
    match id {
        SMOOTH | B12 => knob.bits == 12,
        MID => knob.bits == 6,
        BLOCK => knob.bits == 3,
        B16 => knob.bits == 16,
        B8 => knob.bits == 8,
        B4 => knob.bits == 4,
        B2 => knob.bits == 2,
        B1 => knob.bits == 1,
        BITS6 => knob.bits == 6,
        FAST | SHORT => knob.rate_hz >= 48_000,
        SLOW => knob.rate_hz == 2_500,
        MED => knob.rate_hz == 8_000,
        HOLD12 => knob.rate_hz == 12_000,
        LONG => knob.rate_hz == 2_000,
        HOLD3 => knob.rate_hz == 3_000,
        DRY => knob.mix_milli == 0,
        UNGATE => !knob.gate_open,
        HALF => knob.mix_milli == 500,
        WET => knob.mix_milli == 1_000,
        LITTLE => knob.mix_milli == 250 && knob.bits == 8,
        LOT => knob.mix_milli == 1_000 && knob.bits == 4,
        LID_OFF => !knob.lid_on,
        LID_ON => knob.lid_on,
        OPEN | GATE => knob.gate_open,
        CLOSE => !knob.gate_open,
        SEND => knob.send_milli > 0,
        QUIET => knob.quiet_gain,
        UNITY => !knob.quiet_gain,
        _ => false,
    }
}

fn add_choice_row(spots: &mut Vec<KeySpot>, width: i32, y: i32, row: &[Choice]) -> i32 {
    if row.is_empty() {
        return y;
    }
    let gap = 8;
    let count = row.len() as i32;
    let w = ((width - gap * (count + 1)) / count).max(8);
    let h = 48;
    for (index, choice) in row.iter().enumerate() {
        spots.push(KeySpot {
            id: choice.id,
            label: choice.label,
            x: gap + index as i32 * (w + gap),
            y,
            w,
            h,
        });
    }
    y + h + 8
}

fn fit_scale(width: i32, chars: usize, want: i32) -> i32 {
    let fit = (width - 48) / ((chars as i32).max(1) * 6);
    fit.max(1).min(want.max(1))
}

fn draw_centered(pixels: &mut [u32], stride: usize, width: i32, y: i32, scale: i32, text: &str) {
    let x = (width - text_width(text, scale)).max(0) / 2;
    draw_text(pixels, stride, x, y, scale, text, false);
}

const SIX_ROW: &[Choice] = &[
    Choice {
        id: SMOOTH,
        label: "SMOOTH",
    },
    Choice {
        id: MID,
        label: "MID",
    },
    Choice {
        id: BLOCK,
        label: "BLOCK",
    },
];

const SEVEN_ROW: &[Choice] = &[
    Choice {
        id: B16,
        label: "16",
    },
    Choice { id: B8, label: "8" },
    Choice { id: B4, label: "4" },
    Choice { id: B2, label: "2" },
];

const EIGHT_ROW: &[Choice] = &[
    Choice {
        id: FAST,
        label: "FAST",
    },
    Choice {
        id: SLOW,
        label: "SLOW",
    },
];

const NINE_ROW: &[Choice] = &[
    Choice {
        id: SHORT,
        label: "SHORT",
    },
    Choice {
        id: MED,
        label: "MED",
    },
    Choice {
        id: LONG,
        label: "LONG",
    },
];

const TEN_ROW: &[Choice] = &[
    Choice {
        id: DRY,
        label: "DRY",
    },
    Choice {
        id: HALF,
        label: "HALF",
    },
    Choice {
        id: WET,
        label: "WET",
    },
];

const ELEVEN_ROW: &[Choice] = &[
    Choice {
        id: LITTLE,
        label: "LITTLE",
    },
    Choice {
        id: LOT,
        label: "LOT",
    },
];

const TWELVE_ROW: &[Choice] = &[
    Choice {
        id: LID_OFF,
        label: "LID OFF",
    },
    Choice {
        id: LID_ON,
        label: "LID ON",
    },
];

const THIRTEEN_ROW: &[Choice] = &[
    Choice {
        id: OPEN,
        label: "OPEN",
    },
    Choice {
        id: CLOSE,
        label: "CLOSE",
    },
    Choice {
        id: SEND,
        label: "SEND",
    },
];

const FOURTEEN_ROW: &[Choice] = &[
    Choice {
        id: OPEN,
        label: "OPEN",
    },
    Choice {
        id: CLOSE,
        label: "CLOSE",
    },
    Choice {
        id: SEND,
        label: "SEND",
    },
    Choice {
        id: BITS6,
        label: "BITS 6",
    },
];

const CRUSH_ROW: &[Choice] = &[
    Choice {
        id: B16,
        label: "16",
    },
    Choice { id: B8, label: "8" },
    Choice { id: B4, label: "4" },
    Choice { id: B2, label: "2" },
];

const HOLD_ROW: &[Choice] = &[
    Choice {
        id: SHORT,
        label: "48000",
    },
    Choice {
        id: HOLD12,
        label: "12000",
    },
    Choice {
        id: HOLD3,
        label: "3000",
    },
];

const LEVEL_ROW: &[Choice] = &[
    Choice {
        id: QUIET,
        label: "QUIET",
    },
    Choice {
        id: UNITY,
        label: "FULL",
    },
    Choice {
        id: DRY,
        label: "DRY",
    },
    Choice {
        id: WET,
        label: "WET",
    },
    Choice {
        id: LID_OFF,
        label: "OFF",
    },
    Choice {
        id: LID_ON,
        label: "LID",
    },
];

const PAD_ROW: &[Choice] = THIRTEEN_ROW;

const MIDI_ROW: &[Choice] = &[
    Choice { id: B4, label: "4" },
    Choice {
        id: B12,
        label: "12",
    },
    Choice {
        id: GATE,
        label: "GATE",
    },
    Choice {
        id: UNGATE,
        label: "DRY",
    },
];

pub fn paint(
    pixels: &mut [u32],
    width: i32,
    height: i32,
    spots: &[KeySpot],
    held: &[u16],
    age: u8,
    last: &str,
    mode: BoardMode,
) {
    if width <= 0 || height <= 0 {
        return;
    }
    let stride = width as usize;
    for pixel in pixels.iter_mut().take(stride * height as usize) {
        *pixel = 0;
    }
    let header = format!(
        "KOS TRUST LESSON   age {age}   {last}   ESC leaves   {mode}",
        mode = mode.name()
    );
    draw_text(pixels, stride, 12, 8, 2, &header, false);
    for spot in spots {
        let on = held.contains(&spot.id);
        stroke_rect(
            pixels, stride, width, height, spot.x, spot.y, spot.w, spot.h, on,
        );
        let scale = (spot.h / 12).clamp(2, 6);
        let text_w = text_width(spot.label, scale);
        let text_h = 7 * scale;
        let tx = spot.x + (spot.w - text_w).max(0) / 2;
        let ty = spot.y + (spot.h - text_h).max(0) / 2;
        draw_text(pixels, stride, tx, ty, scale, spot.label, on);
    }
}

fn stroke_rect(
    pixels: &mut [u32],
    stride: usize,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    fill: bool,
) {
    for row in 0..h {
        for col in 0..w {
            let border = row < 2 || col < 2 || row >= h - 2 || col >= w - 2;
            if fill || border {
                put(pixels, stride, width, height, x + col, y + row, WHITE);
            }
        }
    }
}

const WHITE: u32 = 0x00FF_FFFF;

fn put(pixels: &mut [u32], stride: usize, width: i32, height: i32, x: i32, y: i32, color: u32) {
    if x < 0 || y < 0 || x >= width || y >= height {
        return;
    }
    let index = y as usize * stride + x as usize;
    if index < pixels.len() {
        pixels[index] = color;
    }
}

fn text_width(text: &str, scale: i32) -> i32 {
    text.chars().count() as i32 * (6 * scale)
}

fn draw_text(
    pixels: &mut [u32],
    stride: usize,
    x: i32,
    y: i32,
    scale: i32,
    text: &str,
    invert: bool,
) {
    let mut cursor = x;
    for ch in text.chars() {
        blit(pixels, stride, cursor, y, scale, ch, invert);
        cursor += 6 * scale;
    }
}

fn blit(pixels: &mut [u32], stride: usize, x: i32, y: i32, scale: i32, ch: char, invert: bool) {
    let glyph = glyph(ch);
    let width = stride as i32;
    let height = if stride == 0 {
        0
    } else {
        pixels.len() as i32 / width
    };
    for row in 0..7 {
        for col in 0..5 {
            let on = glyph[row] & (1 << (4 - col)) != 0;
            if on == invert {
                continue;
            }
            for sy in 0..scale {
                for sx in 0..scale {
                    put(
                        pixels,
                        stride,
                        width,
                        height,
                        x + col as i32 * scale + sx,
                        y + row as i32 * scale + sy,
                        WHITE,
                    );
                }
            }
        }
    }
}

fn glyph(ch: char) -> [u8; 7] {
    match ch.to_ascii_uppercase() {
        'A' => [0x0E, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'B' => [0x1E, 0x11, 0x11, 0x1E, 0x11, 0x11, 0x1E],
        'C' => [0x0E, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0E],
        'D' => [0x1E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1E],
        'E' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x1F],
        'F' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x10],
        'G' => [0x0E, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0E],
        'H' => [0x11, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'I' => [0x0E, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0E],
        'J' => [0x01, 0x01, 0x01, 0x01, 0x11, 0x11, 0x0E],
        'K' => [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11],
        'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1F],
        'M' => [0x11, 0x1B, 0x15, 0x11, 0x11, 0x11, 0x11],
        'N' => [0x11, 0x19, 0x15, 0x13, 0x11, 0x11, 0x11],
        'O' => [0x0E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'P' => [0x1E, 0x11, 0x11, 0x1E, 0x10, 0x10, 0x10],
        'Q' => [0x0E, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0D],
        'R' => [0x1E, 0x11, 0x11, 0x1E, 0x14, 0x12, 0x11],
        'S' => [0x0E, 0x11, 0x10, 0x0E, 0x01, 0x11, 0x0E],
        'T' => [0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
        'U' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'V' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x0A, 0x04],
        'W' => [0x11, 0x11, 0x11, 0x15, 0x15, 0x1B, 0x11],
        'X' => [0x11, 0x11, 0x0A, 0x04, 0x0A, 0x11, 0x11],
        'Y' => [0x11, 0x11, 0x0A, 0x04, 0x04, 0x04, 0x04],
        'Z' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1F],
        '0' => [0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E],
        '1' => [0x04, 0x0C, 0x04, 0x04, 0x04, 0x04, 0x0E],
        '2' => [0x0E, 0x11, 0x01, 0x06, 0x08, 0x10, 0x1F],
        '3' => [0x0E, 0x11, 0x01, 0x06, 0x01, 0x11, 0x0E],
        '4' => [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02],
        '5' => [0x1F, 0x10, 0x1E, 0x01, 0x01, 0x11, 0x0E],
        '6' => [0x06, 0x08, 0x10, 0x1E, 0x11, 0x11, 0x0E],
        '7' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
        '8' => [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E],
        '9' => [0x0E, 0x11, 0x11, 0x0F, 0x01, 0x02, 0x0C],
        '-' => [0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00],
        '=' => [0x00, 0x00, 0x1F, 0x00, 0x1F, 0x00, 0x00],
        ',' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x08],
        '.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x0C],
        '/' => [0x01, 0x02, 0x04, 0x08, 0x10, 0x00, 0x00],
        '\\' => [0x10, 0x08, 0x04, 0x02, 0x01, 0x00, 0x00],
        ';' => [0x00, 0x0C, 0x0C, 0x00, 0x0C, 0x04, 0x08],
        '\'' => [0x04, 0x04, 0x08, 0x00, 0x00, 0x00, 0x00],
        '[' => [0x0E, 0x08, 0x08, 0x08, 0x08, 0x08, 0x0E],
        ']' => [0x0E, 0x02, 0x02, 0x02, 0x02, 0x02, 0x0E],
        '`' => [0x08, 0x04, 0x02, 0x00, 0x00, 0x00, 0x00],
        ' ' => [0, 0, 0, 0, 0, 0, 0],
        _ => [0x1F, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1F],
    }
}

fn hash_token(token: &str) -> u16 {
    let mut hash = 0u16;
    for byte in token.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(u16::from(byte));
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_leaves_and_every_other_key_plays() {
        assert_eq!(action_for("escape"), Action::Exit);
        assert_eq!(action_for("ESC"), Action::Exit);
        assert!(matches!(action_for("q"), Action::Play(_)));
        assert!(matches!(action_for("space"), Action::Play(_)));
        assert!(matches!(action_for("f12"), Action::Play(_)));
        assert!(matches!(action_for("shift"), Action::Play(_)));
        assert!(matches!(action_for("arrowleft"), Action::Play(_)));
        assert!(matches!(action_for("some-extra-key"), Action::Play(_)));
    }

    #[test]
    fn letters_digits_and_space_are_on_the_board() {
        let spots = layout(1280, 800, 3);
        for label in ["A", "Z", "0", "9", "SPACE", "ENTER", "ESC", "F1"] {
            let id = id_for_label(label).unwrap();
            assert!(spots.iter().any(|spot| spot.id == id), "{label}");
        }
    }

    #[test]
    fn a_tap_hits_the_key_under_it() {
        let spots = layout(1280, 800, 3);
        let key = spots.iter().find(|spot| spot.label == "A").unwrap();
        assert_eq!(
            spot_at(&spots, key.x + key.w / 2, key.y + key.h / 2),
            Some(key.id)
        );
        assert_eq!(spot_at(&spots, 0, 0), None);
    }

    #[test]
    fn neighboring_keys_differ_and_stay_under_the_cap() {
        let a = sound_for(3, id_for_label("A").unwrap(), BoardMode::Lesson);
        let s = sound_for(3, id_for_label("S").unwrap(), BoardMode::Lesson);
        assert_ne!(a.freq_hz, s.freq_hz);
        let cap = level_cap(3);
        assert!(a.gain_milli <= cap.gain_milli);
        assert!(s.gain_milli <= cap.gain_milli);
        assert!(a.ceiling_db_milli <= cap.ceiling_db_milli);
        let hot = sound_for(3, 50_000, BoardMode::Crush);
        assert!(hot.gain_milli <= cap.gain_milli);
        assert!(hot.ceiling_db_milli <= cap.ceiling_db_milli);
    }

    #[test]
    fn age_15_has_mode_buttons_and_a_held_key_is_white() {
        let spots = layout(1280, 800, 15);
        assert!(spots.iter().any(|spot| spot.label == "crush"));
        let mut pixels = vec![0u32; 1280 * 800];
        let id = id_for_label("A").unwrap();
        paint(
            &mut pixels,
            1280,
            800,
            &spots,
            &[id],
            15,
            "A",
            BoardMode::Crush,
        );
        assert!(pixels.iter().any(|pixel| *pixel == WHITE));
        assert!(pixels.iter().any(|pixel| *pixel == 0));
    }

    #[test]
    fn young_keys_keep_their_own_silly_sounds() {
        let mut seen = Vec::new();
        for id in [2u16, 16, 20, 32, 43, 44, 45, 46, 47, 69] {
            let sound = sound_for(3, id, BoardMode::Lesson);
            let cap = level_cap(3);
            assert!(sound.gain_milli <= cap.gain_milli);
            assert!(sound.ceiling_db_milli <= cap.ceiling_db_milli);
            seen.push((sound.freq_hz, sound.bits, sound.rate_hz));
        }
        seen.sort();
        seen.dedup();
        assert!(seen.len() >= 8, "{seen:?}");
        let soft = sound_for(2, 44, BoardMode::Lesson);
        let loud = sound_for(2, 43, BoardMode::Lesson);
        assert!(soft.gain_milli < loud.gain_milli);
    }

    #[test]
    fn older_controls_change_the_sound_and_stay_capped() {
        let loose = knob_for(7, BoardMode::Lesson);
        let tight = press_choice(
            7,
            BoardMode::Lesson,
            loose,
            digit_choice(7, BoardMode::Lesson, id_for_label("4").unwrap()).unwrap(),
        )
        .unwrap();
        let wide = played_sound(7, 43, BoardMode::Lesson, &loose);
        let square = played_sound(7, 43, BoardMode::Lesson, &tight);
        assert_eq!(wide.bits, 16);
        assert_eq!(square.bits, 2);
        assert!(wide.gain_milli <= level_cap(7).gain_milli);
        let open = knob_for(13, BoardMode::Lesson);
        let mut shut = open;
        shut.gate_open = false;
        assert!(!played_sound(13, 43, BoardMode::Lesson, &shut).playing);
        assert!(played_sound(13, 43, BoardMode::Lesson, &open).playing);
        let mut lid = knob_for(12, BoardMode::Lesson);
        let off = played_sound(12, 43, BoardMode::Lesson, &lid);
        lid.lid_on = true;
        let on = played_sound(12, 43, BoardMode::Lesson, &lid);
        assert!(on.ceiling_db_milli < off.ceiling_db_milli);
    }

    #[test]
    fn lesson_pages_have_a_next_and_a_play() {
        let spots = book_spots(1280, 800, 0, 4, false, false);
        let next = spots.iter().find(|spot| spot.label == "NEXT").unwrap();
        assert_eq!(spot_at(&spots, next.x + 4, next.y + 4), Some(NEXT_ID));
        let last = book_spots(1280, 800, 3, 4, true, false);
        assert!(last.iter().any(|spot| spot.label == "PLAY"));
        let card = crate::book::lesson_cards(6, None)[0];
        let mut pixels = vec![0u32; 1280 * 800];
        paint_book(&mut pixels, 1280, 800, &card, &spots, 0, 4, 6);
        assert!(pixels.iter().any(|pixel| *pixel == WHITE));
        assert!(layout(1280, 800, 6)
            .iter()
            .any(|spot| spot.label == "SMOOTH"));
    }
}
