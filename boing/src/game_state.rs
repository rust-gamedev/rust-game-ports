use ggez::audio::{self, SoundSource};
use ggez::event::{EventHandler, KeyCode};
use ggez::{Context, GameResult};

use crate::ball::Ball;
use crate::bat::Bat;
use crate::impact::Impact;

pub struct GameState {
    pub bats: [Bat; 2],
    pub ball: Ball,
    /// List of the current impacts to display.
    pub impacts: Vec<Impact>,
    /// Offset added to the AI player's target Y position, so it won't aim to hit the ball exactly in
    /// the centre of the bat.
    pub ai_offset: i8,

    music: audio::Source,
}

impl GameState {
    pub fn new(
        context: &mut Context,
        controls: (
            Option<Box<dyn Fn(KeyCode) -> i8>>,
            Option<Box<dyn Fn(KeyCode) -> i8>>,
        ),
    ) -> Self {
        // For simplicity, we always assume that it's possible to play the music.
        let music = audio::Source::new(context, "/theme.ogg").unwrap();

        Self {
            bats: [
                Bat {
                    player: 0,
                    move_func: controls.0,
                },
                Bat {
                    player: 1,
                    move_func: controls.1,
                },
            ],
            ball: Ball { dx: -1. },
            impacts: vec![],
            ai_offset: 0,
            music,
        }
    }

    pub fn play_music(&mut self, context: &mut Context) -> GameResult {
        self.music.set_volume(0.3);
        self.music.play(context)
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        println!("TODO: GameState");
        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        println!("TODO: GameState");
        Ok(())
    }
}
