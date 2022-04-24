use crate::prelude::*;

pub fn collisions(
    mut cmd: Commands,
    // Note that we can use two independent queries both accessing PointC, because they have compatible
    // access type (immutable); if they were incompatible, we would have needed ParamSet.
    player_query: Query<&PointC, With<Player>>,
    enemies_query: Query<(Entity, &PointC), With<Enemy>>,
) {
    // We can use Query#single() when it's guaranteed that an entity exists.
    let player_pos = player_query.single().0;

    // The source project often uses the pattern `filter().for_each()`; a direct iteration is simpler,
    // but we'll keep this refactoring for later.
    enemies_query
        .iter()
        .filter(|(_, pos)| pos.0 == player_pos)
        .for_each(|(entity, _)| cmd.entity(entity).despawn());
}
