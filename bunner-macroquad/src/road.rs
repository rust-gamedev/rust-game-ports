use crate::{
    active_row::ActiveRow, actor::Actor, car::Car, car::CarSound, car::TrafficSound, child::Child,
    grass::Grass, mover::Mover, pavement::Pavement, player_state::PlayerState, position::Position,
    rail::Rail, resources::Resources, row::Row, row::RowSound, ROW_HEIGHT, WIDTH,
};

use macroquad::{
    audio::play_sound_once,
    prelude::collections::storage,
    rand::{self, ChooseRandom},
    texture::Texture2D,
};

#[derive(Clone)]
pub struct Road {
    dx: i32,
    timer: f32,
    index: i32,
    y: i32,
    children: Vec<Child>,
}

impl Row for Road {
    fn y(&self) -> i32 {
        self.y
    }

    fn children(&self) -> &[Child] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Child> {
        self.children.as_mut()
    }

    fn update(&mut self, _scroll_pos: i32, bunner_pos: Option<Position>) {
        self.update_children();
        self.children.retain(|c| c.x() > -70 && c.x() < WIDTH + 70);
        self.timer -= 1.;

        // Create new child objects on a random interval
        if self.timer < 0. {
            self.children.push(self.create_random_child(self.dx));
            self.timer = self.random_interval(self.dx);
        }

        if let Some(bunner_pos) = bunner_pos {
            for traffic_sound in Road::CAR_SOUNDS.iter() {
                // Is the player on the appropriate row?
                if bunner_pos.y == self.y + traffic_sound.y_offset {
                    for child in self.children.iter_mut() {
                        if let Child::Car(car) = child {
                            // The car must be within 100 pixels of the player on the x-axis, and moving towards the player
                            // child_obj.dx < 0 is True or False depending on whether the car is moving left or right, and
                            // dx < 0 is True or False depending on whether the player is to the left or right of the car.
                            // If the results of these two comparisons are different, the car is moving towards the player.
                            // Also, for the zoom sound, the car must be travelling faster than one pixel per frame
                            let dx = car.x() - bunner_pos.x;
                            if dx.abs() < 100
                                && ((car.dx() < 0) != (dx < 0))
                                && (traffic_sound.y_offset == 0 || car.dx().abs() > 1)
                            {
                                car.play_sound(traffic_sound.sound.clone());
                            }
                        };
                    }
                }
            }
        }
    }

    fn image(&self) -> Texture2D {
        *storage::get::<Resources>()
            .road_textures
            .get(self.index as usize)
            .unwrap()
    }

    fn play_sound(&self) {
        play_sound_once(storage::get::<Resources>().road_sound);
    }

    fn next(&self) -> Box<dyn Row> {
        let y = self.y - ROW_HEIGHT;
        if self.index == 0 {
            Box::new(Road::new(self.dx, 1, y))
        } else if self.index < 5 {
            let random = rand::gen_range::<u8>(0, 100);
            if random < 80 {
                Box::new(Road::new(self.dx, self.index + 1, y))
            } else if random < 88 {
                Box::new(Grass::without_hedge(rand::gen_range(0, 7), y))
            } else if random < 94 {
                Box::new(Rail::empty(y))
            } else {
                Box::new(Pavement::empty(y))
            }
        } else {
            let random = rand::gen_range::<u8>(0, 100);
            if random < 60 {
                Box::new(Grass::without_hedge(rand::gen_range(0, 7), y))
            } else if random < 90 {
                Box::new(Rail::empty(y))
            } else {
                Box::new(Pavement::empty(y))
            }
        }
    }

    fn allow_movement(&self, x: i32) -> bool {
        (16..=WIDTH - 16).contains(&x)
    }

    fn check_collision(&self, x: i32) -> PlayerState {
        if self.collide(x, 0) {
            PlayerState::Splat(0)
        } else {
            PlayerState::Alive
        }
    }

    fn push(&self) -> i32 {
        0
    }

    fn sound(&self) -> Option<RowSound> {
        Some(RowSound::Traffic)
    }
}

impl ActiveRow for Road {
    fn build_child(dx: i32, position: Position) -> Child {
        Child::Car(Car::new(dx, position))
    }
}

impl Road {
    const CAR_SOUNDS: &'static [TrafficSound] = &[
        TrafficSound {
            y_offset: -ROW_HEIGHT,
            sound: CarSound::Zoom,
        },
        TrafficSound {
            y_offset: 0,
            sound: CarSound::Honk,
        },
        TrafficSound {
            y_offset: ROW_HEIGHT,
            sound: CarSound::Zoom,
        },
    ];
    const DXS: &'static [i32] = &[-5, -4, -3, -2, -1, 1, 2, 3, 4, 5];

    pub fn new(previous_dx: i32, index: i32, y: i32) -> Self {
        // Populate the row with child objects (cars or logs). Without this, the row would initially be empty.
        let dx = **Self::DXS
            .iter()
            .filter(|&dx| *dx != previous_dx)
            .collect::<Vec<&i32>>()
            .choose()
            .unwrap();
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
