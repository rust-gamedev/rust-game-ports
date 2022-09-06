use crate::{actor::Actor, position::Position, resources::Resources};
use macroquad::{
    prelude::{collections::storage, draw_texture, WHITE},
    rand::ChooseRandom,
};

#[derive(Clone)]
pub struct Train {
    dx: i32,
    position: Position,
    image_index: usize,
}

impl Actor for Train {
    fn update(&mut self) {
        self.position.x += self.dx;
    }

    fn draw(&self, offset_x: i32, offset_y: i32) {
        let image = *storage::get::<Resources>()
            .train_textures
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
        860
    }
}

impl Train {
    pub fn new(dx: i32, position: Position) -> Self {
        let image_index = if dx < 0 {
            *vec![0, 2, 4].choose().unwrap()
        } else {
            *vec![1, 3, 5].choose().unwrap()
        };
        Self {
            dx,
            position,
            image_index,
        }
    }
}
