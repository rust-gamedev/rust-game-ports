mod game;
mod global_state;
mod player;
mod resources;
mod state;

use macroquad::{
    audio::{self, PlaySoundParams},
    prelude::{collections::storage, coroutines::start_coroutine, *},
};

use global_state::GlobalState;
use resources::Resources;

use std::error;

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 480;
pub const TITLE: &str = "Cavern Macroquad Naive";

fn window_conf() -> Conf {
    Conf {
        window_title: TITLE.into(),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

async fn load_resources() -> Result<(), Box<dyn error::Error>> {
    let resources_loading = start_coroutine(async move {
        let resources = Resources::new().await.unwrap();
        storage::store(resources);
    });

    while !resources_loading.is_done() {
        clear_background(BLACK);
        let text = format!(
            "Loading resources {}",
            ".".repeat(((get_time() * 2.) as usize) % 4)
        );
        draw_text(
            &text,
            screen_width() / 2. - 160.,
            screen_height() / 2.,
            40.,
            WHITE,
        );

        next_frame().await;
    }

    Ok(())
}

#[macroquad::main(window_conf())]
async fn main() -> Result<(), Box<dyn error::Error>> {
    load_resources().await?;

    let mut state = GlobalState::new();

    // Start music
    let music = audio::load_sound("resources/music/theme.ogg").await?;
    audio::play_sound(
        music,
        PlaySoundParams {
            looped: true,
            volume: 0.3,
        },
    );

    loop {
        state.update();
        state.draw();

        next_frame().await
    }
}
