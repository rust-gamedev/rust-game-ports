// TODO: Remove me. Temporary until the codebase is fully written.
#![allow(dead_code)]

mod ball;
mod bat;
mod controls;
mod game;
mod global_state;
mod impact;
mod state;

use std::env;
use std::path::PathBuf;

use ggez::{event, GameResult};

use global_state::GlobalState;

const RESOURCES_DIR_NAME: &str = "resources";
const RESOURCE_SUBDIRS: [&str; 3] = ["images", "music", "sounds"];

const GAME_ID: &str = "Boing!";
const AUTHOR: &str = "Saverio Miroddi";

const WINDOW_TITLE: &str = GAME_ID;
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 480.;

const HALF_WIDTH: f32 = WINDOW_WIDTH / 2.;
const HALF_HEIGHT: f32 = WINDOW_HEIGHT / 2.;

fn get_resource_dirs() -> Vec<PathBuf> {
    let resources_root_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push(RESOURCES_DIR_NAME);
        path
    } else {
        PathBuf::from(RESOURCES_DIR_NAME)
    };

    RESOURCE_SUBDIRS
        .iter()
        .map(|subdir| resources_root_dir.join(subdir))
        .collect()
}

fn main() -> GameResult {
    let resource_dirs = get_resource_dirs();

    let mut context_builder = ggez::ContextBuilder::new(GAME_ID, AUTHOR)
        .window_setup(ggez::conf::WindowSetup::default().title(WINDOW_TITLE))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));

    for dir in resource_dirs {
        context_builder = context_builder.add_resource_path(dir);
    }

    let (mut context, event_loop) = context_builder.build()?;
    let mut state = GlobalState::new(&mut context);

    state.play_music(&mut context)?;

    event::run(context, event_loop, state)
}
