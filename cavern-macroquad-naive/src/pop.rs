use macroquad::prelude::Texture2D;

use crate::actor::{Actor, Anchor};

pub struct Pop {
    pub timer: i32,

    // Actor trait
    pub x: i32,
    pub y: i32,
    pub image: Texture2D,
    pub anchor: Anchor,
}

impl Pop {
    pub fn update(&mut self) {
        eprintln!("WRITEME: Pop#update");
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
