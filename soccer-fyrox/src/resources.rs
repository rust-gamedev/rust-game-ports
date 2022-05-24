use std::collections::HashMap;

use fyrox::{
    core::futures::executor::block_on, engine::resource_manager::ResourceManager,
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
        let mut images = HashMap::new();

        // As of Fyrox v0.25, loading textures in debug mode is extremely slow (1.4" for each PNG file,
        // even if small), so we need to load them asynchronously.
        // Note that if we don't `collect()`, the compiler merges mapping and iteration, causing the
        // textures to be loaded serially.
        //
        let texture_requests = IMAGE_PATHS
            .iter()
            .map(|path| (path, resource_manager.request_texture(path)))
            .collect::<Vec<_>>();

        for (path, texture_request) in texture_requests {
            let texture = block_on(texture_request).unwrap();

            images.insert(path.to_string(), texture);
        }

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
