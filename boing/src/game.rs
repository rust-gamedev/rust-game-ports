use ggez::{Context, GameResult};

use crate::ball::Ball;
use crate::bat::Bat;
use crate::impact::Impact;

pub struct Game {
    pub bats: [Bat; 2],
    pub ball: Ball,
    /// List of the current impacts to display.
    pub impacts: Vec<Impact>,
    /// Offset added to the AI player's target Y position, so it won't aim to hit the ball exactly in
    /// the centre of the bat.
    pub ai_offset: f32,
}

impl Game {
    pub fn new(
        context: &mut Context,
        controls: (
            Option<fn(&Context, &Ball, f32, &Bat) -> f32>,
            Option<fn(&Context, &Ball, f32, &Bat) -> f32>,
        ),
    ) -> Self {
        Self {
            bats: [
                Bat::new(context, 0, controls.0),
                Bat::new(context, 1, controls.1),
            ],
            ball: Ball::new(context, -1.),
            impacts: vec![],
            ai_offset: 0.,
        }
    }

    pub fn update(&mut self, context: &mut Context) -> GameResult {
        // Update all active objects
        for bat in &mut self.bats {
            bat.update(context, &self.ball, self.ai_offset)?
        }
        self.ball.update(
            context,
            &mut self.bats,
            &mut self.impacts,
            &mut self.ai_offset,
        )?;
        for impact in &mut self.impacts {
            impact.update(context)?
        }

        // Remove any expired impact effects from the list.
        // Interesting, this is easier in Rust :)
        self.impacts.retain(|impact| impact.time < 10);

        Ok(())
    }

    pub fn draw(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }
}
