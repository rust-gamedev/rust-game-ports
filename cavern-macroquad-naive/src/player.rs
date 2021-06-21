use macroquad::prelude::{collections::storage, Texture2D};

use crate::{
    actor::{Actor, Anchor},
    collide_actor::CollideActor,
    gravity_actor::{GravityActor, GRAVITY_ACTOR_DEFAULT_ANCHOR},
    orb::Orb,
    resources::Resources,
    WIDTH,
};

pub struct Player {
    pub lives: i32,
    pub score: i32,
    pub direction_x: i32, // -1 = left, 1 = right
    pub fire_timer: i32,
    pub hurt_timer: i32,
    pub health: i32,
    pub blowing_orb: Option<Orb>,

    // Actor trait
    pub x: i32,
    pub y: i32,
    pub image: Texture2D,
    pub anchor: Anchor,

    // GravityActor trait
    pub vel_y: i32,
    pub landed: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            lives: 2,
            score: 0,
            direction_x: 0,
            fire_timer: 0,
            hurt_timer: 0,
            health: 0,
            blowing_orb: None,

            x: 0,
            y: 0,
            image: storage::get::<Resources>().blank_texture,
            anchor: GRAVITY_ACTOR_DEFAULT_ANCHOR,

            vel_y: 0,
            landed: false,
        }
    }

    pub fn reset(&mut self) {
        self.x = WIDTH / 2;
        self.y = 100;
        self.vel_y = 0;
        self.direction_x = 1; // -1 = left, 1 = right
        self.fire_timer = 0;
        self.hurt_timer = 100; // Invulnerable for this many frames
        self.health = 3;
        self.blowing_orb = None;
    }

    pub fn update(&mut self) {
        eprintln!("WRITEME: Player#update");
    }
}

impl Actor for Player {
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

    fn image(&self) -> Texture2D {
        self.image
    }

    fn anchor(&self) -> Anchor {
        self.anchor
    }
}

impl CollideActor for Player {}

impl GravityActor for Player {
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
