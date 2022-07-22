use macroquad::prelude::{collections::storage, draw_texture, WHITE};

use phf::phf_map;

use crate::{resources::Resources, WIDTH};

// Widths of the letters A to Z in the font images
pub const CHAR_WIDTH: [i32; 26] = [
    27, 26, 25, 26, 25, 25, 26, 25, 12, 26, 26, 25, 33, 25, 26, 25, 27, 26, 26, 25, 26, 26, 38, 25,
    25, 25,
];

pub const IMAGE_WIDTH: phf::Map<&'static str, i32> = phf_map! {
    "life" => 44,
    "plus" => 40,
    "health" => 40,
};

fn char_width(chr: &u8) -> i32 {
    // Return width of given character. For characters other than the letters A to Z (i.e. space, and the digits 0 to 9),
    // the width of the letter A is returned. ord gives the ASCII/Unicode code for the given character.
    let index = if *chr < 65 { 0 } else { chr - 65 } as usize;
    CHAR_WIDTH[index]
}

// Differs from the original function name, due to clashing with the Macroquad API.
pub fn draw_game_text(text: &str, y: i32, x: Option<i32>) {
    let text = text.as_bytes();

    let mut x = x.unwrap_or_else(|| {
        // If no X pos specified, draw text in centre of the screen - must first work out total width of text
        (WIDTH - text.iter().map(char_width).sum::<i32>()) / 2
    });

    let fonts = &storage::get::<Resources>().fonts;

    for chr in text {
        let font = fonts[chr];
        draw_texture(font, x as f32, y as f32, WHITE);
        x += char_width(chr);
    }
}
