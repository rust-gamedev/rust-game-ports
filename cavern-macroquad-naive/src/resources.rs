use std::{collections::HashMap, error};

use macroquad::{
    audio::{self, Sound},
    prelude::{load_texture, Texture2D},
};

const AVAILABLE_FONTS: [u8; 37] = [
    32, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77,
    78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
];

// Async blocks are (as of Jun/2021) unstable, so cycles are used where required.
//
async fn load_textures_list(
    name_prefix: &str,
    number: u8,
) -> Result<Vec<Texture2D>, Box<dyn error::Error>> {
    let mut textures = vec![];

    for i in 0..number {
        textures.push(load_texture(&format!("resources/images/{}{}.png", name_prefix, i)).await?);
    }

    Ok(textures)
}

async fn load_textures_map(
    names: &[&'static str],
) -> Result<HashMap<&'static str, Texture2D>, Box<dyn error::Error>> {
    let mut textures = HashMap::new();

    for name in names {
        let filename = format!("resources/images/{}.png", name);
        let texture = load_texture(&filename).await?;
        textures.insert(*name, texture);
    }

    Ok(textures)
}

async fn load_robot_textures() -> Result<Vec<Texture2D>, Box<dyn error::Error>> {
    let mut textures = vec![];

    textures.extend(load_textures_list("robot00", 8).await?);
    textures.extend(load_textures_list("robot01", 8).await?);
    textures.extend(load_textures_list("robot10", 8).await?);
    textures.extend(load_textures_list("robot11", 8).await?);

    Ok(textures)
}

pub struct Resources {
    pub title_texture: Texture2D,
    pub over_texture: Texture2D,
    pub space_textures: Vec<Texture2D>,
    pub status_textures: HashMap<&'static str, Texture2D>,
    pub background_textures: Vec<Texture2D>,
    pub block_textures: Vec<Texture2D>,
    pub blank_texture: Texture2D,
    /// Rust: Stored contiguously ("00..." -> "01..." -> "10..." -> "11...")
    pub robot_textures: Vec<Texture2D>,

    pub over_sound: Sound,
    pub level_sound: Sound,

    pub fonts: HashMap<u8, Texture2D>,
}

impl Resources {
    pub async fn new() -> Result<Resources, Box<dyn error::Error>> {
        let title_texture = load_texture("resources/images/title.png").await?;
        let over_texture = load_texture("resources/images/over.png").await?;
        let space_textures = load_textures_list("space", 10).await?;
        let status_textures = load_textures_map(&["life", "plus", "health"]).await?;
        let background_textures = load_textures_list("bg", 4).await?;
        let block_textures = load_textures_list("block", 4).await?;
        let blank_texture = load_texture("resources/images/blank.png").await?;
        let robot_textures = load_robot_textures().await?;

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
            background_textures,
            block_textures,
            blank_texture,
            robot_textures,

            over_sound,
            level_sound,

            fonts,
        })
    }
}
