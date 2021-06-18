use macroquad::prelude::*;

use std::error;

fn window_conf() -> Conf {
    Conf {
        window_title: "CavernMacroquadNaive".into(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() -> Result<(), Box<dyn error::Error>> {
    loop {
        // blah...
        next_frame().await
    }
}
