use crate::prelude::*;

pub struct Controls {
    key_up: VirtualKeyCode,
    key_down: VirtualKeyCode,
    key_left: VirtualKeyCode,
    key_right: VirtualKeyCode,
    key_shoot: VirtualKeyCode,
}

impl Controls {
    pub fn new(player_num: u8) -> Self {
        use VirtualKeyCode::*;

        if player_num == 0 {
            Self {
                key_up: Up,
                key_down: Down,
                key_left: Left,
                key_right: Right,
                key_shoot: Space,
            }
        } else {
            Self {
                key_up: W,
                key_down: A,
                key_left: S,
                key_right: D,
                key_shoot: LShift,
            }
        }
    }

    // We could, in theory, store an &input reference in the struct, however, that would pollute the
    // types with lifetimes.
    //
    // Can't name `move`, which is a reserved keyword.
    //
    pub fn move_player(&self, speed: f32, input: &InputController) -> Vector2<f32> {
        let (mut dx, mut dy) = (0., 0.);

        if input.is_key_pressed(self.key_left) {
            dx = -1.;
        } else if input.is_key_pressed(self.key_right) {
            dx = 1.;
        }
        if input.is_key_pressed(self.key_up) {
            dy = -1.;
        } else if input.is_key_pressed(self.key_down) {
            dy = 1.;
        }

        Vector2::new(dx, dy) * speed
    }

    pub fn shoot(&self, input: &InputController) -> bool {
        input.is_key_just_pressed(self.key_shoot)
    }
}
