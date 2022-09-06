use crate::resources::Resources;
use macroquad::prelude::{collections::storage, draw_texture, WHITE};

pub enum NumberAlign {
    Left,
    Right,
}

pub enum NumberColor {
    Blue,
    Yellow,
}

pub fn display_number(number: u32, color: NumberColor, x: i32, align: NumberAlign) {
    let resources = storage::get::<Resources>();
    let n = number.to_string();
    for (i, digit) in n.chars().enumerate() {
        let digit_index: usize = match color {
            NumberColor::Yellow => digit.to_digit(10).unwrap() as usize + 10,
            NumberColor::Blue => digit.to_digit(10).unwrap() as usize,
        };
        let x_pos = match align {
            NumberAlign::Right => x + ((i as i32 - n.len() as i32) * 25),
            NumberAlign::Left => x + (i as i32 * 25),
        };
        draw_texture(
            resources.digit_textures[digit_index],
            x_pos as f32,
            0.,
            WHITE,
        );
    }
}
