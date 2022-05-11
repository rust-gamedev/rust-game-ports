use crate::prelude::*;

pub fn player_input(
    mut commands: Commands,
    mut move_events: EventWriter<WantsToMove>,
    mut attack_events: EventWriter<WantsToAttack>,
    active_item_events: EventWriter<ActivateItem>,
    player_query: Query<(Entity, &PointC), With<Player>>,
    level_items_query: Query<(Entity, &PointC), With<Item>>,
    carried_items_query: Query<(Entity, &Carried), With<Item>>,
    weapons_query: Query<&Weapon>,
    carried_weapons_query: Query<(Entity, &Carried), With<Weapon>>,
    enemies_query: Query<(Entity, &PointC), With<Enemy>>,
    key: Option<Res<VirtualKeyCode>>,
) {
    let (player_entity, player_pos) = player_query.single();

    if let Some(key) = key.as_deref() {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::G => {
                for (entity, item_pos) in level_items_query.iter() {
                    if item_pos.0 == player_pos.0 {
                        commands.entity(entity).remove::<PointC>();
                        commands.entity(entity).insert(Carried(player_entity));

                        if weapons_query.get(entity).is_ok() {
                            for (e, c) in carried_weapons_query.iter() {
                                if c.0 == player_entity {
                                    commands.entity(e).despawn();
                                }
                            }
                        }
                    }
                }
                Point::new(0, 0)
            }
            VirtualKeyCode::Key1 => {
                use_item(0, player_entity, active_item_events, carried_items_query)
            }
            VirtualKeyCode::Key2 => {
                use_item(1, player_entity, active_item_events, carried_items_query)
            }
            VirtualKeyCode::Key3 => {
                use_item(2, player_entity, active_item_events, carried_items_query)
            }
            VirtualKeyCode::Key4 => {
                use_item(3, player_entity, active_item_events, carried_items_query)
            }
            VirtualKeyCode::Key5 => {
                use_item(4, player_entity, active_item_events, carried_items_query)
            }
            VirtualKeyCode::Key6 => {
                use_item(5, player_entity, active_item_events, carried_items_query)
            }
            VirtualKeyCode::Key7 => {
                use_item(6, player_entity, active_item_events, carried_items_query)
            }
            VirtualKeyCode::Key8 => {
                use_item(7, player_entity, active_item_events, carried_items_query)
            }
            VirtualKeyCode::Key9 => {
                use_item(8, player_entity, active_item_events, carried_items_query)
            }
            _ => Point::new(0, 0),
        };

        let destination = player_pos.0 + delta;

        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            // The Iterator#any API could also be conveniently used, although it's often assumed not
            // to have side effects, which is not the case here.
            for (entity, pos) in enemies_query.iter() {
                if pos.0 == destination {
                    hit_something = true;

                    attack_events.send(WantsToAttack {
                        attacker: player_entity,
                        victim: entity,
                    });
                }
            }

            if !hit_something {
                move_events.send(WantsToMove {
                    entity: player_entity,
                    destination,
                });
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

fn use_item(
    n: usize,
    player_entity: Entity,
    mut active_item_events: EventWriter<ActivateItem>,
    items: Query<(Entity, &Carried), With<Item>>,
) -> Point {
    let item_entity = items
        .iter()
        .filter(|(_, carried)| carried.0 == player_entity)
        .enumerate()
        .filter(|(item_count, (_, _))| *item_count == n)
        .find_map(|(_, (item_entity, _))| Some(item_entity));

    if let Some(item_entity) = item_entity {
        active_item_events.send(ActivateItem {
            used_by: player_entity,
            item: item_entity,
        });
    }

    Point::zero()
}
