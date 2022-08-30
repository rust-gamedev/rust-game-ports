use crate::{
    child::Child, player_state::PlayerState, position::Position, resources::Resources, road::Road,
    row::Row, train::Train, water::Water, HEIGHT, ROW_HEIGHT, WIDTH,
};

use macroquad::{
    audio::play_sound_once,
    prelude::collections::storage,
    rand::{self, ChooseRandom},
    texture::Texture2D,
};

#[derive(Clone)]
pub struct Rail {
    index: i32,
    y: i32,
    children: Vec<Child>,
}

impl Row for Rail {
    fn y(&self) -> i32 {
        self.y
    }

    fn children(&self) -> &[Child] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Child> {
        self.children.as_mut()
    }

    fn update(&mut self, scroll_pos: i32, _bunner_pos: Option<Position>) {
        self.update_children();
        if self.index == 2 {
            self.children
                .retain(|c| c.x() > -1000 && c.x() < WIDTH + 1000);
            if self.y < scroll_pos + HEIGHT
                && self.children.is_empty()
                && rand::gen_range::<u8>(0, 100) < 1
            {
                let dx = *vec![-20, 20].choose().unwrap();
                let position = if dx < 0 {
                    Position::new(WIDTH + 1000, 47)
                } else {
                    Position::new(WIDTH - 1000, 47)
                };
                self.children.push(Child::Train(Train::new(dx, position)));
                play_sound_once(storage::get::<Resources>().bell_sound);
                let train_sound = *storage::get::<Resources>()
                    .train_sounds
                    .get(rand::gen_range::<usize>(0, 2))
                    .unwrap();
                play_sound_once(train_sound);
            }
        }
    }

    fn image(&self) -> Texture2D {
        *storage::get::<Resources>()
            .rail_textures
            .get(self.index as usize)
            .unwrap()
    }

    fn play_sound(&self) {
        play_sound_once(storage::get::<Resources>().grass_sound);
    }

    fn next(&self) -> Box<dyn Row> {
        let y = self.y - ROW_HEIGHT;
        if self.index < 3 {
            Box::new(Rail::new(self.index + 1, y))
        } else if rand::gen_range::<u8>(0, 2) == 0 {
            Box::new(Road::empty(y))
        } else {
            Box::new(Water::empty(y))
        }
    }

    fn check_collision(&self, x: i32) -> PlayerState {
        if self.index == 2 && self.collide(x, 0) {
            return PlayerState::Splat(8);
        }
        PlayerState::Alive
    }
}

impl Rail {
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
