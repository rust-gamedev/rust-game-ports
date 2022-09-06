use crate::{
    actor::Actor, bunner::Bunner, eagle::Eagle, grass::Grass, player_state::PlayerState,
    position::Position, resources::Resources, row::Row, row::RowSound, HEIGHT, ROW_HEIGHT,
};
use macroquad::{
    audio::{play_sound, set_sound_volume, stop_sound, PlaySoundParams},
    prelude::{clear_background, collections::storage, KeyCode, BLACK},
    rand::gen_range,
};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Default)]
pub struct Game {
    pub bunner: Option<Bunner>,
    pub scroll_pos: i32,
    eagle: Option<Eagle>,
    rows: Vec<Box<dyn Row>>,
    looped_sounds: HashSet<RowSound>,
}

impl Game {
    pub fn new(bunner: Option<Bunner>) -> Self {
        Self {
            bunner,
            scroll_pos: -HEIGHT,
            eagle: None,
            rows: vec![Box::new(Grass::without_hedge(0, 0))],
            looped_sounds: HashSet::new(),
        }
    }

    pub fn update(&mut self, input_queue: VecDeque<KeyCode>) {
        if let Some(bunner) = &self.bunner {
            // Scroll faster if the player is close to the top of the screen. Limit scroll speed to
            // between 1 and 3 pixels per frame.
            self.scroll_pos -=
                1.max(3.min(self.scroll_pos + HEIGHT - bunner.position.y) / (HEIGHT / 4));
        } else {
            self.scroll_pos -= 1;
        }

        // Remove rows that have scrolled past the bottom of the screen.
        let scroll_pos = self.scroll_pos;
        self.rows
            .retain(|row| row.y() < (scroll_pos + HEIGHT + ROW_HEIGHT * 2));

        // Add rows
        while let Some(last_row) = self.rows.last() {
            if last_row.y() > self.scroll_pos + ROW_HEIGHT {
                let new_row = last_row.next();
                self.rows.push(new_row)
            } else {
                break;
            }
        }

        for row in self.rows.iter_mut() {
            row.update(
                self.scroll_pos,
                self.bunner.as_ref().map(|bunner| bunner.position),
            );
        }
        if let Some(bunner) = self.bunner.as_mut() {
            bunner.update(self.scroll_pos, &mut self.rows, input_queue);
            if let PlayerState::Eagle(x) = bunner.state {
                self.eagle
                    .get_or_insert_with(|| Eagle::new(Position::new(x, self.scroll_pos)));
            };
        }
        if let Some(eagle) = self.eagle.as_mut() {
            eagle.update();
        }

        if let Some(bunner) = &self.bunner {
            let mut sounds: HashMap<RowSound, f32> = HashMap::new();
            for row in self.rows.iter() {
                if let Some(sound_name) = row.sound() {
                    let volume = sounds.get(&sound_name).unwrap_or(&0.)
                        + 16.0 / 16.0_f32.max((row.y() - bunner.position.y).abs() as f32);
                    sounds.insert(sound_name, volume);
                }
            }
            for (sound, volume) in sounds.drain() {
                self.loop_sound(sound, volume - 0.2);
            }
        }
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        for row in self.rows.iter().rev() {
            row.draw(0, -self.scroll_pos);
        }
        if let Some(bunner) = &self.bunner {
            bunner.draw(0, -self.scroll_pos);
        }
        if let Some(eagle) = &self.eagle {
            eagle.draw(0, -self.scroll_pos);
        }
    }

    pub fn game_over(&self) -> bool {
        if let Some(bunner) = &self.bunner {
            bunner.state != PlayerState::Alive && bunner.timer < 0
        } else {
            false
        }
    }

    pub fn score(&self) -> u32 {
        if let Some(bunner) = &self.bunner {
            0.max((-320 - bunner.min_y as i32) / 40) as u32
        } else {
            0
        }
    }

    fn loop_sound(&mut self, row_sound: RowSound, volume: f32) {
        let sound = match row_sound {
            RowSound::River => *storage::get::<Resources>()
                .river_sounds
                .get(gen_range::<usize>(0, 2))
                .unwrap(),
            RowSound::Traffic => *storage::get::<Resources>()
                .traffic_sounds
                .get(gen_range::<usize>(0, 3))
                .unwrap(),
        };
        if volume > 0. && self.looped_sounds.insert(row_sound) {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: true,
                    volume,
                },
            );
        }

        if self.looped_sounds.contains(&row_sound) {
            if volume > 0. {
                set_sound_volume(sound, volume);
            } else {
                stop_sound(sound);
                self.looped_sounds.remove(&row_sound);
            }
        }
    }
}
