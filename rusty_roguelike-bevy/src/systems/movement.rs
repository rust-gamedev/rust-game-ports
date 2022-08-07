use crate::prelude::*;

pub fn movement(
    mut commands: Commands,
    mut move_events: EventReader<WantsToMove>,
    query: Query<(Entity, &FieldOfView, Option<&Player>)>,
    (mut map, mut camera): (ResMut<Map>, ResMut<Camera>),
) {
    for &WantsToMove {
        entity,
        destination,
    } in move_events.iter()
    {
        if map.can_enter_tile(destination) {
            commands.entity(entity).insert(PointC(destination));

            if let Ok((entity, fov, player)) = query.get(entity) {
                // In Bevy, we don't need to test for Result<FieldOfView>, because the entity, if found,
                // will have a FieldOfView component, due to the query definition.
                commands.entity(entity).insert(fov.clone_dirty());

                if player.is_some() {
                    camera.on_player_move(destination);
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    });
                }
            }
        }
    }
}
