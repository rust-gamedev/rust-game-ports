use crate::prelude::*;

pub fn movement(
    mut commands: Commands,
    mut move_events: EventReader<WantsToMove>,
    query: Query<(Entity, Option<&FieldOfView>, Option<&Player>)>,
    (map, mut camera): (Res<Map>, ResMut<Camera>),
) {
    for &WantsToMove {
        entity,
        destination,
    } in move_events.iter()
    {
        if map.can_enter_tile(destination) {
            commands.entity(entity).insert(PointC(destination));

            if let Ok((entity, fov, player)) = query.get(entity) {
                if let Some(fov) = fov {
                    commands.entity(entity).insert(fov.clone_dirty());
                }

                if player.is_some() {
                    camera.on_player_move(destination);
                }
            }
        }
    }
}
