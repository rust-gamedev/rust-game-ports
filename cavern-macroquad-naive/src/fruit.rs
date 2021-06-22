use macroquad::{
    prelude::{collections::storage, Texture2D},
    rand::ChooseRandom,
};

use crate::{
    actor::{Actor, Anchor},
    collide_actor::CollideActor,
    gravity_actor::{GravityActor, GRAVITY_ACTOR_DEFAULT_ANCHOR},
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

    pub fn update(&mut self) {
        eprintln!("WRITEME: Fruit#update");
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
