// Rust: The data should be loaded from the on-disk list of files, rather than each type individually.
// The file naming actually helps, since "map" textures don't have an index.

use std::{collections::HashMap, error};

use macroquad::{
    audio::{self, load_sound, Sound},
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

/// Rust: Load texture that have multiple states, e.g. Robots of two types, with two directions each.
/// The textures are stored in a contiguous array, respecting the order of the states passed.
async fn load_multi_state_textures(
    name_prefix: &str,
    states: &[&str],
    state_number: u8,
) -> Result<Vec<Texture2D>, Box<dyn error::Error>> {
    let mut textures = vec![];

    for state in states {
        let prefix = &format!("{}{}", name_prefix, state);
        textures.extend(load_textures_list(prefix, state_number).await?);
    }

    Ok(textures)
}

async fn load_sounds_list(
    name_prefix: &str,
    number: u8,
) -> Result<Vec<Sound>, Box<dyn error::Error>> {
    let mut sounds = vec![];

    for i in 0..number {
        sounds.push(load_sound(&format!("resources/sounds/{}{}.ogg", name_prefix, i)).await?);
    }

    Ok(sounds)
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
    pub recoil_textures: Vec<Texture2D>,
    pub fall_textures: Vec<Texture2D>,
    pub blow_textures: Vec<Texture2D>,
    pub still_texture: Texture2D,
    /// Rust: Stored contiguously ("0..." -> "1...")
    pub run_textures: Vec<Texture2D>,
    pub orb_textures: Vec<Texture2D>,
    pub trap_textures: Vec<Texture2D>,
    pub bolt_textures: Vec<Texture2D>,
    pub pop_textures: Vec<Texture2D>,
    pub fruit_textures: Vec<Texture2D>,

    pub over_sound: Sound,
    pub level_sound: Sound,
    pub pop_sounds: Vec<Sound>,
    pub ouch_sounds: Vec<Sound>,
    pub die_sound: Sound,
    pub laser_sounds: Vec<Sound>,
    pub trap_sounds: Vec<Sound>,
    pub blow_sounds: Vec<Sound>,
    pub jump_sound: Sound,
    pub bonus_sound: Sound,
    pub score_sound: Sound,

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
        let robot_textures =
            load_multi_state_textures("robot", &["00", "01", "10", "11"], 8).await?;
        let recoil_textures = load_textures_list("recoil", 2).await?;
        let fall_textures = load_textures_list("fall", 2).await?;
        let blow_textures = load_textures_list("blow", 2).await?;
        let still_texture: Texture2D = load_texture("resources/images/still.png").await?;
        let run_textures = load_multi_state_textures("run", &["0", "1"], 4).await?;
        let orb_textures = load_textures_list("orb", 7).await?;
        let trap_textures = load_multi_state_textures("trap", &["0", "1"], 8).await?;
        let bolt_textures = load_multi_state_textures("bolt", &["0", "1"], 2).await?;
        let pop_textures = load_multi_state_textures("pop", &["0", "1"], 7).await?;
        let fruit_textures =
            load_multi_state_textures("fruit", &["0", "1", "2", "3", "4"], 3).await?;

        let over_sound = audio::load_sound("resources/sounds/over0.ogg").await?;
        let level_sound = audio::load_sound("resources/sounds/level0.ogg").await?;
        let pop_sounds = load_sounds_list("pop", 4).await?;
        let ouch_sounds = load_sounds_list("ouch", 4).await?;
        let die_sound = audio::load_sound("resources/sounds/die0.ogg").await?;
        let laser_sounds = load_sounds_list("laser", 4).await?;
        let trap_sounds = load_sounds_list("trap", 4).await?;
        let blow_sounds = load_sounds_list("blow", 4).await?;
        let jump_sound = audio::load_sound("resources/sounds/jump0.ogg").await?;
        let bonus_sound = audio::load_sound("resources/sounds/bonus0.ogg").await?;
        let score_sound = audio::load_sound("resources/sounds/score0.ogg").await?;

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
            recoil_textures,
            fall_textures,
            blow_textures,
            still_texture,
            run_textures,
            orb_textures,
            trap_textures,
            bolt_textures,
            pop_textures,
            fruit_textures,

            over_sound,
            level_sound,
            pop_sounds,
            ouch_sounds,
            die_sound,
            laser_sounds,
            trap_sounds,
            blow_sounds,
            jump_sound,
            bonus_sound,
            score_sound,

            fonts,
        })
    }
}
