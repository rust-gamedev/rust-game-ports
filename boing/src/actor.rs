use ggez::{Context, GameResult};

pub trait Actor {
    fn update(&mut self, context: &mut Context) -> GameResult;
    fn draw(&mut self, context: &mut Context) -> GameResult;
}
