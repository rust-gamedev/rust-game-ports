// #![allow(unused_imports)]
// #![allow(unused_variables)]
#![allow(dead_code)]

mod controls;
mod difficulty;
mod game;
mod game_global;
mod input_controller;
mod media;
mod menu_state;
mod state;
mod team;

use fyrox::engine::framework::Framework;
use game_global::GameGlobal;

const TITLE: &str = "Substitute Soccer";

fn main() {
    Framework::<GameGlobal>::new().unwrap().title(TITLE).run();
}
