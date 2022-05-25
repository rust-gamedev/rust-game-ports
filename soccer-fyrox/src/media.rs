use std::{collections::HashMap, fmt::Display};

use fyrox::{
    core::{
        algebra::Vector3,
        futures::{executor::block_on, future::join_all},
        pool::Handle,
    },
    engine::resource_manager::ResourceManager,
    resource::texture::Texture,
    scene::{
        base::BaseBuilder,
        dim2::rectangle::RectangleBuilder,
        graph::Graph,
        node::Node,
        pivot::PivotBuilder,
        sound::{SoundBufferResource, SoundBuilder, Status},
        transform::TransformBuilder,
        Scene,
    },
};

use crate::game_global;

const ZERO_ORD: u8 = '0' as u8;

const IMAGE_PATHS: &'static [&'static str] = &[
    "resources/images/menu01.png",
    "resources/images/menu02.png",
    "resources/images/menu10.png",
    "resources/images/menu11.png",
    "resources/images/menu12.png",
];

const SOUND_PATHS: &'static [&'static str] =
    &["resources/sounds/move.ogg", "resources/music/theme.ogg"];

// It's not easy to make the overall design of the program simple, since Fyrox requires several elements
// to be carried around (scene, handles, resources...).
// For a simple game like this, a simple type like this will do, and it will take care of everything.
pub struct Media {
    image_textures: HashMap<String, Texture>,
    sound_resources: HashMap<String, SoundBufferResource>,
    images_root: Handle<Node>,
    music: Option<Handle<Node>>,
}

impl Media {
    pub fn new(resource_manager: &ResourceManager, scene: &mut Scene) -> Self {
        // As of Fyrox v0.25, loading textures in debug mode is extremely slow (1.4" for each PNG file,
        // even if small), so we need to load them asynchronously.
        //
        let texture_requests = join_all(
            IMAGE_PATHS
                .iter()
                .map(|path| resource_manager.request_texture(path)),
        );

        let sound_requests = join_all(
            SOUND_PATHS
                .iter()
                .map(|path| resource_manager.request_sound_buffer(path)),
        );

        let image_textures = IMAGE_PATHS
            .iter()
            .zip(block_on(texture_requests))
            .map(|(path, texture)| (path.to_string(), texture.unwrap()))
            .collect::<HashMap<_, _>>();

        let sound_resources = SOUND_PATHS
            .iter()
            .zip(block_on(sound_requests))
            .map(|(path, texture)| (path.to_string(), texture.unwrap()))
            .collect::<HashMap<_, _>>();

        let images_root = PivotBuilder::new(
            BaseBuilder::new().with_local_transform(
                TransformBuilder::new()
                    .with_local_scale(Vector3::new(
                        game_global::WIDTH,
                        game_global::HEIGHT,
                        f32::EPSILON,
                    ))
                    .build(),
            ),
        )
        .build(&mut scene.graph);

        let music = None;

        Self {
            image_textures,
            sound_resources,
            images_root,
            music,
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

    // Draws the image (loads the texture, adds the node to the scene, and links it to the root).
    // This is difficult to name, since the semantics of bevy and a 2d game are different.
    //
    pub fn draw_image(
        &mut self,
        scene: &mut Scene,
        base: &str,
        indexes: &[u8],
        x: i16,
        y: i16,
        z: i16,
    ) {
        let texture = self.image(base, indexes);
        let background = Self::build_image_node(&mut scene.graph, texture, x, y, z);
        scene.graph.link_nodes(background, self.images_root);
    }

    pub fn play_sound(&self, scene: &mut Scene, base: &str, indexes: &[u8]) {
        let base = "sounds/".to_string() + base;
        let sound = self.sound(base, indexes);

        SoundBuilder::new(BaseBuilder::new())
            .with_buffer(Some(sound))
            .with_status(Status::Playing)
            .with_play_once(true)
            .build(&mut scene.graph);
    }

    pub fn play_music(&mut self, scene: &mut Scene, base: &str) {
        if self.music.is_some() {
            panic!("There must be no music references, in order to play_music()");
        }

        let base = "music/".to_string() + base;
        let sound = self.sound(base, &[]);

        self.music = Some(
            SoundBuilder::new(BaseBuilder::new())
                .with_buffer(Some(sound))
                .with_looping(true)
                .with_status(Status::Playing)
                .build(&mut scene.graph),
        );
    }

    fn stop_music(&mut self, scene: &mut Scene) {
        let music_h = self
            .music
            .expect("A music reference must exist, in order to stop_music()!");

        scene.remove_node(music_h);
        self.music = None;
    }

    fn image<S: AsRef<str> + Display>(&self, base: S, indexes: &[u8]) -> Texture {
        if indexes.len() > 2 {
            panic!();
        }

        let mut full_path = format!("resources/images/{}", base);

        for index in indexes {
            full_path.push((ZERO_ORD + index) as char);
        }

        full_path.push_str(".png");

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

        let mut full_path = format!("resources/{}", base);

        for index in indexes {
            full_path.push((ZERO_ORD + index) as char);
        }

        full_path.push_str(".ogg");

        self.sound_resources
            .get(&full_path)
            .expect(&format!("Sound '{}' not found!", &full_path))
            .clone()
    }

    fn build_image_node(
        graph: &mut Graph,
        texture: Texture,
        x: i16,
        y: i16,
        z: i16,
    ) -> Handle<Node> {
        RectangleBuilder::new(
            BaseBuilder::new().with_local_transform(
                TransformBuilder::new()
                    .with_local_position(Vector3::new(x as f32, y as f32, z as f32))
                    .build(),
            ),
        )
        .with_texture(texture)
        .build(graph)
    }
}
