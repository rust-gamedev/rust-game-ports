use crate::prelude::*;

pub fn chasing(
    mut move_events: EventWriter<WantsToMove>,
    mut attack_events: EventWriter<WantsToAttack>,
    movers: Query<(Entity, &PointC, &FieldOfView), With<ChasingPlayer>>,
    positions: Query<(Entity, &PointC), With<Health>>,
    player: Query<&PointC, With<Player>>,
    map: Res<Map>,
) {
    let player_pos = player.single().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &search_targets,
        map.as_ref(),
        1024.0,
    );

    for (entity, pos, fov) in movers.iter() {
        if !fov.visible_tiles.contains(&player_pos) {
            continue;
        }

        let idx = map_idx(pos.0.x, pos.0.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map.as_ref()) {
            let distance = DistanceAlg::Pythagoras.distance2d(pos.0, player_pos);
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                player_pos
            };

            let mut attacked = false;
            for (victim, target_pos) in positions.iter() {
                if target_pos.0 == destination {
                    if player.get(victim).is_ok() {
                        attack_events.send(WantsToAttack {
                            attacker: entity,
                            victim: victim,
                        });
                    }
                    attacked = true;
                }
            }

            if !attacked {
                move_events.send(WantsToMove {
                    entity: entity,
                    destination,
                });
            }
        }
    }
}
