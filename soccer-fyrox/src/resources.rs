use std::collections::HashMap;

use fyrox::{
    core::futures::{executor::block_on, future::join_all},
    engine::resource_manager::ResourceManager,
    resource::texture::Texture,
};

const ZERO_ORD: u8 = '0' as u8;

const IMAGE_PATHS: &'static [&'static str] = &[
    "resources/images/menu01.png",
    "resources/images/menu02.png",
    "resources/images/menu10.png",
    "resources/images/menu11.png",
    "resources/images/menu12.png",
];

pub struct Resources {
    images: HashMap<String, Texture>,
}

impl Resources {
    pub fn load(resource_manager: &ResourceManager) -> Self {
        // As of Fyrox v0.25, loading textures in debug mode is extremely slow (1.4" for each PNG file,
        // even if small), so we need to load them asynchronously.
        //
        let texture_requests = join_all(
            IMAGE_PATHS
                .iter()
                .map(|path| resource_manager.request_texture(path)),
        );

        let images = IMAGE_PATHS
            .iter()
            .zip(block_on(texture_requests))
            .map(|(path, texture)| (path.to_string(), texture.unwrap()))
            .collect::<HashMap<_, _>>();

        Self { images }
    }

    pub fn image(&self, base: &str, indexes: &[u8]) -> Texture {
        if indexes.len() > 2 {
            panic!();
        }

        let mut full_path = format!("resources/images/{}", base);

        for index in indexes {
            full_path.push((ZERO_ORD + index) as char);
        }

        full_path.push_str(".png");

        self.images[&full_path].clone()
    }
}
