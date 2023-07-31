use ggez::{graphics::Image, Context, GameResult};

use crate::graphic_entity::GraphicEntity;

/// Type for an animation which is displayed briefly whenever the ball bounces
pub struct Impact {
    pub x: f32,
    pub y: f32,
    pub time: u8,

    current_image: usize,
    images: Vec<Image>,
}

impl GraphicEntity for Impact {
    fn image(&self) -> &Image {
        &self.images[self.current_image]
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

impl Impact {
    pub fn new(context: &mut Context, x: f32, y: f32) -> Self {
        // There are 5 impact sprites numbered 0 to 4. We update to a new sprite every 2 frames.
        let images = (0..5)
            .map(|i| {
                let image_filename = format!("/impact{}.png", i / 2);
                Image::from_path(context, image_filename).unwrap()
            })
            .collect();

        Self {
            x,
            y,
            time: 0,
            current_image: 0,
            images,
        }
    }

    pub fn update(&mut self, _context: &mut Context) -> GameResult {
        self.current_image = self.time as usize / 2;

        // The Game type maintains a list of Impact instances. In Game.update, if the timer for an
        // object has gone beyond 10, the object is removed from the list.
        self.time += 1;

        Ok(())
    }
}
