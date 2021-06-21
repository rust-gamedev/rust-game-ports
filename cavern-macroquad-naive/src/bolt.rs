use macroquad::prelude::{collections::storage, Texture2D};

use crate::{
    actor::{Actor, Anchor},
    collide_actor::{CollideActor, COLLIDE_ACTOR_DEFAULT_ANCHOR},
    resources::Resources,
};

pub struct Bolt {
    pub direction_x: i32,
    pub active: bool,

    // Actor trait
    pub x: i32,
    pub y: i32,
    pub image: Texture2D,
    pub anchor: Anchor,
}

impl Bolt {
    pub fn new(x: i32, y: i32, direction_x: i32) -> Self {
        Self {
            direction_x,
            active: true,

            x,
            y,
            image: storage::get::<Resources>().blank_texture,
            anchor: COLLIDE_ACTOR_DEFAULT_ANCHOR,
        }
    }

    pub fn update(&mut self) {
        eprintln!("WRITEME: Bolt#update");
    }
}

impl Actor for Bolt {
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

impl CollideActor for Bolt {}
