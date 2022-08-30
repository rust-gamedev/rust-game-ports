use crate::{actor::Actor, mover::Mover, position::Position, resources::Resources};
use macroquad::{
    prelude::{collections::storage, draw_texture, WHITE},
    rand::gen_range,
};

#[derive(Clone)]
pub struct Log {
    dx: i32,
    position: Position,
    image_index: usize,
}

impl Mover for Log {
    fn dx(&self) -> i32 {
        self.dx
    }
}

impl Actor for Log {
    fn update(&mut self) {
        self.position.x += self.dx;
    }

    fn draw(&self, offset_x: i32, offset_y: i32) {
        let image = *storage::get::<Resources>()
            .log_textures
            .get(self.image_index)
            .unwrap();
        draw_texture(
            image,
            (self.position.x + offset_x) as f32 - image.width() / 2.,
            (self.position.y + offset_y) as f32 - image.height(),
            WHITE,
        );
    }

    fn x(&self) -> i32 {
        self.position.x
    }

    fn y(&self) -> i32 {
        self.position.y
    }

    fn width(&self) -> i32 {
        if self.image_index == 0 {
            84
        } else {
            138
        }
    }
}

impl Log {
    pub fn new(dx: i32, position: Position) -> Self {
        let image_index = gen_range::<usize>(0, 2);
        Self {
            dx,
            position,
            image_index,
        }
    }
}
