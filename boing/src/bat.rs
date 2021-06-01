use ggez::{event::KeyCode, Context, GameResult};

use crate::{actor::Actor, HALF_HEIGHT};

pub struct Bat {
    pub x: f32,
    pub y: f32,
    /// Player number
    pub player: i8,
    // 2^16 points out to be enough for anybody.
    pub score: u16,
    /// Control logic; None uses AI.
    pub move_func: Option<Box<dyn Fn(KeyCode) -> i8>>,
}

impl Bat {
    pub fn new(player: i8, move_func: Option<Box<dyn Fn(KeyCode) -> i8>>) -> Self {
        let x = if player == 0 { 40. } else { 760. };

        Self {
            x: x,
            y: HALF_HEIGHT,
            player,
            score: 0,
            move_func,
        }
    }
}

impl Actor for Bat {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }
}
