use ggez::{Context, GameResult};

use crate::actor::Actor;

pub struct Ball {
    pub dx: f32,
}

impl Actor for Ball {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }
}
