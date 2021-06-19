use std::error;

use macroquad::{
    audio::{self, Sound},
    prelude::{load_texture, Texture2D},
};

pub struct Resources {
    pub title_texture: Texture2D,
    pub over_texture: Texture2D,
    pub space_textures: Vec<Texture2D>,

    pub over_sound: Sound,
    pub level_sound: Sound,
}

impl Resources {
    pub async fn new() -> Result<Resources, Box<dyn error::Error>> {
        let title_texture = load_texture("resources/images/title.png").await?;
        let over_texture = load_texture("resources/images/over.png").await?;
        // Async blocks are (as of Jun/2021) unstable!
        let mut space_textures = vec![];
        for i in 0..10 {
            space_textures.push(load_texture(&format!("resources/images/space{}.png", i)).await?);
        }

        let over_sound = audio::load_sound("resources/sounds/over0.ogg").await?;
        let level_sound = audio::load_sound("resources/sounds/level0.ogg").await?;

        Ok(Resources {
            title_texture,
            over_texture,
            space_textures,

            over_sound,
            level_sound,
        })
    }
}
