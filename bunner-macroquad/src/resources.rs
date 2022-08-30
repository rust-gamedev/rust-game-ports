// Rust: The data should be loaded from the on-disk list of files, rather than each type individually.
// The file naming actually helps, since "map" textures don't have an index.

use macroquad::{
    audio::{self, load_sound, Sound},
    prelude::{collections::storage, coroutines::start_coroutine, load_texture, Texture2D, *},
};
use std::error;

// Async blocks are (as of Jun/2021) unstable, so cycles are used where required.
//
async fn load_textures_list(
    name_prefix: &str,
    number: u8,
) -> Result<Vec<Texture2D>, Box<dyn error::Error>> {
    let mut textures = vec![];

    for i in 0..number {
        let path = &format!("resources/images/{}{}.png", name_prefix, i);
        textures.push(load_texture(path).await?);
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
    extension: &str,
) -> Result<Vec<Sound>, Box<dyn error::Error>> {
    let mut sounds = vec![];

    for i in 0..number {
        sounds.push(
            load_sound(&format!(
                "resources/sounds/{}{}.{}",
                name_prefix, i, extension
            ))
            .await?,
        );
    }

    Ok(sounds)
}

pub struct Resources {
    pub title_texture: Texture2D,
    pub gameover_texture: Texture2D,
    pub blank_texture: Texture2D,
    pub bush_textures: Vec<Texture2D>,
    pub car_textures: Vec<Texture2D>,
    pub digit_textures: Vec<Texture2D>,
    pub dirt_textures: Vec<Texture2D>,
    pub eagle_texture: Texture2D,
    pub eagles_texture: Texture2D,
    pub grass_textures: Vec<Texture2D>,
    pub jump_textures: Vec<Texture2D>,
    pub log_textures: Vec<Texture2D>,
    pub rail_textures: Vec<Texture2D>,
    pub road_textures: Vec<Texture2D>,
    pub side_textures: Vec<Texture2D>,
    pub sit_textures: Vec<Texture2D>,
    pub splash_textures: Vec<Texture2D>,
    pub splat_textures: Vec<Texture2D>,
    pub start_textures: Vec<Texture2D>,
    pub train_textures: Vec<Texture2D>,
    pub water_textures: Vec<Texture2D>,

    pub bell_sound: Sound,
    pub dirt_sound: Sound,
    pub eagle_sound: Sound,
    pub grass_sound: Sound,
    pub honk_sounds: Vec<Sound>,
    pub jump_sound: Sound,
    pub log_sound: Sound,
    pub river_sounds: Vec<Sound>,
    pub road_sound: Sound,
    pub sidewalk_sound: Sound,
    pub splash_sound: Sound,
    pub splat_sound: Sound,
    pub traffic_sounds: Vec<Sound>,
    pub train_sounds: Vec<Sound>,
    pub zoom_sounds: Vec<Sound>,
}

impl Resources {
    pub async fn new() -> Result<Resources, Box<dyn error::Error>> {
        let title_texture = load_texture("resources/images/title.png").await?;
        let gameover_texture = load_texture("resources/images/gameover.png").await?;
        let blank_texture = load_texture("resources/images/blank.png").await?;
        let bush_textures =
            load_multi_state_textures("bush", &["0", "1", "2", "3", "4", "5"], 2).await?;
        let car_textures = load_multi_state_textures("car", &["0", "1", "2", "3"], 2).await?;
        let digit_textures = load_multi_state_textures("digit", &["0", "1"], 10).await?;
        let dirt_textures = load_textures_list("dirt", 16).await?;
        let eagle_texture = load_texture("resources/images/eagle.png").await?;
        let eagles_texture = load_texture("resources/images/eagles.png").await?;
        let grass_textures = load_textures_list("grass", 16).await?;
        let jump_textures = load_textures_list("jump", 4).await?;
        let log_textures = load_textures_list("log", 2).await?;
        let rail_textures = load_textures_list("rail", 4).await?;
        let road_textures = load_textures_list("road", 6).await?;
        let side_textures = load_textures_list("side", 3).await?;
        let sit_textures = load_textures_list("sit", 4).await?;
        let splash_textures = load_textures_list("splash", 8).await?;
        let splat_textures = load_textures_list("splat", 4).await?;
        let start_textures = load_textures_list("start", 3).await?;
        let train_textures = load_multi_state_textures("train", &["0", "1", "2"], 2).await?;
        let water_textures = load_textures_list("water", 8).await?;

        let bell_sound = audio::load_sound("resources/sounds/bell0.wav").await?;
        let dirt_sound = audio::load_sound("resources/sounds/dirt0.wav").await?;
        let eagle_sound = audio::load_sound("resources/sounds/eagle0.wav").await?;
        let grass_sound = audio::load_sound("resources/sounds/grass0.wav").await?;
        let honk_sounds = load_sounds_list("honk", 4, "wav").await?;
        let jump_sound = audio::load_sound("resources/sounds/jump0.wav").await?;
        let log_sound = audio::load_sound("resources/sounds/log0.wav").await?;
        let river_sounds = load_sounds_list("river", 2, "ogg").await?;
        let road_sound = audio::load_sound("resources/sounds/road0.wav").await?;
        let sidewalk_sound = audio::load_sound("resources/sounds/sidewalk0.wav").await?;
        let splash_sound = audio::load_sound("resources/sounds/splash0.wav").await?;
        let splat_sound = audio::load_sound("resources/sounds/splat0.wav").await?;
        let traffic_sounds = load_sounds_list("traffic", 3, "ogg").await?;
        let train_sounds = load_sounds_list("train", 2, "wav").await?;
        let zoom_sounds = load_sounds_list("zoom", 6, "wav").await?;

        Ok(Resources {
            title_texture,
            gameover_texture,
            blank_texture,
            bush_textures,
            car_textures,
            digit_textures,
            dirt_textures,
            eagle_texture,
            eagles_texture,
            grass_textures,
            jump_textures,
            log_textures,
            rail_textures,
            road_textures,
            side_textures,
            sit_textures,
            splash_textures,
            splat_textures,
            start_textures,
            train_textures,
            water_textures,
            bell_sound,
            dirt_sound,
            eagle_sound,
            grass_sound,
            honk_sounds,
            jump_sound,
            log_sound,
            river_sounds,
            road_sound,
            sidewalk_sound,
            splash_sound,
            splat_sound,
            traffic_sounds,
            train_sounds,
            zoom_sounds,
        })
    }

    pub async fn load() -> Result<(), Box<dyn error::Error>> {
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
}
