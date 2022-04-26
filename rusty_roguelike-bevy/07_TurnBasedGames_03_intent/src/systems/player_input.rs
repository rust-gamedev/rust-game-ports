use crate::prelude::*;

pub fn player_input(
    mut commands: Commands,
    mut move_events: EventWriter<WantsToMove>,
    mut query: Query<(Entity, &PointC), With<Player>>, //(1) (2)
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
            for (entity, pos) in query.iter_mut() {
                //(3)
                let destination = pos.0 + delta;
                move_events.send(WantsToMove {
                    entity,
                    destination: destination,
                });
            }
        }
        commands.insert_resource(NextState(GameStep::MovePlayer));

        // WATCH OUT!! If they key resource is not removed, multiple keypresses will be detected over
        // the same frame. This is because a system (set) may run multiple times over a frame, due to
        // state circularity.
        // By removing they key, once this system is run a second time, no keypress is detected, and
        // the circle stops.
        //
        commands.remove_resource::<VirtualKeyCode>();
    }
}
