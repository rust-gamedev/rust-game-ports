use macroquad::prelude::{collections::storage, draw_texture, is_key_down, KeyCode, WHITE};

use crate::{game::Game, player::Player, resources::Resources, state::State};

pub struct GlobalState {
    state: State,
    game: Game,
    space_down: bool,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            // Set the initial game state
            state: State::Menu,
            game: Game::new(None),
            space_down: false,
        }
    }

    pub fn update(&mut self) {
        match self.state {
            State::Menu => {
                if self.space_pressed() {
                    // Switch to play state, and create a new Game object, passing it a new Player object to use
                    self.state = State::Play;
                    self.game = Game::new(Some(Player::new()));
                } else {
                    self.game.update();
                }
            }
            State::Play => {
                if self.game.player.as_ref().unwrap().lives < 0 {
                    self.game.play_sound("over");
                    self.state = State::GameOver;
                } else {
                    self.game.update();
                }
            }
            State::GameOver => {
                if self.space_pressed() {
                    self.state = State::Menu;
                    self.game = Game::new(None);
                }
            }
        }
    }

    pub fn draw(&self) {
        let resources = storage::get::<Resources>();

        self.game.draw();

        match self.state {
            State::Menu => {
                // Draw title screen
                draw_texture(resources.title_texture, 0., 0., WHITE);

                // Draw "Press SPACE" animation, which has 10 frames numbered 0 to 9
                // The first part gives us a number between 0 and 159, based on the game timer
                // Dividing by 4 means we go to a new animation frame every 4 frames
                // We enclose this calculation in the min function, with the other argument being 9, which results in the
                // animation staying on frame 9 for three quarters of the time. Adding 40 to the game timer is done to alter
                // which stage the animation is at when the game first starts
                let anim_frame = (((self.game.timer + 40) % 160) / 4).min(9) as usize;
                draw_texture(resources.space_textures[anim_frame], 130., 280., WHITE);
            }
            State::Play => {
                self.draw_status();
            }
            State::GameOver => {
                self.draw_status();
                // Display "Game Over" image
                draw_texture(resources.over_texture, 0., 0., WHITE);
            }
        }
    }

    fn space_pressed(&mut self) -> bool {
        if is_key_down(KeyCode::Space) {
            if self.space_down {
                // Space was down previous frame, and is still down
                false
            } else {
                // Space wasn't down previous frame, but now is
                self.space_down = true;
                true
            }
        } else {
            self.space_down = false;
            false
        }
    }

    fn draw_status(&self) {
        println!("WRITEME: GlobalStatus#draw_status")
    }
}
