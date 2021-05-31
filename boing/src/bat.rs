use ggez::{event::KeyCode, Context, GameResult};

use crate::actor::Actor;

pub struct Bat {
    /// Player number
    pub player: i8,
    /// Control logic; None uses AI.
    pub move_func: Option<Box<dyn Fn(KeyCode) -> i8>>,
}

impl Actor for Bat {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }
}
