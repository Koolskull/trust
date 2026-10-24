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

use crate::{level_cap, Sound, FULL_CLASS_AGE};

/// On-screen and physical Escape. Tapping this key leaves the lesson.
pub const EXIT_ID: u16 = 1;

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
    if let Some(mode) = mode_from_id(id) {
        return mode.name();
    }
    KEYS.iter()
        .find(|key| key.id == id)
        .map(|key| key.label)
        .unwrap_or("KEY")
}

pub fn layout(width: i32, height: i32, age: u8) -> Vec<KeySpot> {
    let width = width.max(320);
    let height = height.max(240);
    let mut spots = Vec::new();
    let mut y = 40;
    if age >= FULL_CLASS_AGE {
        let gap = 8;
        let w = (width - gap * (MODES.len() as i32 + 1)) / MODES.len() as i32;
        let h = 52;
        for (index, mode) in MODES.iter().copied().enumerate() {
            spots.push(KeySpot {
                id: mode_id(mode),
                label: mode.name(),
                x: gap + index as i32 * (w + gap),
                y,
                w,
                h,
            });
        }
        y += h + 8;
    }
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
}
