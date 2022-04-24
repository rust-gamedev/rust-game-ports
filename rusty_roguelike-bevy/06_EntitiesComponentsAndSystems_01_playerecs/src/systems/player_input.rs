use crate::prelude::*;

pub fn player_input(
    mut query: Query<(&Player, &mut PointC)>, //(1) (2)
    (map, key, mut camera): (Res<Map>, Option<Res<VirtualKeyCode>>, ResMut<crate::Camera>),
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
            for (_player, mut pos) in query.iter_mut() {
                //(3)
                let destination = pos.0 + delta;
                if map.can_enter_tile(destination) {
                    pos.0 = destination;
                    camera.on_player_move(destination);
                }
            }
        }
    }
}
