use macroquad::{
    prelude::{collections::storage, Texture2D},
    rand::{gen_range, ChooseRandom},
};

use crate::bolt::Bolt;
use crate::{
    actor::{Actor, Anchor},
    collide_actor::CollideActor,
    gravity_actor::GravityActor,
    orb::Orb,
    player::Player,
    resources::Resources,
};

#[derive(Clone, Copy)]
pub enum RobotType {
    Aggressive,
    Normal,
}

impl RobotType {
    fn val(&self) -> i32 {
        match self {
            RobotType::Aggressive => 0,
            RobotType::Normal => 1,
        }
    }
}

pub struct Robot {
    pub type_: RobotType,
    pub speed: i32,
    pub direction_x: i32,
    pub alive: bool,
    pub change_dir_timer: i32,
    pub fire_timer: i32,

    // Actor trait
    pub x: i32,
    pub y: i32,
    pub image: Texture2D,
    pub anchor: Anchor,
}

impl Robot {
    pub fn new(x: i32, y: i32, type_: RobotType) -> Self {
        Self {
            x,
            y,
            image: storage::get::<Resources>().blank_texture,
            anchor: Anchor::CentreBottom,
            type_,
            speed: gen_range(1, 4),
            direction_x: 1,
            alive: true,
            change_dir_timer: 0,
            fire_timer: 100,
        }
    }

    pub fn update(
        &mut self,
        bolts: &mut Vec<Bolt>,
        orbs: &mut [Orb],
        player: Option<&Player>,
        mut fire_probability: f32,
        game_timer: i32,
    ) {
        GravityActor::update(self);

        self.change_dir_timer -= 1;
        self.fire_timer += 1;

        // Move in current direction - turn around if we hit a wall
        if self.move_(self.direction_x, 0, self.speed) {
            self.change_dir_timer = 0;
        }

        if self.change_dir_timer <= 0 {
            // Randomly choose a direction to move in
            // If there's a player, there's a two thirds chance that we'll move towards them
            let mut directions = vec![-1, 1];
            if player.is_some() {
                directions.push((player.unwrap().x() - self.x()).signum());
            }
            self.direction_x = *directions.choose().unwrap();
            self.change_dir_timer = gen_range(100, 250 + 1);
        }

        // The more powerful type of robot can deliberately shoot at orbs - turning to face them if necessary
        if matches!(self.type_, RobotType::Aggressive) && self.fire_timer >= 24 {
            // Go through all orbs to see if any can be shot at
            for orb in orbs.iter_mut() {
                // The orb must be at our height, and within 200 pixels on the x axis
                if orb.y >= self.top() && orb.y < self.bottom() && (orb.x() - self.x()).abs() < 200
                {
                    self.direction_x = (orb.x() - self.x()).signum();
                    self.fire_timer = 0;
                    break;
                }
            }
        }

        // Check to see if we can fire at player
        if self.fire_timer >= 12 {
            // Random chance of firing each frame. Likelihood increases 10 times if player is at the same height as us
            if let Some(player) = player {
                if self.top() < player.bottom() && self.bottom() > player.top() {
                    fire_probability *= 10.;
                }
            }
            if gen_range(0., 1.) < fire_probability {
                self.fire_timer = 0;
                eprint!("WRITEME: play_sound inside Robot#update()");
                //game.play_sound("laser", 4);
            }
        } else if self.fire_timer == 8 {
            //  Once the fire timer has been set to 0, it will count up - frame 8 of the animation is when the actual bolt is fired
            bolts.push(Bolt::new(
                self.x() + self.direction_x * 20,
                self.y() - 38,
                self.direction_x,
            ));
        }

        // Am I colliding with an orb? If so, become trapped by it
        for orb in orbs.iter_mut() {
            if orb.trapped_enemy_type.is_none() && self.collidepoint(orb.center()) {
                self.alive = false;
                orb.floating = true;
                orb.trapped_enemy_type = Some(self.type_);
                eprint!("WRITEME: play_sound inside Robot#update()");
                //game.play_sound("trap", 4);
                break;
            }
        }

        // Choose and set sprite image
        let type_factor = 16 * self.type_.val();
        let direction_factor = if self.direction_x > 0 { 8 } else { 0 };
        let fire_factor = if self.fire_timer < 12 {
            5 + (self.fire_timer / 4)
        } else {
            1 + ((game_timer / 4) % 4)
        };
        let image_i = (type_factor + direction_factor + fire_factor) as usize;
        self.image = storage::get::<Resources>().robot_textures[image_i];
    }
}

impl Actor for Robot {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn image(&self) -> macroquad::prelude::Texture2D {
        self.image
    }

    fn anchor(&self) -> Anchor {
        self.anchor
    }
}

impl CollideActor for Robot {}

impl GravityActor for Robot {}
