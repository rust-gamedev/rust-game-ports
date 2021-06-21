use macroquad::prelude::{collections::storage, Texture2D};

use crate::{
    actor::{Actor, Anchor},
    collide_actor::CollideActor,
    gravity_actor::{GravityActor, GRAVITY_ACTOR_DEFAULT_ANCHOR},
    resources::Resources,
};

pub struct Fruit {
    pub time_to_live: i32,

    // Actor trait
    pub x: i32,
    pub y: i32,
    pub image: Texture2D,
    pub anchor: Anchor,

    // GravityActor trait
    pub vel_y: i32,
    pub landed: bool,
}

impl Fruit {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            time_to_live: 500, // Counts down to zero

            x,
            y,
            image: storage::get::<Resources>().blank_texture,
            anchor: GRAVITY_ACTOR_DEFAULT_ANCHOR,

            vel_y: 0,
            landed: false,
        }
    }

    pub fn update(&mut self) {
        eprintln!("WRITEME: Fruit#update");
    }
}

impl Actor for Fruit {
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

impl CollideActor for Fruit {}

impl GravityActor for Fruit {
    fn vel_y(&self) -> i32 {
        self.vel_y
    }

    fn vel_y_mut(&mut self) -> &mut i32 {
        &mut self.vel_y
    }

    fn landed(&self) -> bool {
        self.landed
    }

    fn landed_mut(&mut self) -> &mut bool {
        &mut self.landed
    }
}
