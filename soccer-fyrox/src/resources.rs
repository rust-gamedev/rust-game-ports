use std::collections::HashMap;

use fyrox::{
    core::futures::executor::block_on, engine::resource_manager::ResourceManager,
    resource::texture::Texture,
};
use imagesize::ImageSize;

const ZERO_ORD: u8 = '0' as u8;

const IMAGE_PATHS: &'static [&'static str] = &[
    "resources/images/menu01.png",
    "resources/images/menu02.png",
    "resources/images/menu10.png",
    "resources/images/menu11.png",
    "resources/images/menu12.png",
];

pub struct Resources {
    images: HashMap<String, (Texture, f32, f32)>,
}

impl Resources {
    pub fn load(resource_manager: &ResourceManager) -> Self {
        let mut images = HashMap::new();

        for path in IMAGE_PATHS {
            let texture = block_on(resource_manager.request_texture(path)).unwrap();
            let ImageSize { width, height } = imagesize::size(path).unwrap();

            images.insert(path.to_string(), (texture, width as f32, height as f32));
        }

        Self { images }
    }

    pub fn image(&self, base: &str, indexes: &[u8]) -> (Texture, f32, f32) {
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
