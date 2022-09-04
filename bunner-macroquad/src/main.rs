#![allow(clippy::all)]
#![deny(clippy::correctness)]

use macroquad::{
    audio::{self},
    input::{is_key_pressed, utils::*, KeyCode},
    prelude::{next_frame, Conf},
    time::get_frame_time,
};

use bunner_macroquad::{
    global_state::GlobalState, resources::Resources, HEIGHT, TIME_PER_FRAME, TITLE, WIDTH,
};

use std::error;

fn window_conf() -> Conf {
    Conf {
        window_title: TITLE.into(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() -> Result<(), Box<dyn error::Error>> {
    Resources::load().await?;

    // Start music
    let music = audio::load_sound("resources/music/theme.ogg").await?;
    let mut global_state = GlobalState::new(music);
    global_state.init();

    let input_subscriber = register_input_subscriber();
    let mut frame_time: f32 = 0.;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            std::process::exit(0);
        }
        repeat_all_miniquad_input(&mut global_state, input_subscriber);
        frame_time += get_frame_time().min(0.25);
        while frame_time >= TIME_PER_FRAME {
            global_state.update();
            frame_time -= TIME_PER_FRAME;
        }
        global_state.draw();

        next_frame().await
    }
}
