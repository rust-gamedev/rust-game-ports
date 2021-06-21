use macroquad::prelude::{collections::storage, Texture2D};

use crate::{
    actor::{Actor, Anchor},
    resources::Resources,
    robot::RobotType,
};

pub struct Orb {
    pub direction_x: i32,
    pub timer: i32,
    pub floating: bool,
    /// Number of frames during which we will be pushed horizontally
    pub blown_frames: i32,
    /// Type of enemy trapped in this bubble
    pub trapped_enemy_type: Option<RobotType>,

    // Actor trait
    pub x: i32,
    pub y: i32,
    pub image: Texture2D,
    pub anchor: Anchor,
}

impl Orb {
    pub fn new(x: i32, y: i32, direction_x: i32) -> Self {
        Self {
            direction_x, // Orbs are initially blown horizontally, then start floating upwards
            timer: -1,
            floating: false,
            blown_frames: 6,
            trapped_enemy_type: None,
            x,
            y,
            image: storage::get::<Resources>().blank_texture,
            anchor: Anchor::Centre,
        }
    }

    pub fn update(&mut self) {
        eprintln!("WRITEME: Orb#update");
    }
}

impl Actor for Orb {
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
