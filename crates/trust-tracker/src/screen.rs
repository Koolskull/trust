//                    ☦
//   Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

use crate::phrase::{note_name, Phrase, CHANNELS, ROWS};
use crate::VERSION;

pub const WHITE: u32 = 0x00FF_FFFF;
pub const BLACK: u32 = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spot {
    pub channel: usize,
    pub row: usize,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Clone, Debug)]
pub struct Layout {
    pub esc_x: i32,
    pub esc_y: i32,
    pub esc_w: i32,
    pub esc_h: i32,
    pub cells: Vec<Spot>,
}

pub fn layout(width: i32, height: i32) -> Layout {
    let width = width.max(320);
    let height = height.max(240);
    let mut cells = Vec::with_capacity(ROWS * CHANNELS);
    let top = 72;
    let gap = 4;
    let grid_w = width - 16;
    let grid_h = (height - top - 28).max(ROWS as i32 * 12);
    let cell_w = ((grid_w - gap * (CHANNELS as i32 + 1)) / CHANNELS as i32).max(8);
    let cell_h = ((grid_h - gap * (ROWS as i32 + 1)) / ROWS as i32).max(8);
    for row in 0..ROWS {
        for channel in 0..CHANNELS {
            cells.push(Spot {
                channel,
                row,
                x: 8 + gap + channel as i32 * (cell_w + gap),
                y: top + gap + row as i32 * (cell_h + gap),
                w: cell_w,
                h: cell_h,
            });
        }
    }
    Layout {
        esc_x: 8,
        esc_y: 8,
        esc_w: 96,
        esc_h: 40,
        cells,
    }
}

pub fn cell_at(layout: &Layout, x: i32, y: i32) -> Option<(usize, usize)> {
    layout
        .cells
        .iter()
        .find(|spot| x >= spot.x && y >= spot.y && x < spot.x + spot.w && y < spot.y + spot.h)
        .map(|spot| (spot.channel, spot.row))
}

pub fn esc_at(layout: &Layout, x: i32, y: i32) -> bool {
    x >= layout.esc_x
        && y >= layout.esc_y
        && x < layout.esc_x + layout.esc_w
        && y < layout.esc_y + layout.esc_h
}

pub fn paint(
    pixels: &mut [u32],
    width: i32,
    height: i32,
    phrase: &Phrase,
    play_row: Option<usize>,
) {
    if width <= 0 || height <= 0 {
        return;
    }
    let stride = width as usize;
    for pixel in pixels.iter_mut().take(stride * height as usize) {
        *pixel = BLACK;
    }
    let board = layout(width, height);
    stroke(
        pixels,
        stride,
        width,
        height,
        board.esc_x,
        board.esc_y,
        board.esc_w,
        board.esc_h,
        WHITE,
    );
    draw_text(
        pixels,
        stride,
        width,
        height,
        board.esc_x + 22,
        board.esc_y + 8,
        2,
        "ESC",
        false,
    );
    let transport = if phrase.playing() { "PLAY" } else { "STOP" };
    let header = format!("THRASH {VERSION}  {transport}  OCT {}", phrase.octave());
    draw_text(pixels, stride, width, height, 120, 16, 2, &header, false);
    draw_text(
        pixels,
        stride,
        width,
        height,
        12,
        height - 22,
        2,
        "ARROWS MOVE   Z WRITES   SPACE PLAY   PEN SELECTS",
        false,
    );
    for spot in &board.cells {
        let play = play_row == Some(spot.row);
        let cursor = phrase.channel() == spot.channel && phrase.row() == spot.row;
        if play {
            fill(
                pixels, stride, width, height, spot.x, spot.y, spot.w, spot.h, WHITE,
            );
        } else {
            stroke(
                pixels, stride, width, height, spot.x, spot.y, spot.w, spot.h, WHITE,
            );
        }
        if cursor {
            let ink = if play { BLACK } else { WHITE };
            stroke(
                pixels,
                stride,
                width,
                height,
                spot.x + 2,
                spot.y + 2,
                spot.w - 4,
                spot.h - 4,
                ink,
            );
        }
        let label = note_name(phrase.note(spot.channel, spot.row));
        let scale = ((spot.h - 8) / 8).clamp(1, 4);
        let text_w = label.chars().count() as i32 * 6 * scale;
        let tx = spot.x + (spot.w - text_w).max(0) / 2;
        let ty = spot.y + (spot.h - 7 * scale).max(0) / 2;
        draw_text(pixels, stride, width, height, tx, ty, scale, &label, play);
    }
}

fn fill(
    pixels: &mut [u32],
    stride: usize,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: u32,
) {
    for row in 0..h {
        for col in 0..w {
            put(pixels, stride, width, height, x + col, y + row, color);
        }
    }
}

fn stroke(
    pixels: &mut [u32],
    stride: usize,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: u32,
) {
    if w <= 2 || h <= 2 {
        return;
    }
    for col in 0..w {
        put(pixels, stride, width, height, x + col, y, color);
        put(pixels, stride, width, height, x + col, y + h - 1, color);
    }
    for row in 0..h {
        put(pixels, stride, width, height, x, y + row, color);
        put(pixels, stride, width, height, x + w - 1, y + row, color);
    }
}

fn draw_text(
    pixels: &mut [u32],
    stride: usize,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    scale: i32,
    text: &str,
    invert: bool,
) {
    let mut cursor = x;
    let ink = if invert { BLACK } else { WHITE };
    for ch in text.chars() {
        blit(pixels, stride, width, height, cursor, y, scale, ch, ink);
        cursor += 6 * scale;
    }
}

fn blit(
    pixels: &mut [u32],
    stride: usize,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    scale: i32,
    ch: char,
    ink: u32,
) {
    let glyph = glyph(ch);
    for row in 0..7 {
        for col in 0..5 {
            if glyph[row] & (1 << (4 - col)) == 0 {
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
                        ink,
                    );
                }
            }
        }
    }
}

fn put(pixels: &mut [u32], stride: usize, width: i32, height: i32, x: i32, y: i32, color: u32) {
    if x < 0 || y < 0 || x >= width || y >= height {
        return;
    }
    let index = y as usize * stride + x as usize;
    if index < pixels.len() {
        pixels[index] = color;
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
        '#' => [0x0A, 0x0A, 0x1F, 0x0A, 0x1F, 0x0A, 0x0A],
        ' ' => [0, 0, 0, 0, 0, 0, 0],
        _ => [0x1F, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1F],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phrase::{Edit, Phrase};

    #[test]
    fn a_tap_selects_the_cell_and_the_grid_paints() {
        let board = layout(1280, 800);
        let spot = board
            .cells
            .iter()
            .find(|spot| spot.channel == 3 && spot.row == 4)
            .unwrap();
        assert_eq!(cell_at(&board, spot.x + 2, spot.y + 2), Some((3, 4)));
        assert!(esc_at(&board, board.esc_x + 4, board.esc_y + 4));
        let mut phrase = Phrase::new();
        phrase.edit(Edit::Note(60));
        let mut pixels = vec![0u32; 1280 * 800];
        paint(&mut pixels, 1280, 800, &phrase, Some(0));
        assert!(pixels.iter().any(|pixel| *pixel == WHITE));
        assert!(pixels.iter().any(|pixel| *pixel == BLACK));
    }
}
