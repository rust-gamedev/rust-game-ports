use crate::prelude::*;

pub fn player_input(
    mut commands: Commands,
    mut move_events: EventWriter<WantsToMove>,
    mut attack_events: EventWriter<WantsToAttack>,
    player_query: Query<(Entity, &PointC), With<Player>>,
    enemies_query: Query<(Entity, &PointC), With<Enemy>>,
    key: Option<Res<VirtualKeyCode>>,
) {
    if let Some(key) = key.as_deref() {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        // From this iteration, the source project assumes that there is always a player entity, so
        // we can use the single() API, which makes this assumption.
        //
        let (player_entity, player_pos) = player_query.single();
        let destination = player_pos.0 + delta;

        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            // The Iterator#any API could also be conveniently used, although it's often assumed not
            // to have side effects, which is not the case here.
            for (entity, pos) in enemies_query.iter() {
                if pos.0 == destination {
                    hit_something = true;

                    attack_events.send(WantsToAttack {
                        attacker: player_entity,
                        victim: entity,
                    });
                }
            }

            if !hit_something {
                move_events.send(WantsToMove {
                    entity: player_entity,
                    destination,
                });
            }
        }

        commands.insert_resource(NextState(TurnState::PlayerTurn));

        // WATCH OUT!! If they key resource is not removed, multiple keypresses will be detected over
        // the same frame. This is because a system (set) may run multiple times over a frame, due to
        // state circularity.
        // By removing they key, once this system is run a second time, no keypress is detected, and
        // the circle stops.
        // This may not be needed if there is one game step per frame, but it's good practice to keep
        // in mind.
        //
        commands.remove_resource::<VirtualKeyCode>();
    }
}
