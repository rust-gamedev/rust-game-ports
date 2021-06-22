use macroquad::{
    prelude::{collections::storage, Texture2D},
    rand::gen_range,
};

use crate::{
    actor::{Actor, Anchor},
    bolt::Bolt,
    collide_actor::CollideActor,
    fruit::Fruit,
    pop::Pop,
    resources::Resources,
    robot::RobotType,
};

const MAX_TIMER: i32 = 250;

#[derive(Clone, Copy)]
pub struct Orb {
    pub direction_x: i32,
    pub timer: i32,
    pub floating: bool,
    /// Number of frames during which we will be pushed horizontally
    pub blown_frames: i32,
    /// Type of enemy trapped in this bubble
    pub trapped_enemy_type: Option<RobotType>,

    // Actor trait
    pub x: i32,
    pub y: i32,
    pub image: Texture2D,
    pub anchor: Anchor,
}

impl Orb {
    pub fn new(x: i32, y: i32, direction_x: i32) -> Self {
        Self {
            direction_x, // Orbs are initially blown horizontally, then start floating upwards
            timer: -1,
            floating: false,
            blown_frames: 6,
            trapped_enemy_type: None,
            x,
            y,
            image: storage::get::<Resources>().blank_texture,
            anchor: Anchor::Centre,
        }
    }

    #[allow(dead_code)]
    pub fn hit_test(&mut self, bolt: &Bolt) -> bool {
        // Check for collision with a bolt
        let collided = self.collidepoint((bolt.x, bolt.y));
        if collided {
            self.timer = MAX_TIMER - 1;
        }
        collided
    }

    pub fn update(&mut self, fruits: &mut Vec<Fruit>, pops: &mut Vec<Pop>, grid: &[&str]) {
        self.timer += 1;

        if self.floating {
            // Float upwards
            self.move_(0, -1, gen_range(1, 3), grid);
        } else {
            // Move horizontally
            if self.move_(self.direction_x, 0, 4, grid) {
                // If we hit a block, start floating
                self.floating = true;
            }
        }

        if self.timer == self.blown_frames {
            self.floating = true;
        } else if self.timer >= MAX_TIMER || self.y <= -40 {
            // Pop if our lifetime has run out or if we have gone off the top of the screen
            pops.push(Pop::new(self.x, self.y, 1));
            if let Some(trapped_enemy_type) = self.trapped_enemy_type {
                // trapped_enemy_type is either zero or one. A value of one means there's a chance of creating a
                // powerup such as an extra life or extra health
                fruits.push(Fruit::new(self.x, self.y, Some(trapped_enemy_type)));
            }
            eprintln!("WRITEME: play sound inside Orb#update");
            // game.play_sound("pop", 4);
        }

        let resources = storage::get::<Resources>();

        if self.timer < 9 {
            // Orb grows to full size over the course of 9 frames - the animation frame updating every 3 frames
            let timer_factor = self.timer / 3;
            self.image = resources.orb_textures[timer_factor as usize];
        } else {
            if let Some(trapped_enemy_type) = self.trapped_enemy_type {
                let enemy_type_factor = trapped_enemy_type.val() * 8;
                let timer_factor = (self.timer / 4) % 8;
                let image_i = (enemy_type_factor + timer_factor) as usize;
                self.image = resources.trap_textures[image_i];
            } else {
                let timer_factor = 3 + (((self.timer - 9) / 8) % 4);
                self.image = resources.orb_textures[timer_factor as usize];
            }
        }
    }
}

impl Actor for Orb {
    fn x(&self) -> i32 {
        self.x
    }

    fn x_mut(&mut self) -> &mut i32 {
        &mut self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn y_mut(&mut self) -> &mut i32 {
        &mut self.y
    }

    fn image(&self) -> macroquad::prelude::Texture2D {
        self.image
    }

    fn anchor(&self) -> crate::actor::Anchor {
        self.anchor
    }
}

impl CollideActor for Orb {}
