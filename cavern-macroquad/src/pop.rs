use macroquad::prelude::{collections::storage, Texture2D};

use crate::{
    actor::{Actor, Anchor},
    resources::Resources,
};

pub struct Pop {
    pub timer: i32,
    pub type_: i32,

    // Actor trait
    pub x: i32,
    pub y: i32,
    pub image: Texture2D,
    pub anchor: Anchor,
}

impl Pop {
    pub fn new(x: i32, y: i32, type_: i32) -> Self {
        Self {
            type_: type_,
            timer: -1,
            x,
            y,
            image: storage::get::<Resources>().blank_texture,
            anchor: Anchor::Centre,
        }
    }

    pub fn update(&mut self) {
        self.timer += 1;

        let type_factor = self.type_ * 7;
        let timer_factor = self.timer / 2;
        let image_i = (type_factor + timer_factor) as usize;
        self.image = storage::get::<Resources>().pop_textures[image_i];
    }
}

impl Actor for Pop {
    fn x(&self) -> i32 {
        self.x
    }

    fn x_mut(&mut self) -> &mut i32 {
        &mut self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn y_mut(&mut self) -> &mut i32 {
        &mut self.y
    }

    fn image(&self) -> macroquad::prelude::Texture2D {
        self.image
    }

    fn anchor(&self) -> crate::actor::Anchor {
        self.anchor
    }
}
