use crate::prelude::*;

pub fn random_move(
    mut move_events: EventWriter<WantsToMove>,
    movers: Query<(Entity, &PointC), With<MovingRandomly>>,
) {
    movers.iter().for_each(|(entity, pos)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + pos.0;

        move_events.send(WantsToMove {
            entity,
            destination: destination,
        });
    })
}
