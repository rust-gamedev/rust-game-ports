use ggez::audio::{self, SoundSource};
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{self, Image};
use ggez::{timer, Context, GameResult};

use crate::ball::Ball;
use crate::bat::Bat;
use crate::impact::Impact;
use crate::state::State;

/// Global game state.
/// The name is a bit confusing (due to the State enum), however, this is the ggez naming.
/// This holds also the concepts that in the original code, are stored in global variables.
pub struct GameState {
    pub bats: [Bat; 2],
    pub ball: Ball,
    /// List of the current impacts to display.
    pub impacts: Vec<Impact>,
    /// Offset added to the AI player's target Y position, so it won't aim to hit the ball exactly in
    /// the centre of the bat.
    pub ai_offset: i8,

    menu_images: Vec<Image>,
    game_over_image: Image,

    music: audio::Source,

    state: State,

    num_players: usize,
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

        let menu_images = (0..2)
            .map(|i| {
                let menu_image_filename = format!("/menu{}.png", i);
                Image::new(context, menu_image_filename).unwrap()
            })
            .collect();

        let game_over_image = Image::new(context, "/over.png").unwrap();

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

            menu_images,
            game_over_image,

            music,

            state: State::Menu,

            num_players: 1,
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

    fn draw(&mut self, context: &mut Context) -> GameResult {
        match self.state {
            State::Menu => {
                graphics::draw(
                    context,
                    &self.menu_images[self.num_players - 1],
                    graphics::DrawParam::new(),
                )?;
            }
            State::GameOver => {
                graphics::draw(context, &self.game_over_image, graphics::DrawParam::new())?;
            }
            State::Play => {}
        }

        graphics::present(context)?;

        timer::yield_now();

        Ok(())
    }
}
