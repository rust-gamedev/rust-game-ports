use macroquad::{
    audio::{self, Sound},
    rand::ChooseRandom,
};

use crate::player::Player;

pub struct Game {
    pub player: Option<Player>,
    pub timer: i32,
}

impl Game {
    pub fn new(player: Option<Player>) -> Self {
        Self { player, timer: -1 }
    }

    pub fn update(&mut self) {
        self.timer += 1;

        println!("WRITEME: Game#update");
    }

    pub fn draw(&self) {
        println!("WRITEME: Game#draw");
    }

    pub fn play_sound(&self, sound: &Sound) {
        if self.player.is_some() {
            audio::play_sound_once(*sound);
        }
    }

    pub fn play_random_sound(&self, sounds: Vec<Sound>) {
        self.play_sound(sounds.choose().unwrap())
    }
}
