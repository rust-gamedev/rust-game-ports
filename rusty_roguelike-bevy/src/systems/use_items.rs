use crate::prelude::*;

pub fn use_items(
    mut commands: Commands,
    mut activate_item_events: EventReader<ActivateItem>,
    items_query: Query<(Option<&ProvidesHealing>, Option<&ProvidesDungeonMap>)>,
    mut health_query: Query<&mut Health>,
    mut map: ResMut<Map>,
) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    for activate in activate_item_events.iter() {
        if let Ok((healing, mapper)) = items_query.get(activate.item) {
            if let Some(healing) = healing {
                healing_to_apply.push((activate.used_by, healing.amount));
            }

            if mapper.is_some() {
                map.revealed_tiles.iter_mut().for_each(|t| *t = true);
            }
        }
        commands.entity(activate.item).despawn();
    }

    for heal in &healing_to_apply {
        if let Ok(mut health) = health_query.get_mut(heal.0) {
            health.current = i32::min(health.max, health.current + heal.1);
        }
    }
}
