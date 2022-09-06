use crate::{
    active_row::ActiveRow, child::Child, dirt::Dirt, log::Log, player_state::PlayerState,
    position::Position, resources::Resources, row::Row, row::RowSound, ROW_HEIGHT, WIDTH,
};
use macroquad::{
    audio::play_sound_once,
    prelude::collections::storage,
    rand::{self},
    texture::Texture2D,
};

#[derive(Clone)]
pub struct Water {
    dx: i32,
    timer: f32,
    index: i32,
    y: i32,
    children: Vec<Child>,
}

impl Row for Water {
    fn y(&self) -> i32 {
        self.y
    }

    fn children(&self) -> &[Child] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Child> {
        self.children.as_mut()
    }

    fn update(&mut self, _scroll_pos: i32, _bunner_pos: Option<Position>) {
        self.update_children();
        self.children.retain(|c| c.x() > -70 && c.x() < WIDTH + 70);
        self.timer -= 1.;

        // Create new child objects on a random interval
        if self.timer < 0. {
            self.children.push(self.create_random_child(self.dx));
            self.timer = self.random_interval(self.dx);
        }
    }

    fn image(&self) -> Texture2D {
        *storage::get::<Resources>()
            .water_textures
            .get(self.index as usize)
            .unwrap()
    }

    fn play_sound(&self) {
        play_sound_once(storage::get::<Resources>().log_sound);
    }

    fn next(&self) -> Box<dyn Row> {
        let y = self.y - ROW_HEIGHT;
        if self.index == 7 || (self.index >= 1 && rand::gen_range(0, 2) == 0) {
            Box::new(Dirt::new(rand::gen_range(4, 7), y))
        } else {
            Box::new(Water::new(self.dx, self.index + 1, y))
        }
    }

    fn check_collision(&self, x: i32) -> PlayerState {
        if self.collide(x, -4) {
            return PlayerState::Alive;
        }
        PlayerState::Splash
    }

    fn push(&self) -> i32 {
        self.dx
    }

    fn sound(&self) -> Option<RowSound> {
        Some(RowSound::River)
    }
}

impl ActiveRow for Water {
    fn build_child(dx: i32, position: Position) -> Child {
        Child::Log(Log::new(dx, position))
    }
}

impl Water {
    pub fn new(previous_dx: i32, index: i32, y: i32) -> Self {
        let dx = if previous_dx >= 0 {
            -rand::gen_range(1, 3)
        } else {
            rand::gen_range(1, 3)
        };
        Self {
            dx,
            timer: 0.,
            index,
            y,
            children: Self::build_children(dx),
        }
    }

    pub fn empty(y: i32) -> Self {
        Self::new(0, 0, y)
    }
}
