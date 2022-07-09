use crate::prelude::*;

pub fn player_input(
    mut commands: Commands,
    mut move_events: EventWriter<WantsToMove>,
    player_query: Query<(Entity, &PointC), With<Player>>, //(1) (2)
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

        if delta.x != 0 || delta.y != 0 {
            // In the source project, the query assumes multiple query entities; here we use the single
            // entity API, still allowing the possibility that there is no player.
            if let Ok((entity, pos)) = player_query.get_single() {
                //(3)
                let destination = pos.0 + delta;
                move_events.send(WantsToMove {
                    entity,
                    destination: destination,
                });
            }
        }
        commands.insert_resource(TurnState::PlayerTurn);

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
