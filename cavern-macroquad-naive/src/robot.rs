use macroquad::prelude::{collections::storage, Texture2D};

use crate::{
    actor::{Actor, Anchor},
    resources::Resources,
};

#[derive(Clone, Copy)]
pub enum RobotType {
    Aggressive,
    Normal,
}

#[allow(dead_code)]
pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub image: Texture2D,
    pub anchor: Anchor,
    pub type_: RobotType,
    pub alive: bool,
}

impl Robot {
    pub fn new(x: i32, y: i32, type_: RobotType) -> Self {
        Self {
            x,
            y,
            image: storage::get::<Resources>().blank_texture,
            anchor: Anchor::CentreBottom,
            type_,
            alive: true,
        }
    }

    pub fn update(&mut self) {
        eprintln!("WRITEME: Robot#update");
    }
}

impl Actor for Robot {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn image(&self) -> macroquad::prelude::Texture2D {
        self.image
    }

    fn anchor(&self) -> Anchor {
        self.anchor
    }
}
