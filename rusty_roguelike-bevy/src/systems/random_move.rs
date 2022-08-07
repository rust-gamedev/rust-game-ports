use crate::prelude::*;

pub fn random_move(
    mut move_events: EventWriter<WantsToMove>,
    mut attack_events: EventWriter<WantsToAttack>,
    movers: Query<(Entity, &PointC), With<MovingRandomly>>,
    positions: Query<(Entity, &PointC), With<Health>>,
    player_query: Query<Entity, With<Player>>,
) {
    movers.iter().for_each(|(entity, pos)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + pos.0;

        // This variable name is a bit misleading, as enemies don't attack each other.
        let mut attacked = false;

        // Something important to be aware of is that this logic doesn't prevent multiple enemies to
        // move to the same position if it's empty, as they move only in a subsequent stage. Solving
        // this issue needs to consider edge cases, like an enemy in a corridor surrounded by two enemies
        // intending to move around it. This can be intended or not; some users discussed it - see source
        // project [issue tracker](https://github.com/thebracket/HandsOnRust/pull/1)).
        //
        for (victim, target_pos) in positions.iter() {
            if target_pos.0 == destination {
                if player_query.get(victim).is_ok() {
                    attack_events.send(WantsToAttack {
                        attacker: entity,
                        victim: victim,
                    });
                }
                attacked = true;
            }
        }

        if !attacked {
            move_events.send(WantsToMove {
                entity,
                destination,
            })
        }
    })
}
