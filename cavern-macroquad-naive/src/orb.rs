use macroquad::prelude::Texture2D;

use crate::{
    actor::{Actor, Anchor},
    robot::RobotType,
};

pub struct Orb {
    pub timer: i32,
    pub floating: bool,
    pub trapped_enemy_type: Option<RobotType>,

    // Actor trait
    pub x: i32,
    pub y: i32,
    pub image: Texture2D,
    pub anchor: Anchor,
}

impl Orb {
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
