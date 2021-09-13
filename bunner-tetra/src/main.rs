mod bunner;
mod eagle;
mod game;
mod grass;
mod resource_path;
mod row;
mod row_type;
mod state;

use tetra::ContextBuilder;

use crate::game::Game;

const WIDTH: i32 = 480;
const HEIGHT: i32 = 800;
const TITLE: &str = "Infinite Bunner";

fn main() -> tetra::Result {
    ContextBuilder::new(TITLE, WIDTH, HEIGHT)
        .quit_on_escape(true)
        .build()?
        .run(Game::new)
}
