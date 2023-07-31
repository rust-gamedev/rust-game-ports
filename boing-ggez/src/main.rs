#![allow(clippy::all)]
#![deny(clippy::correctness)]

mod audio_entity;
mod ball;
mod bat;
mod controls;
mod game;
mod global_state;
mod graphic_entity;
mod impact;
mod state;

use std::env;
use std::path::PathBuf;

use ggez::{event, graphics::Rect, winit::dpi::PhysicalSize, Context, GameResult};

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
        .map(|subdir| resources_root_dir.join(subdir).canonicalize().unwrap())
        .collect()
}

// Returns the viewport and scissor coordinates.
//
fn compute_viewport(context: &Context) -> (Rect, Rect) {
    // Assume that the pixels are square.
    //
    let PhysicalSize {
        width: screen_physical_width,
        height: screen_physical_height,
    } = context.gfx.window().inner_size();

    let window_ratio = WINDOW_WIDTH / WINDOW_HEIGHT;
    let screen_physical_ratio = screen_physical_width as f32 / screen_physical_height as f32;

    let (screen_logical_width, screen_logical_height, logical_scale) =
        if screen_physical_ratio >= window_ratio {
            (
                WINDOW_HEIGHT * screen_physical_ratio,
                WINDOW_HEIGHT,
                screen_physical_height as f32 / WINDOW_HEIGHT,
            )
        } else {
            (
                WINDOW_WIDTH,
                WINDOW_WIDTH / screen_physical_ratio,
                screen_physical_width as f32 / WINDOW_WIDTH,
            )
        };

    let horizontal_bar_width = (screen_logical_width - WINDOW_WIDTH) / 2.;
    let vertical_bar_height = (screen_logical_height - WINDOW_HEIGHT) / 2.;

    let viewport_rect = Rect::new(
        -horizontal_bar_width,
        -vertical_bar_height,
        screen_logical_width,
        screen_logical_height,
    );

    let scissors_rect = Rect::new(
        (screen_physical_width as f32 - WINDOW_WIDTH * logical_scale) / 2.,
        (screen_physical_height as f32 - WINDOW_HEIGHT * logical_scale) / 2.,
        viewport_rect.w * logical_scale,
        viewport_rect.h * logical_scale,
    );

    (viewport_rect, scissors_rect)
}

fn main() -> GameResult {
    let resource_dirs = get_resource_dirs();

    let mut context_builder = ggez::ContextBuilder::new(GAME_ID, AUTHOR)
        .window_setup(ggez::conf::WindowSetup::default().title(WINDOW_TITLE))
        .window_mode(
            ggez::conf::WindowMode::default().fullscreen_type(ggez::conf::FullscreenType::Desktop),
        );

    for dir in resource_dirs {
        context_builder = context_builder.add_resource_path(dir);
    }

    let (mut context, event_loop) = context_builder.build()?;

    let (viewport_rect, scissors_rect) = compute_viewport(&context);

    let mut state = GlobalState::new(&mut context, viewport_rect, scissors_rect);

    state.play_music(&mut context)?;

    event::run(context, event_loop, state)
}
