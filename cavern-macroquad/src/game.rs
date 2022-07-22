use crate::actor::Actor;
use crate::bolt::Bolt;
use crate::fruit::Fruit;
use crate::game_playback::play_game_sound;
use crate::orb::RcOrb;
use crate::pop::Pop;
use crate::resources::Resources;
use crate::robot::{Robot, RobotType};
use crate::{levels::LEVELS, player::Player};
use crate::{GRID_BLOCK_SIZE, LEVEL_X_OFFSET, NUM_COLUMNS, NUM_ROWS, WIDTH};

use macroquad::prelude::{draw_texture, WHITE};
use macroquad::rand::gen_range;
use macroquad::{prelude::collections::storage, rand::ChooseRandom};

#[derive(Default)]
pub struct Game {
    pub player: Option<Player>,
    pub level_colour: i8,
    pub level: i8,
    pub timer: i32,
    pub grid: Vec<&'static str>,

    pub fruits: Vec<Fruit>,
    pub bolts: Vec<Bolt>,
    pub enemies: Vec<Robot>,
    pub pending_enemies: Vec<RobotType>,
    pub pops: Vec<Pop>,
    pub orbs: Vec<RcOrb>,
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
            if self.grid[0].as_bytes()[grid_x as usize] == b' ' {
                return GRID_BLOCK_SIZE * grid_x + LEVEL_X_OFFSET + 12;
            }
        }

        // If we failed to find an opening in the top grid row (shouldn't ever happen), just spawn the enemy
        // in the centre of the screen
        WIDTH / 2
    }

    pub fn update(&mut self) {
        self.timer += 1;

        // Rust: We precompute this here, since it can't be done inside the enemies cycle, due to borrowing
        // rules.
        let fire_probability = self.fire_probability();

        // Update all objects
        for fruit in &mut self.fruits {
            fruit.update(&mut self.pops, self.player.as_mut(), self.timer, &self.grid)
        }
        for bolt in &mut self.bolts {
            bolt.update(&mut self.orbs, self.player.as_mut(), self.timer, &self.grid)
        }
        for enemy in &mut self.enemies {
            enemy.update(
                &mut self.bolts,
                &mut self.orbs,
                self.player.as_ref(),
                fire_probability,
                self.timer,
                &self.grid,
            )
        }
        self.pops.iter_mut().for_each(|p| p.update());
        if let Some(p) = &mut self.player {
            p.update(&mut self.orbs, &self.grid, self.timer);
        }
        for orb in &mut self.orbs {
            orb.borrow_mut().update(
                &mut self.fruits,
                &mut self.pops,
                self.player.as_ref(),
                &self.grid,
            )
        }

        // Remove objects which are no longer wanted from the lists. For example, we recreate
        // self.fruits such that it contains all existing fruits except those whose time_to_live counter has reached zero
        self.fruits.retain(|f| f.time_to_live > 0);
        self.bolts.retain(|b| b.active);
        self.enemies.retain(|e| e.alive);
        self.pops.retain(|p| p.timer < 12);
        self.orbs
            .retain(|o| o.borrow().timer < 250 && o.borrow().y > -40);

        // Every 100 frames, create a random fruit (unless there are no remaining enemies on this level)
        if self.timer % 100 == 0 && (self.pending_enemies.len() + self.enemies.len()) > 0 {
            // Create fruit at random position
            self.fruits.push(Fruit::new(
                gen_range(70, 730 + 1),
                gen_range(75, 400 + 1),
                None,
            ));
        }

        // Every 81 frames, if there is at least 1 pending enemy, and the number of active enemies is below the current
        // level's maximum enemies, create a robot
        if self.timer % 81 == 0
            && self.pending_enemies.len() > 0
            && self.enemies.len() < self.max_enemies() as usize
        {
            // Retrieve and remove the last element from the pending enemies list
            let robot_type = self.pending_enemies.pop().unwrap();
            let (x, y) = (self.get_robot_spawn_x(), -30);
            self.enemies.push(Robot::new(x, y, robot_type));
        }

        // End level if there are no enemies remaining to be created, no existing enemies, no fruit, no popping orbs,
        // and no orbs containing trapped enemies. (We don't want to include orbs which don't contain trapped enemies,
        // as the level would never end if the player kept firing new orbs)
        if self.pending_enemies.len() + self.fruits.len() + self.enemies.len() + self.pops.len()
            == 0
        {
            if self
                .orbs
                .iter()
                .all(|orb| orb.borrow().trapped_enemy_type.is_none())
            {
                self.next_level();
            }
        }
    }

    pub fn draw(&self) {
        let resources = storage::get::<Resources>();

        draw_texture(
            resources.background_textures[self.level as usize % 4],
            0.,
            0.,
            WHITE,
        );

        let block_sprite = resources.block_textures[(self.level % 4) as usize];

        // Display blocks
        for row_y in 0..NUM_ROWS {
            let row = self.grid[row_y as usize];
            if row.len() > 0 {
                // Initial offset - large blocks at edge of level are 50 pixels wide
                let mut x = LEVEL_X_OFFSET;
                for block in row.chars() {
                    if block != ' ' {
                        draw_texture(
                            block_sprite,
                            x as f32,
                            (row_y * GRID_BLOCK_SIZE) as f32,
                            WHITE,
                        );
                    }
                    x += GRID_BLOCK_SIZE;
                }
            }
        }

        // Draw all objects
        self.fruits.iter().for_each(|f| f.draw());
        self.bolts.iter().for_each(|b| b.draw());
        self.enemies.iter().for_each(|e| e.draw());
        self.pops.iter().for_each(|p| p.draw());
        self.orbs.iter().for_each(|o| o.borrow().draw());
        if let Some(p) = &self.player {
            p.draw();
        }
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

        play_game_sound(
            self.player.as_ref(),
            &storage::get::<Resources>().level_sound,
        );
    }
}
