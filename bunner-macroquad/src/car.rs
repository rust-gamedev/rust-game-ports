use crate::{actor::Actor, mover::Mover, position::Position, resources::Resources};
use macroquad::{
    audio::play_sound_once,
    prelude::{collections::storage, draw_texture, WHITE},
    rand::{self, ChooseRandom},
};
use std::collections::HashSet;

#[derive(Clone)]
pub struct Car {
    dx: i32,
    position: Position,
    image_index: usize,
    played_sounds: HashSet<CarSound>,
}

impl Mover for Car {
    fn dx(&self) -> i32 {
        self.dx
    }
}

impl Actor for Car {
    fn update(&mut self) {
        self.position.x += self.dx;
    }

    fn draw(&self, offset_x: i32, offset_y: i32) {
        let image = *storage::get::<Resources>()
            .car_textures
            .get(self.image_index)
            .unwrap();
        draw_texture(
            image,
            (self.position.x + offset_x) as f32 - image.width() / 2.,
            (self.position.y + offset_y) as f32 - image.height(),
            WHITE,
        );
    }

    fn x(&self) -> i32 {
        self.position.x
    }

    fn y(&self) -> i32 {
        self.position.y
    }

    fn width(&self) -> i32 {
        90
    }
}

impl Car {
    pub fn new(dx: i32, position: Position) -> Self {
        let image_index = if dx < 0 {
            *vec![0, 2, 4].choose().unwrap()
        } else {
            *vec![1, 3, 5].choose().unwrap()
        };
        Self {
            dx,
            position,
            image_index,
            played_sounds: HashSet::new(),
        }
    }

    pub fn play_sound(&mut self, sound: CarSound) {
        if self.played_sounds.insert(sound.clone()) {
            match sound {
                CarSound::Zoom => {
                    let rnd = rand::gen_range::<usize>(0, 6);
                    play_sound_once(storage::get::<Resources>().zoom_sounds[rnd]);
                }
                CarSound::Honk => {
                    let rnd = rand::gen_range::<usize>(0, 4);
                    play_sound_once(storage::get::<Resources>().honk_sounds[rnd]);
                }
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum CarSound {
    Zoom,
    Honk,
}

pub struct TrafficSound {
    pub y_offset: i32,
    pub sound: CarSound,
}

impl TrafficSound {
    pub fn new(y_offset: i32, sound: CarSound) -> Self {
        TrafficSound { y_offset, sound }
    }
}
