use std::{collections::HashMap, error};

use macroquad::{
    audio::{self, Sound},
    prelude::{load_texture, Texture2D},
};

const AVAILABLE_FONTS: [u8; 37] = [
    32, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77,
    78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
];

pub struct Resources {
    pub title_texture: Texture2D,
    pub over_texture: Texture2D,
    pub space_textures: Vec<Texture2D>,
    pub status_textures: HashMap<&'static str, Texture2D>,

    pub over_sound: Sound,
    pub level_sound: Sound,

    pub fonts: HashMap<u8, Texture2D>,
}

impl Resources {
    pub async fn new() -> Result<Resources, Box<dyn error::Error>> {
        // Async blocks are (as of Jun/2021) unstable, so cycles are used where required.

        let title_texture = load_texture("resources/images/title.png").await?;
        let over_texture = load_texture("resources/images/over.png").await?;
        let mut space_textures = vec![];
        for i in 0..10 {
            space_textures.push(load_texture(&format!("resources/images/space{}.png", i)).await?);
        }
        let mut status_textures = HashMap::new();
        for status in ["life", "plus", "health"] {
            let filename = format!("resources/images/{}.png", status);
            let texture = load_texture(&filename).await?;
            status_textures.insert(status, texture);
        }

        let over_sound = audio::load_sound("resources/sounds/over0.ogg").await?;
        let level_sound = audio::load_sound("resources/sounds/level0.ogg").await?;

        let mut fonts = HashMap::new();
        for chr in AVAILABLE_FONTS {
            let filename = format!("resources/images/font0{:02}.png", chr);
            let font = load_texture(&filename).await?;
            fonts.insert(chr, font);
        }

        Ok(Resources {
            title_texture,
            over_texture,
            space_textures,
            status_textures,

            over_sound,
            level_sound,

            fonts,
        })
    }
}
