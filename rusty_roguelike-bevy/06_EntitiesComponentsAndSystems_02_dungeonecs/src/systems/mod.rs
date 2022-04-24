use crate::prelude::*;

mod collisions;
mod entity_render;
mod map_render;
mod player_input;

pub fn build_system_set() -> SystemSet {
    // At this project stage, system sets (Legion schedulers) are not differentiated, so we just use
    // a generic one.
    SystemSet::new()
        .with_system(player_input::player_input)
        .with_system(collisions::collisions)
        .with_system(map_render::map_render)
        .with_system(entity_render::entity_render)
}
