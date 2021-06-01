use ggez::audio::{self, SoundSource};
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{self, Image};
use ggez::input::keyboard::is_key_pressed;
use ggez::{timer, Context, GameResult};

use crate::game::Game;
use crate::state::State;

/// Global state, not to be confused with the game state (which is a part of it).
pub struct GlobalState {
    state: State,
    game: Game,
    num_players: usize,
    space_down: bool,

    menu_images: Vec<Image>,
    game_over_image: Image,

    music: audio::Source,

    down_sound: audio::Source,
    up_sound: audio::Source,
}

impl GlobalState {
    pub fn new(context: &mut Context) -> Self {
        let menu_images = (0..2)
            .map(|i| {
                let menu_image_filename = format!("/menu{}.png", i);
                Image::new(context, menu_image_filename).unwrap()
            })
            .collect();

        let game_over_image = Image::new(context, "/over.png").unwrap();

        // For simplicity, we always assume that it's possible to play the music.
        let music = audio::Source::new(context, "/theme.ogg").unwrap();

        // In the [docs](https://pygame-zero.readthedocs.io/en/stable/builtins.html), it's not explicit
        // what happens if there is an error, so we just implement the intuitive logic.
        let down_sound = audio::Source::new(context, "/down.ogg").unwrap();
        let up_sound = audio::Source::new(context, "/up.ogg").unwrap();

        Self {
            state: State::Menu,
            game: Game::new((None, None)),
            num_players: 1,
            space_down: false,
            menu_images,
            game_over_image,
            music,
            down_sound,
            up_sound,
        }
    }

    pub fn play_music(&mut self, context: &mut Context) -> GameResult {
        self.music.set_volume(0.3);
        self.music.play(context)
    }
}

impl EventHandler for GlobalState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        match self.state {
            State::Menu => {
                if self.num_players == 2 && is_key_pressed(context, KeyCode::Up) {
                    self.up_sound.play(context)?;
                    self.num_players = 1;
                } else if self.num_players == 1 && is_key_pressed(context, KeyCode::Down) {
                    self.down_sound.play(context)?;
                    self.num_players = 2;
                }
            }
            State::GameOver => {
                //
            }
            State::Play => {
                //
            }
        }

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
