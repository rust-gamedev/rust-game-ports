// #![allow(unused_imports)]
// #![allow(unused_variables)]
#![allow(dead_code)]

mod game;

mod menu_state;
mod resources;
mod state;
mod texture_node_builder;

use fyrox::engine::framework::Framework;
use game::Game;

const TITLE: &str = "Substitute Soccer";

fn main() {
    Framework::<Game>::new().unwrap().title(TITLE).run();
}
