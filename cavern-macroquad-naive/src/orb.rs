use crate::{actor::Actor, robot::RobotType};

pub struct Orb {
    pub timer: i32,
    pub floating: bool,
    pub trapped_enemy_type: Option<RobotType>,

    // Actor trait
    pub x: i32,
    pub y: i32,
}

impl Orb {
    pub fn update(&mut self) {
        eprintln!("WRITEME: Orb#update");
    }

    pub fn draw(&self) {
        eprintln!("WRITEME: Orb#draw");
    }
}

impl Actor for Orb {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn image(&self) -> macroquad::prelude::Texture2D {
        todo!()
    }

    fn anchor(&self) -> crate::actor::Anchor {
        todo!()
    }
}
