use macroquad::prelude::{collections::storage, draw_texture, is_key_pressed, KeyCode, WHITE};

use crate::{
    drawing::{draw_game_text, CHAR_WIDTH, IMAGE_WIDTH},
    game::Game,
    game_playback::play_game_sound,
    player::Player,
    resources::Resources,
    state::State,
    WIDTH,
};

pub struct GlobalState {
    state: State,
    game: Game,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            // Set the initial game state
            state: State::Menu,
            game: Game::new(None),
        }
    }

    pub fn update(&mut self) {
        match self.state {
            State::Menu => {
                if is_key_pressed(KeyCode::Space) {
                    // Switch to play state, and create a new Game object, passing it a new Player object to use
                    self.state = State::Play;
                    self.game = Game::new(Some(Player::new()));
                } else {
                    self.game.update();
                }
            }
            State::Play => {
                if self.game.player.as_ref().unwrap().lives < 0 {
                    play_game_sound(
                        self.game.player.as_ref(),
                        &storage::get::<Resources>().over_sound,
                    );
                    self.state = State::GameOver;
                } else {
                    self.game.update();
                }
            }
            State::GameOver => {
                if is_key_pressed(KeyCode::Space) {
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

    fn draw_status(&self) {
        // For Rust convenience
        let player = self.game.player.as_ref().unwrap();

        // Display score, right-justified at edge of screen
        let number_width = CHAR_WIDTH[0];
        let s = player.score.to_string();
        draw_game_text(&s, 451, Some(WIDTH - 2 - (number_width * s.len() as i32)));

        // Display level number
        draw_game_text(&format!("LEVEL {}", self.game.level + 1), 451, None);

        // Display lives and health
        // We only display a maximum of two lives - if there are more than two, a plus symbol is displayed
        let mut lives_health = ["life"].repeat(2.min(player.lives as usize));
        if player.lives > 2 {
            lives_health.push("plus");
        }
        if player.lives >= 0 {
            lives_health.extend(["health"].repeat(player.health as usize));
        };

        let status_textures = &storage::get::<Resources>().status_textures;

        let mut x = 0;
        for image in lives_health {
            let texture = status_textures[image];
            draw_texture(texture, x as f32, 450., WHITE);
            x += IMAGE_WIDTH[image];
        }
    }
}
