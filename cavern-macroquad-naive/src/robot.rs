use macroquad::{
    prelude::{collections::storage, Texture2D},
    rand::gen_range,
};

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
    pub speed: i32,
    pub direction_x: i32,
    pub alive: bool,
    pub change_dir_timer: i32,
    pub fire_timer: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, type_: RobotType) -> Self {
        Self {
            x,
            y,
            image: storage::get::<Resources>().blank_texture,
            anchor: Anchor::CentreBottom,
            type_,
            speed: gen_range(1, 4),
            direction_x: 1,
            alive: true,
            change_dir_timer: 0,
            fire_timer: 100,
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
