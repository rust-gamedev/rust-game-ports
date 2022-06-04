// #![allow(unused_imports)]
// #![allow(unused_variables)]
#![allow(dead_code)]

mod anchor;
mod ball;
mod bare_actor;
mod controls;
mod difficulty;
mod game;
mod game_global;
mod goal;
mod input_controller;
mod math_utils;
mod media;
mod menu_state;
mod my_actor;
mod player;
mod state;
mod target;
mod target_ref;
mod team;

pub mod prelude {
    pub use fyrox::{
        core::{
            algebra::{Vector2, Vector3},
            pool::{Handle, Pool},
        },
        event::VirtualKeyCode,
        scene::{base::BaseBuilder, node::Node, transform::TransformBuilder, Scene},
    };

    pub use crate::anchor::Anchor;
    pub use crate::ball::Ball;
    pub use crate::bare_actor::BareActor;
    pub use crate::controls::Controls;
    pub use crate::difficulty::{Difficulty, DIFFICULTY};
    pub use crate::game::{Game, DEFAULT_DIFFICULTY};
    pub use crate::goal::Goal;
    pub use crate::input_controller::InputController;
    pub use crate::math_utils::*;
    pub use crate::media::{Media, BLANK_IMAGE};
    pub use crate::menu_state::MenuState;
    pub use crate::my_actor::MyActor;
    pub use crate::player::Player;
    pub use crate::state::State;
    pub use crate::target::Target;
    pub use crate::target_ref::TargetRef;
    pub use crate::team::Team;
    pub use soccer_macros_fyrox::my_actor_based;

    pub const WIDTH: i16 = 800;
    pub const HEIGHT: i16 = 480;

    pub const HALF_WINDOW_W: i16 = WIDTH / 2;

    //# Size of level, including both the pitch and the boundary surrounding it
    pub const LEVEL_W: i16 = 1000;
    pub const LEVEL_H: i16 = 1400;
    pub const HALF_LEVEL_W: i16 = LEVEL_W / 2;
    pub const HALF_LEVEL_H: i16 = LEVEL_H / 2;

    pub const HALF_PITCH_W: i16 = 442;
    pub const HALF_PITCH_H: i16 = 622;
}

use fyrox::engine::framework::Framework;

use game_global::GameGlobal;

const TITLE: &str = "Substitute Soccer";

fn main() {
    Framework::<GameGlobal>::new().unwrap().title(TITLE).run();
}
