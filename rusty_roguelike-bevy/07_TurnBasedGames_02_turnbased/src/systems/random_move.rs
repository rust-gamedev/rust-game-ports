use crate::prelude::*;

pub fn random_move(mut movers: Query<&mut PointC, With<MovingRandomly>>, map: Res<Map>) {
    movers.iter_mut().for_each(|mut pos| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + pos.0;

        if map.can_enter_tile(destination) {
            pos.0 = destination;
        }
    })
}
