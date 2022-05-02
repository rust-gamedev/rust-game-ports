use crate::prelude::*;

pub fn movement(
    mut commands: Commands,
    mut move_events: EventReader<WantsToMove>,
    query: Query<&Player>,
    (map, mut camera): (Res<Map>, ResMut<Camera>),
) {
    for &WantsToMove {
        entity,
        destination,
    } in move_events.iter()
    {
        if map.can_enter_tile(destination) {
            commands.entity(entity).insert(PointC(destination));

            // An alternative design is to split the messages in two: for player and for enemies.
            //
            if query.get(entity).is_ok() {
                camera.on_player_move(destination);
            }
        }
    }
}
