use crate::{actor::Actor, position::Position, resources::Resources};
use macroquad::prelude::{collections::storage, draw_texture, WHITE};

pub struct Eagle {
    position: Position,
}

impl Actor for Eagle {
    fn update(&mut self) {
        self.position.y += 12;
    }

    fn draw(&self, offset_x: i32, offset_y: i32) {
        let shadow_image = storage::get::<Resources>().eagles_texture;
        let x = (self.position.x + offset_x) as f32 - shadow_image.width() / 2.;
        let y = (self.position.y + offset_y) as f32 - shadow_image.height();
        draw_texture(shadow_image, x, y, WHITE);

        let eagle_image = storage::get::<Resources>().eagle_texture;
        draw_texture(eagle_image, x, y + 32., WHITE);
    }

    fn x(&self) -> i32 {
        self.position.x
    }

    fn y(&self) -> i32 {
        self.position.y
    }

    fn width(&self) -> i32 {
        166
    }
}

impl Eagle {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}
