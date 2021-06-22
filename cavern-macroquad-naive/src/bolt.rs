use macroquad::prelude::{collections::storage, Texture2D};

use crate::{
    actor::{Actor, Anchor},
    collide_actor::{CollideActor, COLLIDE_ACTOR_DEFAULT_ANCHOR},
    orb::RcOrb,
    player::Player,
    resources::Resources,
};

const BOLT_SPEED: i32 = 7;

pub struct Bolt {
    pub direction_x: i32,
    pub active: bool,

    // Actor trait
    pub x: i32,
    pub y: i32,
    pub image: Texture2D,
    pub anchor: Anchor,
}

impl Bolt {
    pub fn new(x: i32, y: i32, direction_x: i32) -> Self {
        Self {
            direction_x,
            active: true,

            x,
            y,
            image: storage::get::<Resources>().blank_texture,
            anchor: COLLIDE_ACTOR_DEFAULT_ANCHOR,
        }
    }

    pub fn update(
        &mut self,
        orbs: &mut Vec<RcOrb>,
        player: Option<&mut Player>,
        game_timer: i32,
        grid: &[&str],
    ) {
        // Move horizontally and check to see if we've collided with a block
        if self.move_(self.direction_x, 0, BOLT_SPEED, grid) {
            // Collided
            self.active = false;
        } else {
            // We didn't collide with a block - check to see if we collided with an orb or the player
            for orb in orbs {
                if orb.borrow_mut().hit_test(self) {
                    self.active = false;
                    break;
                }
            }

            if self.active {
                if let Some(player) = player {
                    if player.hit_test(self) {
                        self.active = false;
                    }
                }
            }
        }

        let direction_factor = if self.direction_x > 0 { 2 } else { 0 };
        let timer_factor = (game_timer / 4) % 2;
        let image_i = (direction_factor + timer_factor) as usize;
        self.image = storage::get::<Resources>().bolt_textures[image_i];
    }
}

impl Actor for Bolt {
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

impl CollideActor for Bolt {}
