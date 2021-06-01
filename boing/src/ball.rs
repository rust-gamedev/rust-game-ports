use ggez::{Context, GameResult};

use crate::{HALF_HEIGHT, HALF_WIDTH};

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
}

impl Ball {
    pub fn new(dx: f32) -> Self {
        Self {
            x: HALF_WIDTH,
            y: HALF_HEIGHT,
            dx,
        }
    }

    pub fn update(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }

    pub fn draw(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }
}
