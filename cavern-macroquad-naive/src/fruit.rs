use macroquad::{
    prelude::{collections::storage, Texture2D},
    rand::ChooseRandom,
};

use crate::{
    actor::{Actor, Anchor},
    collide_actor::CollideActor,
    gravity_actor::{GravityActor, GRAVITY_ACTOR_DEFAULT_ANCHOR},
    player::Player,
    pop::Pop,
    resources::Resources,
    robot::RobotType,
};

#[derive(Clone, Copy)]
pub enum FruitType {
    Apple,
    Raspberry,
    Lemon,
    ExtraHealth,
    ExtraLife,
}

impl FruitType {
    fn val(&self) -> i32 {
        match self {
            FruitType::Apple => 0,
            FruitType::Raspberry => 1,
            FruitType::Lemon => 2,
            FruitType::ExtraHealth => 3,
            FruitType::ExtraLife => 4,
        }
    }
}

pub struct Fruit {
    pub time_to_live: i32,
    pub trapped_enemy_type: Option<RobotType>,
    pub type_: FruitType,

    // Actor trait
    pub x: i32,
    pub y: i32,
    pub image: Texture2D,
    pub anchor: Anchor,

    // GravityActor trait
    pub vel_y: i32,
    pub landed: bool,
}

impl Fruit {
    pub fn new(x: i32, y: i32, trapped_enemy_type: Option<RobotType>) -> Self {
        // Choose which type of fruit we're going to be.
        let type_ = if let Some(RobotType::Normal) = trapped_enemy_type {
            *[FruitType::Apple, FruitType::Raspberry, FruitType::Lemon]
                .to_vec()
                .choose()
                .unwrap()
        } else {
            // If trapped_enemy_type is 1, it means this fruit came from bursting an orb containing the more dangerous type
            // of enemy. In this case there is a chance of getting an extra help or extra life power up
            // We create a list containing the possible types of fruit, in proportions based on the probability we want
            // each type of fruit to be chosen
            let mut types = [FruitType::Apple, FruitType::Raspberry, FruitType::Lemon].repeat(10); // Each of these appear in the list 10 times
            types.extend([FruitType::ExtraHealth].repeat(9)); // This appears 9 times
            types.extend([FruitType::ExtraLife]); // This only appears once
            *types.choose().unwrap() // Randomly choose one from the list
        };

        Self {
            time_to_live: 500, // Counts down to zero
            trapped_enemy_type,
            type_,

            x,
            y,
            image: storage::get::<Resources>().blank_texture,
            anchor: GRAVITY_ACTOR_DEFAULT_ANCHOR,

            vel_y: 0,
            landed: false,
        }
    }

    pub fn update(
        &mut self,
        pops: &mut Vec<Pop>,
        player: Option<&mut Player>,
        game_timer: i32,
        grid: &[&str],
    ) {
        GravityActor::update(self, true, grid);

        // Does the player exist, and are they colliding with us?

        match player {
            Some(player) if player.collidepoint(self.center()) => {
                match self.type_ {
                    FruitType::ExtraHealth => {
                        player.health = 3.min(player.health + 1);
                        eprint!("WRITEME: play sound inside Fruit#update");
                        // game.play_sound("bonus");
                    }
                    FruitType::ExtraLife => {
                        player.lives += 1;
                        eprint!("WRITEME: play sound inside Fruit#update");
                        // game.play_sound("bonus");
                    }
                    _ => {
                        player.score += (self.type_.val() + 1) * 100;
                        eprint!("WRITEME: play sound inside Fruit#update");
                        // game.play_sound("score");
                    }
                }

                self.time_to_live = 0; // Disappear
            }
            _ => {
                self.time_to_live -= 1;
            }
        }

        if self.time_to_live <= 0 {
            // Create 'pop' animation
            pops.push(Pop::new(self.x, self.y - 27, 0));
        }

        let type_factor = self.type_.val() * 3;
        let timer_factor = [0, 1, 2, 1][((game_timer / 6) % 4) as usize];
        let image_i = (type_factor + timer_factor) as usize;
        self.image = storage::get::<Resources>().fruit_textures[image_i];
    }
}

impl Actor for Fruit {
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

impl CollideActor for Fruit {}

impl GravityActor for Fruit {
    fn vel_y(&self) -> i32 {
        self.vel_y
    }

    fn vel_y_mut(&mut self) -> &mut i32 {
        &mut self.vel_y
    }

    fn landed(&self) -> bool {
        self.landed
    }

    fn landed_mut(&mut self) -> &mut bool {
        &mut self.landed
    }
}
