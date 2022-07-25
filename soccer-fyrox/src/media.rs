use std::{collections::HashMap, fmt::Display, fs::read_dir, path::PathBuf};

use fyrox::{
    core::futures::{executor::block_on, future::join_all},
    engine::resource_manager::ResourceManager,
    resource::texture::Texture,
    scene::sound::{SoundBufferResource, SoundBuilder, Status},
};

use crate::prelude::*;

const ZERO_ORD: u8 = b'0';

// Drawing is skipped for images with this name.
//
pub const BLANK_IMAGE: &str = "blank";

// Use Media::build_path to access resources.
//
const RESOURCES_PATH: &str = "resources";
const IMAGES_PATH: &str = "images";
const SOUNDS_PATH: &str = "sounds";

// Avoid loading other files, ie. .options
//
const SUPPORTED_IMAGE_EXTENSIONS: &[&str] = &[".gif", ".png"];
const SUPPORTED_SOUND_EXTENSIONS: &[&str] = &[".ogg"];

// It's not easy to make the overall design of the program simple, since Fyrox requires several elements
// to be carried around (scene, handles, resources...).
// For a simple game like this, a simple type like this will do, and it will take care of everything.
pub struct Media {
    image_textures: HashMap<String, Texture>,
    sound_resources: HashMap<String, SoundBufferResource>,
    looping_sounds: HashMap<String, Handle<Node>>,
}

impl Media {
    pub fn new(resource_manager: &ResourceManager) -> Self {
        let images_path = Self::resource_path(&[IMAGES_PATH]);

        let image_paths = read_dir(images_path)
            .unwrap()
            .filter_map(|entry| {
                let filename = entry.unwrap().path().to_string_lossy().into_owned();

                SUPPORTED_IMAGE_EXTENSIONS
                    .iter()
                    .any(|ext| filename.ends_with(ext))
                    .then_some(filename)
            })
            .collect::<Vec<String>>();

        // As of Fyrox v0.25, loading textures in debug mode is extremely slow (1.4" for each PNG file,
        // even if small), so we need to load them asynchronously.
        //
        let texture_requests = join_all(
            image_paths
                .iter()
                .map(|path| resource_manager.request_texture(path)),
        );

        let sounds_path = Self::resource_path(&[SOUNDS_PATH]);

        let sound_paths = read_dir(sounds_path)
            .unwrap()
            .filter_map(|entry| {
                let filename = entry.unwrap().path().to_string_lossy().into_owned();

                SUPPORTED_SOUND_EXTENSIONS
                    .iter()
                    .any(|ext| filename.ends_with(ext))
                    .then_some(filename)
            })
            .collect::<Vec<String>>();

        let sound_requests = join_all(
            sound_paths
                .iter()
                .map(|path| resource_manager.request_sound_buffer(path)),
        );

        // For simplicity, we strip the extension, and assume:
        //
        // - that there are no images with the same bare name but different extension
        // - that all the extensions are 3 chars long
        //
        let image_textures = image_paths
            .iter()
            .zip(block_on(texture_requests))
            .map(|(path, texture)| {
                (
                    path[..(path.len() - 4)].to_string(),
                    texture.unwrap_or_else(|_| panic!("Error while loading image file '{}'", path)),
                )
            })
            .collect::<HashMap<_, _>>();

        let sound_resources = sound_paths
            .iter()
            .zip(block_on(sound_requests))
            .map(|(path, sound)| {
                (
                    path.to_string(),
                    sound.unwrap_or_else(|_| panic!("Error while loading sound file: '{}'", path)),
                )
            })
            .collect::<HashMap<_, _>>();

        let looping_sounds = HashMap::new();

        Self {
            image_textures,
            sound_resources,
            looping_sounds,
        }
    }

    pub fn play_sound(&self, scene: &mut Scene, base: &str, indexes: &[u8]) {
        let sound = self.sound(&base, indexes);

        SoundBuilder::new(BaseBuilder::new())
            .with_buffer(Some(sound))
            .with_status(Status::Playing)
            .with_play_once(true)
            .build(&mut scene.graph);
    }

    // In PyGame, music is a streamed (and repeated) sound; in Fyrox, this can also be enabled programmatically,
    // but the simplest thing is to use an options file - see `theme.ogg.options`.
    //
    // In the source project, looping sounds don't have an index.
    //
    // We could merge this and the play_sound(), but it's simpler (API-wise) to separate them, taking
    // advantage of the two points above.
    //
    pub fn play_looping_sound(&mut self, scene: &mut Scene, name: &str) {
        let sound = self.sound(name, &[]);

        let node = SoundBuilder::new(BaseBuilder::new())
            .with_buffer(Some(sound))
            .with_looping(true)
            .with_status(Status::Playing)
            .build(&mut scene.graph);

        self.looping_sounds.insert(name.to_string(), node);
    }

    // The source project allows attempting to stop a sound that hasn't been started.
    //
    // Looping sounds don't have an index (see play_sound()).
    //
    pub fn stop_looping_sound(&mut self, scene: &mut Scene, base: &str) {
        if let Some(sound_h) = self.looping_sounds.remove(base) {
            let sound = &mut scene.graph[sound_h];

            // Removing the node also stops the sound, so this is technically redundant.
            //
            sound.as_sound_mut().stop();

            scene.remove_node(sound_h);
        }
    }

    pub fn image<S: AsRef<str> + Display>(&self, base: S, indexes: &[u8]) -> Texture {
        if indexes.len() > 3 {
            panic!();
        }

        let mut filename = base.to_string();

        for index in indexes {
            filename.push((ZERO_ORD + index) as char);
        }

        let full_path = Self::resource_path(&[IMAGES_PATH, &filename]);

        self.image_textures
            .get(&full_path)
            .unwrap_or_else(|| panic!("Image '{}' not found!", &full_path))
            .clone()
    }

    // Substantially common with the above. May optionally base both on a shared API.
    //
    fn sound<S: AsRef<str> + Display>(&self, base: S, indexes: &[u8]) -> SoundBufferResource {
        if indexes.len() > 1 {
            panic!();
        }

        let mut filename = base.to_string();

        for index in indexes {
            filename.push((ZERO_ORD + index) as char);
        }

        filename.push_str(".ogg");

        let full_path = Self::resource_path(&[SOUNDS_PATH, &filename]);

        self.sound_resources
            .get(&full_path)
            .unwrap_or_else(|| panic!("Sound '{}' not found!", &full_path))
            .clone()
    }

    fn resource_path(paths: &[&str]) -> String {
        paths
            .iter()
            .fold(PathBuf::from(RESOURCES_PATH), |result, current| {
                result.join(current)
            })
            .to_string_lossy()
            .into_owned()
    }
}
