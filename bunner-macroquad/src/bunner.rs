use crate::{
    child::Child, player_direction::PlayerDirection, player_state::PlayerState, position::Position,
    resources::Resources, row::Row, splat::Splat, HEIGHT, WIDTH,
};
use macroquad::{
    audio::play_sound_once,
    color::colors::WHITE,
    experimental::collections::storage,
    input::KeyCode,
    texture::{draw_texture, Texture2D},
};
use std::collections::VecDeque;

pub struct Bunner {
    pub state: PlayerState,
    pub timer: i32,
    pub position: Position,
    pub min_y: i32,
    direction: PlayerDirection,
    input_queue: VecDeque<PlayerDirection>,
    image: Texture2D,
}

impl Bunner {
    const MOVE_DISTANCE: i32 = 10;

    pub fn new(position: Position) -> Self {
        Self {
            state: PlayerState::Alive,
            timer: 0,
            position,
            min_y: position.y,
            direction: PlayerDirection::Down,
            input_queue: VecDeque::new(),
            image: storage::get::<Resources>().blank_texture,
        }
    }

    pub fn update(
        &mut self,
        scroll_pos: i32,
        rows: &mut [Box<dyn Row>],
        input_queue: VecDeque<KeyCode>,
    ) {
        self.input_queue.append(
            &mut input_queue
                .iter()
                .filter_map(|key_code| match key_code {
                    KeyCode::Up => Some(PlayerDirection::Up),
                    KeyCode::Right => Some(PlayerDirection::Right),
                    KeyCode::Down => Some(PlayerDirection::Down),
                    KeyCode::Left => Some(PlayerDirection::Left),
                    _ => None,
                })
                .collect::<VecDeque<PlayerDirection>>(),
        );

        match self.state {
            PlayerState::Alive => {
                // While the player is alive, the timer variable is used for movement.
                // If it's zero, the player is on the ground. If it's above zero,
                // they're currently jumping to a new location.

                // Are we on the ground, and are there inputs to process?
                if self.timer == 0 {
                    // Take the next input off the queue and process it
                    let direction = self.input_queue.pop_front();
                    self.handle_input(direction, rows);
                }

                let mut land = false;
                if self.timer > 0 {
                    // Apply movement
                    self.position.x += Self::dx(&self.direction);
                    self.position.y += Self::dy(&self.direction);
                    self.timer -= 1;
                    // If timer reaches zero, we've just landed
                    land = self.timer == 0;
                }

                if let Some(current_row) = rows.iter_mut().find(|row| row.y() == self.position.y) {
                    self.state = current_row.check_collision(self.position.x);
                    match self.state {
                        PlayerState::Alive => {
                            self.position.x += current_row.push();
                            if land {
                                current_row.play_sound();
                            }
                        }
                        PlayerState::Splat(y_offset) => {
                            self.position.y += y_offset;
                            self.timer = 100;
                            current_row.children_mut().insert(
                                0,
                                Child::Splat(Splat::new(
                                    self.direction,
                                    Position::new(self.position.x, y_offset),
                                )),
                            );
                            play_sound_once(storage::get::<Resources>().splat_sound);
                        }
                        PlayerState::Splash => {
                            play_sound_once(storage::get::<Resources>().splash_sound);
                            self.timer = 100;
                        }
                        _ => self.timer = 100,
                    }
                } else if self.position.y > scroll_pos + HEIGHT + 80 {
                    self.state = PlayerState::Eagle(self.position.x);
                    self.timer = 150;
                    play_sound_once(storage::get::<Resources>().eagle_sound);
                }

                // Limit x position
                self.position.x = 16.max((WIDTH - 16).min(self.position.x));
            }
            _ => {
                // Not alive - timer now counts down prior to game over screen
                self.timer -= 1
            }
        }

        // Keep track of the furthest we've got in the level
        self.min_y = self.min_y.min(self.position.y);

        // Choose sprite image
        self.image = match self.state {
            PlayerState::Alive => {
                if self.timer > 0 {
                    *storage::get::<Resources>()
                        .jump_textures
                        .get(self.direction as usize)
                        .unwrap()
                } else {
                    *storage::get::<Resources>()
                        .sit_textures
                        .get(self.direction as usize)
                        .unwrap()
                }
            }
            PlayerState::Splash if self.timer > 84 => {
                // Display appropriate 'splash' animation frame. Note that we use a different technique to display the
                // 'splat' image - see: comments earlier in this method. The reason two different techniques are used is
                // that the splash image should be drawn on top of other objects, whereas the splat image must be drawn
                // underneath other objects. Since the player is always drawn on top of other objects, changing the player
                // sprite is a suitable method of displaying the splash image.
                let splash_index = ((100 - self.timer) / 2) as usize;
                *storage::get::<Resources>()
                    .splash_textures
                    .get(splash_index)
                    .unwrap()
            }
            _ => storage::get::<Resources>().blank_texture,
        };
    }

    pub fn draw(&self, offset_x: i32, offset_y: i32) {
        let x = (self.position.x + offset_x) as f32 - self.image.width() / 2.;
        let y = (self.position.y + offset_y) as f32 - self.image.height();
        draw_texture(self.image, x, y, WHITE);
    }

    pub fn handle_input(&mut self, direction: Option<PlayerDirection>, rows: &[Box<dyn Row>]) {
        if let Some(direction) = direction {
            for row in rows.iter() {
                if row.y() == self.position.y + Self::MOVE_DISTANCE * Self::dy(&direction) {
                    if row.allow_movement(
                        self.position.x + Self::MOVE_DISTANCE * Self::dx(&direction),
                    ) {
                        self.direction = direction;
                        self.timer = Bunner::MOVE_DISTANCE;
                        play_sound_once(storage::get::<Resources>().jump_sound);
                    }
                    break;
                }
            }
        }
    }

    fn dx(direction: &PlayerDirection) -> i32 {
        match direction {
            PlayerDirection::Up => 0,
            PlayerDirection::Right => 4,
            PlayerDirection::Down => 0,
            PlayerDirection::Left => -4,
        }
    }

    fn dy(direction: &PlayerDirection) -> i32 {
        match direction {
            PlayerDirection::Up => -4,
            PlayerDirection::Right => 0,
            PlayerDirection::Down => 4,
            PlayerDirection::Left => 0,
        }
    }
}
