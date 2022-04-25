use crate::prelude::*;

pub fn player_input(
    mut move_events: EventWriter<WantsToMove>,
    mut query: Query<(Entity, &PointC)>, //(1) (2)
    (key, mut turn_state): (Option<Res<VirtualKeyCode>>, ResMut<State<TurnState>>),
) {
    use TurnState::PlayerTurn;

    if let Some(key) = key.as_deref() {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            for (entity, pos) in query.iter_mut() {
                //(3)
                let destination = pos.0 + delta;
                move_events.send(WantsToMove {
                    entity,
                    destination: destination,
                });
            }
        }
        turn_state.set(PlayerTurn).unwrap();
    }
}
