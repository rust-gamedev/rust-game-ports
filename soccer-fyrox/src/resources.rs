use std::collections::HashMap;

use fyrox::{
    core::futures::executor::block_on, engine::resource_manager::ResourceManager,
    resource::texture::Texture,
};

const IMAGE_PATHS: [(&str, u16, u16); 2] = [
    ("resources/images/menu01.png", 800, 480),
    ("resources/images/menu02.png", 800, 480),
];

pub struct Resources {
    images: HashMap<String, (Texture, f32, f32)>,
}

impl Resources {
    pub fn load(resource_manager: &ResourceManager) -> Self {
        let mut images = HashMap::new();

        for (path, width, height) in IMAGE_PATHS {
            let texture = block_on(resource_manager.request_texture(path)).unwrap();
            images.insert(path.to_string(), (texture, width as f32, height as f32));
        }

        Self { images }
    }

    pub fn image(&self, base: &str, i1: u8, i2: u8) -> (Texture, f32, f32) {
        let full_path = format!("resources/images/{}{}{}.png", base, i1, i2);

        self.images[&full_path].clone()
    }
}
