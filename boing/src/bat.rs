use ggez::{Context, GameResult};

use crate::{ball::Ball, controls::ai, HALF_HEIGHT};

pub struct Bat {
    pub x: f32,
    pub y: f32,
    /// Player number
    pub player: i8,
    // 2^16 points out to be enough for anybody.
    pub score: u16,
    /// move_func is a function we may or may not have been passed by the code which created this
    /// object. If this bat is meant to be player controlled, move_func will be a function that when
    /// called, returns a number indicating the direction and speed in which the bat should move,
    /// based on the keys the player is currently pressing.
    /// If move_func is None, this indicates that this bat should instead be controlled by the AI method.
    pub move_func: fn(&Context, &Ball, f32, &Bat) -> f32,

    /// Each bat has a timer which starts at zero and counts down by one every frame. When a player
    /// concedes a point, their timer is set to 20, which causes the bat to display a different animation
    /// frame. It is also used to decide when to create a new ball in the centre of the screen - see
    /// comments in Game.update for more on this. Finally, it is used in Game.draw to determine when
    /// to display a visual effect over the top of the background.
    pub timer: i32,
}

impl Bat {
    pub fn new(player: i8, move_func: Option<fn(&Context, &Ball, f32, &Bat) -> f32>) -> Self {
        let x = if player == 0 { 40. } else { 760. };

        let move_func = move_func.unwrap_or(ai);

        Self {
            x: x,
            y: HALF_HEIGHT,
            player,
            score: 0,
            move_func,
            timer: 0,
        }
    }

    pub fn update(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }

    pub fn draw(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }
}
