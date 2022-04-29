use crate::prelude::*;

pub fn player_input(
    mut commands: Commands,
    players: Query<(&Player, &mut PointC)>, //(1) (2)
    (map, key, mut camera): (Res<Map>, Option<Res<VirtualKeyCode>>, ResMut<Camera>),
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
            for (_player, mut pos) in players.iter() {
                //(3)
                let destination = pos.0 + delta;
                if map.can_enter_tile(destination) {
                    pos.0 = destination;
                    camera.on_player_move(destination);
                }
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
