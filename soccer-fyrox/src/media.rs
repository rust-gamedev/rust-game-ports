use std::{collections::HashMap, fmt::Display, fs::read_dir, path::PathBuf};

use fyrox::{
    core::futures::{executor::block_on, future::join_all},
    engine::resource_manager::ResourceManager,
    resource::texture::{Texture, TextureKind},
    scene::{
        dim2::rectangle::RectangleBuilder,
        pivot::PivotBuilder,
        sound::{SoundBufferResource, SoundBuilder, Status},
    },
};

use crate::prelude::*;

const ZERO_ORD: u8 = '0' as u8;

// Drawing is skipped for images with this name.
//
pub const BLANK_IMAGE: &str = "blank";

// Use Media::build_path to access resources.
//
const RESOURCES_PATH: &str = "resources";
const IMAGES_PATH: &str = "images";
const SOUNDS_PATH: &str = "sounds";

// It's not easy to make the overall design of the program simple, since Fyrox requires several elements
// to be carried around (scene, handles, resources...).
// For a simple game like this, a simple type like this will do, and it will take care of everything.
pub struct Media {
    image_textures: HashMap<String, Texture>,
    sound_resources: HashMap<String, SoundBufferResource>,
    images_root: Handle<Node>,
    looping_sounds: HashMap<String, Handle<Node>>,
}

impl Media {
    pub fn new(resource_manager: &ResourceManager, scene: &mut Scene) -> Self {
        let images_path = Self::resource_path(&[IMAGES_PATH]);

        let image_paths = read_dir(images_path)
            .unwrap()
            .map(|entry| entry.unwrap().path().to_string_lossy().into_owned())
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
            .map(|entry| entry.unwrap().path().to_string_lossy().into_owned())
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
            .map(|(path, texture)| (path[..(path.len() - 4)].to_string(), texture.expect(path)))
            .collect::<HashMap<_, _>>();

        let sound_resources = sound_paths
            .iter()
            .zip(block_on(sound_requests))
            .map(|(path, sound)| (path.to_string(), sound.expect(path)))
            .collect::<HashMap<_, _>>();

        let images_root = PivotBuilder::new(BaseBuilder::new()).build(&mut scene.graph);

        let looping_sounds = HashMap::new();

        Self {
            image_textures,
            sound_resources,
            images_root,
            looping_sounds,
        }
    }

    // The simplest way to model a design that is as close a possible to a conventional 2d game
    // library, is to a pivot node as root node, and to dynamically add each sprite to draw as node.
    // By scaling the pivot node to the screen size, we don't need to scale the sprites.
    //
    pub fn clear_images(&mut self, scene: &mut Scene) {
        for child in scene.graph[self.images_root].children().to_vec() {
            scene.graph.remove_node(child);
        }
    }

    // This is a convenience, to make comparison with the source project simpler.
    // Blitting is equal to drawing with a TopLeft Anchor.
    pub fn blit_image(
        &mut self,
        scene: &mut Scene,
        base: &str,
        indexes: &[u8],
        std_x: f32,
        std_y: f32,
        z: f32,
    ) {
        self.draw_image(scene, base, indexes, std_x, std_y, z, Anchor::TopLeft);
    }

    // Draws the image (loads the texture, adds the node to the scene, and links it to the root).
    //
    // The coordinates ("std" = "standard") are the typical orientation used for 2d libraries (center
    // at top left, x -> right, y -> down).
    //
    // This is difficult to name, since the semantics of Fyrox and (simple) 2d games are different.
    //
    pub fn draw_image(
        &mut self,
        scene: &mut Scene,
        base: &str,
        indexes: &[u8],
        std_x: f32,
        std_y: f32,
        z: f32,
        anchor: Anchor,
    ) {
        if base == BLANK_IMAGE {
            return;
        }

        let texture = self.image(base, indexes);
        let texture_kind = texture.data_ref().kind();

        if let TextureKind::Rectangle {
            width: texture_width,
            height: texture_height,
        } = texture_kind
        {
            use Anchor::*;

            let (texture_width, texture_height) = (texture_width as f32, texture_height as f32);

            // As a base, we start with the top left corner of the screen, and we subtract the "standard"
            // coordinates, since they go to the opposite direction to the Fyrox ones.
            //
            let (mut fyrox_x, mut fyrox_y) = (WIDTH / 2. - std_x, HEIGHT / 2. - std_y);

            match anchor {
                Center => {
                    // Do nothing
                }
                TopLeft => {
                    // Shift the texture, to the bottom right, of half texture.
                    //
                    fyrox_x = fyrox_x - texture_width / 2.;
                    fyrox_y = fyrox_y - texture_height / 2.;
                }
                Custom(anchor) => {
                    // Shift bottom right like TopLeft, then shift top left according to the anchor.
                    //
                    fyrox_x = fyrox_x - texture_width / 2. + anchor.x;
                    fyrox_y = fyrox_y - texture_height / 2. + anchor.y;
                }
            };

            let node = RectangleBuilder::new(
                BaseBuilder::new().with_local_transform(
                    TransformBuilder::new()
                        .with_local_position(Vector3::new(fyrox_x, fyrox_y, z))
                        .with_local_scale(Vector3::new(texture_width, texture_height, f32::EPSILON))
                        .build(),
                ),
            )
            .with_texture(texture)
            .build(&mut scene.graph);

            scene.graph.link_nodes(node, self.images_root);
        } else {
            panic!("Texture is not a rectangle!")
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

    // In PyGame, music is a streamed (and repeated) sound; in Fyrox, there isn't the streaming concept.
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

    fn image<S: AsRef<str> + Display>(&self, base: S, indexes: &[u8]) -> Texture {
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
            .expect(&format!("Image '{}' not found!", &full_path))
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
            .expect(&format!("Sound '{}' not found!", &full_path))
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
