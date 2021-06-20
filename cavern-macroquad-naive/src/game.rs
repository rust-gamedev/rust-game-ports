use crate::resources::Resources;
use crate::robot::RobotType;
use crate::{levels::LEVELS, player::Player};
use crate::{GRID_BLOCK_SIZE, LEVEL_X_OFFSET, NUM_COLUMNS, WIDTH};

use macroquad::rand::gen_range;
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

    #[allow(dead_code)]
    pub fn fire_probability(&self) -> f32 {
        // Likelihood per frame of each robot firing a bolt - they fire more often on higher levels
        0.001 + (0.0001 * 100.min(self.level) as f32)
    }

    pub fn max_enemies(&self) -> i32 {
        // Maximum number of enemies on-screen at once – increases as you progress through the levels
        ((self.level + 6) / 2).min(8) as i32
    }

    pub fn get_robot_spawn_x(&self) -> i32 {
        // Find a spawn location for a robot, by checking the top row of the grid for empty spots
        // Start by choosing a random grid column
        let r = gen_range(0, NUM_COLUMNS);

        for i in 0..NUM_COLUMNS {
            // Keep looking at successive columns (wrapping round if we go off the right-hand side) until
            // we find one where the top grid column is unoccupied
            let grid_x = (r + i) % NUM_COLUMNS;
            if self.grid[0].as_bytes()[grid_x as usize] == ' ' as u8 {
                return GRID_BLOCK_SIZE * grid_x + LEVEL_X_OFFSET + 12;
            }
        }

        // If we failed to find an opening in the top grid row (shouldn't ever happen), just spawn the enemy
        // in the centre of the screen
        WIDTH / 2
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
