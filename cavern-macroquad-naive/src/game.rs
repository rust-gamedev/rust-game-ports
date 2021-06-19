use crate::resources::Resources;
use crate::robot::RobotType;
use crate::{levels::LEVELS, player::Player};

use macroquad::{
    audio::{self, Sound},
    prelude::collections::storage,
    rand::ChooseRandom,
};

#[derive(Default)]
pub struct Game {
    pub player: Option<Player>,
    pub level_colour: i8,
    pub level: i8,
    pub timer: i32,
    pub grid: Vec<&'static str>,

    pub fruits: Vec<u32>,
    pub bolts: Vec<u32>,
    pub enemies: Vec<u32>,
    pub pending_enemies: Vec<RobotType>,
    pub pops: Vec<u32>,
    pub orbs: Vec<u32>,
}

impl Game {
    pub fn new(player: Option<Player>) -> Self {
        let mut game = Self {
            player,
            level_colour: -1,
            level: -1,
            timer: -1,
            ..Default::default()
        };

        game.next_level();

        game
    }

    pub fn play_sound(&self, sound: &Sound) {
        if self.player.is_some() {
            audio::play_sound_once(*sound);
        }
    }

    #[allow(dead_code)]
    pub fn play_random_sound(&self, sounds: Vec<Sound>) {
        self.play_sound(sounds.choose().unwrap())
    }

    pub fn update(&mut self) {
        self.timer += 1;

        eprintln!("WRITEME: Game#update");
    }

    pub fn draw(&self) {
        eprintln!("WRITEME: Game#draw");
    }

    fn next_level(&mut self) {
        self.level_colour = (self.level_colour + 1) % 4;
        self.level += 1;

        // Set up grid
        self.grid = LEVELS[(self.level as usize) % LEVELS.len()].to_vec();

        // The last row is a copy of the first row
        self.grid.push(self.grid[0]);

        self.timer = -1;

        if let Some(player) = &mut self.player {
            player.reset();
        }

        self.fruits = vec![];
        self.bolts = vec![];
        self.enemies = vec![];
        self.pops = vec![];
        self.orbs = vec![];

        // At the start of each level we create a list of pending enemies - enemies to be created as the level plays out.
        // When this list is empty, we have no more enemies left to create, and the level will end once we have destroyed
        // all enemies currently on-screen. Each element of the list will be either 0 or 1, where 0 corresponds to
        // a standard enemy, and 1 is a more powerful enemy.
        // First we work out how many total enemies and how many of each type to create
        let num_enemies = 10 + self.level as usize;
        let num_strong_enemies = 1 + (self.level as f32 / 1.5) as usize;
        let num_weak_enemies = num_enemies - num_strong_enemies;

        // Then we create the list of pending enemies. The resulting list will consist of a series of copies of
        // the number RobotType::Aggressive (the number depending on the value of num_strong_enemies), followed by a
        // series of copies of RobotType::Normal, based on num_weak_enemies.
        self.pending_enemies = [RobotType::Aggressive].repeat(num_strong_enemies);
        self.pending_enemies
            .append(&mut [RobotType::Normal].repeat(num_weak_enemies));

        // Finally we shuffle the list so that the order is randomised
        self.pending_enemies.shuffle();

        self.play_sound(&storage::get::<Resources>().level_sound);
    }
}
