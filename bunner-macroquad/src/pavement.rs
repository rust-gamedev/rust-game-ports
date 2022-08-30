use crate::{child::Child, resources::Resources, road::Road, row::Row, ROW_HEIGHT};
use macroquad::{audio::play_sound_once, prelude::collections::storage, texture::Texture2D};

#[derive(Clone)]
pub struct Pavement {
    index: i32,
    y: i32,
    children: Vec<Child>,
}

impl Row for Pavement {
    fn y(&self) -> i32 {
        self.y
    }

    fn children(&self) -> &[Child] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Child> {
        self.children.as_mut()
    }

    fn image(&self) -> Texture2D {
        *storage::get::<Resources>()
            .side_textures
            .get(self.index as usize)
            .unwrap()
    }

    fn play_sound(&self) {
        play_sound_once(storage::get::<Resources>().sidewalk_sound);
    }

    fn next(&self) -> Box<dyn Row> {
        let y = self.y - ROW_HEIGHT;
        if self.index < 2 {
            Box::new(Pavement::new(self.index + 1, y))
        } else {
            Box::new(Road::empty(y))
        }
    }
}

impl Pavement {
    pub fn new(index: i32, y: i32) -> Self {
        Self {
            index,
            y,
            children: Vec::new(),
        }
    }

    pub fn empty(y: i32) -> Self {
        Self::new(0, y)
    }
}
