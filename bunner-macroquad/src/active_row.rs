use crate::{child::Child, position::Position, WIDTH};
use macroquad::rand;

pub trait ActiveRow {
    fn build_children(dx: i32) -> Vec<Child>
    where
        Self: Sized,
    {
        let mut children = Vec::new();
        let mut x = -WIDTH / 2 - 70;
        while x < WIDTH / 2 + 70 {
            x += rand::gen_range::<i32>(240, 481);
            let position = if dx > 0 {
                Position::new(WIDTH / 2 + x, 0)
            } else {
                Position::new(WIDTH / 2 - x, 0)
            };
            children.push(Self::build_child(dx, position));
        }
        children
    }

    fn build_child(dx: i32, position: Position) -> Child
    where
        Self: Sized;

    fn create_random_child(&self, dx: i32) -> Child
    where
        Self: Sized,
    {
        let pos = Position::new(if dx < 0 { WIDTH + 70 } else { -70 }, 0);
        Self::build_child(dx, pos)
    }

    fn random_interval(&self, dx: i32) -> f32
    where
        Self: Sized,
    {
        // 240 is minimum distance between the start of one child object and the start of the next, assuming its
        // speed is 1. If the speed is 2, they can occur twice as frequently without risk of overlapping with
        // each other. The maximum distance is double the minimum distance (1 + random value of 1)
        (1. + rand::gen_range::<f32>(0.0, 1.0)) * (240 / dx.abs()) as f32
    }
}
