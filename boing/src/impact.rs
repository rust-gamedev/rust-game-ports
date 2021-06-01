use ggez::{Context, GameResult};

use crate::actor::Actor;

/// Type for an animation which is displayed briefly whenever the ball bounces
pub struct Impact {
    pub time: u8,
}

impl Actor for Impact {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }
}
