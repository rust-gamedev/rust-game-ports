use crate::{
    bunner::Bunner,
    drawing::{display_number, NumberAlign, NumberColor},
    game::Game,
    position::Position,
    resources::Resources,
    state::State,
    HEIGHT, WIDTH,
};
use macroquad::{
    audio::{play_sound, set_sound_volume, PlaySoundParams, Sound},
    prelude::{collections::storage, draw_texture, miniquad, rand, KeyCode, WHITE},
};
use std::collections::VecDeque;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;

pub struct GlobalState {
    state: State,
    game: Game,
    high_score: u32,
    music: Sound,
    input_queue: VecDeque<KeyCode>,
}

impl miniquad::EventHandler for GlobalState {
    fn update(&mut self, _ctx: &mut miniquad::Context) {}

    fn draw(&mut self, _ctx: &mut miniquad::Context) {}

    fn key_down_event(
        &mut self,
        _ctx: &mut miniquad::Context,
        keycode: KeyCode,
        _keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        self.input_queue.push_back(keycode);
    }
}

impl GlobalState {
    pub fn new(music: Sound) -> Self {
        Self {
            // Set the initial game state
            state: State::Menu,
            game: Game::new(None),
            high_score: 0,
            music,
            input_queue: VecDeque::new(),
        }
    }

    pub fn init(&mut self) {
        rand::srand(macroquad::miniquad::date::now() as u64);
        play_sound(
            self.music,
            PlaySoundParams {
                looped: true,
                volume: 1.,
            },
        );
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.high_score = fs::read_to_string("high.txt")
                .map_or(Ok(0), |i| i.parse::<u32>())
                .unwrap_or(0);
        }
    }

    pub fn update(&mut self) {
        match self.state {
            State::Menu => {
                if self.input_queue.contains(&KeyCode::Space) {
                    // Switch to play state, and create a new Game object, passing it a new Player object to use
                    self.state = State::Play;
                    self.game = Game::new(Some(Bunner::new(Position::new(240, -320))));
                    set_sound_volume(self.music, 0.3);
                } else {
                    self.game.update(self.input_queue.clone());
                }
            }
            State::Play => {
                if self.game.game_over() {
                    self.high_score = self.high_score.max(self.game.score());
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        fs::write("high.txt", self.high_score.to_string()).ok();
                    }

                    self.state = State::GameOver;
                } else {
                    self.game.update(self.input_queue.clone());
                }
            }
            State::GameOver => {
                if self.input_queue.contains(&KeyCode::Space) {
                    // Switch to menu state, and create a new game object
                    self.state = State::Menu;
                    self.game = Game::new(None);
                    set_sound_volume(self.music, 1.0);
                }
            }
        }
        self.input_queue.clear();
    }

    pub fn draw(&mut self) {
        let resources = storage::get::<Resources>();

        self.game.draw();

        match self.state {
            State::Menu => {
                // Draw title screen
                draw_texture(resources.title_texture, 0., 0., WHITE);
                let index: usize = ((self.game.scroll_pos.abs() / 6) % 4) as usize;
                if let Some(start_index) = [0, 1, 2, 1].get(index) {
                    draw_texture(
                        resources.start_textures[*start_index],
                        (WIDTH - 270) as f32 / 2.,
                        (HEIGHT - 240) as f32,
                        WHITE,
                    );
                };
            }
            State::Play => {
                // Display score and high score
                display_number(self.game.score(), NumberColor::Blue, 0, NumberAlign::Left);
                display_number(
                    self.high_score,
                    NumberColor::Yellow,
                    WIDTH - 10,
                    NumberAlign::Right,
                );
            }
            State::GameOver => {
                // Display "Game Over" image
                draw_texture(resources.gameover_texture, 0., 0., WHITE);
            }
        }
    }
}
