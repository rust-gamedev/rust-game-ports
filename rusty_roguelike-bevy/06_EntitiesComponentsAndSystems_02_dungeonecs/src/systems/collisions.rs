use crate::prelude::*;

pub fn collisions(
    mut commands: Commands,
    // Note that we can use two independent queries both accessing PointC, because they have compatible
    // access type (immutable); if they were incompatible, we would have needed ParamSet.
    player_query: Query<&PointC, With<Player>>,
    enemies_query: Query<(Entity, &PointC), With<Enemy>>,
) {
    // We can use Query#single() when it's guaranteed that an entity exists.
    let player_pos = player_query.single().0;

    for (entity, pos) in enemies_query.iter() {
        if pos.0 == player_pos {
            commands.entity(entity).despawn()
        }
    }
}
