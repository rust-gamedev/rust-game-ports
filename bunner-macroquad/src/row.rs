use crate::{child::Child, player_state::PlayerState, position::Position, WIDTH};
use macroquad::{
    color::colors::WHITE,
    texture::{draw_texture, Texture2D},
};

pub trait Row {
    fn y(&self) -> i32;
    fn children(&self) -> &[Child];
    fn children_mut(&mut self) -> &mut Vec<Child>;

    fn update(&mut self, _scroll_pos: i32, _bunner_pos: Option<Position>) {
        self.update_children();
    }

    fn update_children(&mut self) {
        for child in self.children_mut().iter_mut() {
            child.update();
        }
    }

    fn image(&self) -> Texture2D;

    fn draw(&self, offset_x: i32, offset_y: i32) {
        let (x, y) = self.draw_row(offset_x, offset_y);
        self.draw_children(x, y);
    }

    fn draw_row(&self, offset_x: i32, offset_y: i32) -> (i32, i32) {
        let image = self.image();
        let x = offset_x;
        let y = self.y() + offset_y;
        draw_texture(image, x as f32, y as f32 - image.height(), WHITE);
        (x, y)
    }

    fn draw_children(&self, offset_x: i32, offset_y: i32) {
        for child in self.children() {
            child.draw(offset_x, offset_y);
        }
    }

    fn play_sound(&self);

    fn next(&self) -> Box<dyn Row>;

    fn check_collision(&self, _x: i32) -> PlayerState {
        PlayerState::Alive
    }

    fn allow_movement(&self, x: i32) -> bool {
        (16..=WIDTH - 16).contains(&x)
    }

    fn collide(&self, x: i32, margin: i32) -> bool {
        for child in self.children().iter() {
            if x >= child.x() - (child.width() / 2) - margin
                && x < child.x() + (child.width() / 2) + margin
            {
                return true;
            }
        }
        false
    }

    fn push(&self) -> i32 {
        0
    }

    fn sound(&self) -> Option<RowSound> {
        None
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum RowSound {
    Traffic,
    River,
}
