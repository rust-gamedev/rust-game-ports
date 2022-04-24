use crate::prelude::*;

mod collisions;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum SystemSets {
    InputAndCollisions,
    RenderAndMove,
}

pub fn build_system_sets(app: &mut App) {
    use SystemSets::*;

    // In order to establish ordering, Bevy uses before()/after() APIs (and labelling), in place of
    // Legion's flush().
    app.add_system_set(
        SystemSet::new()
            .label(InputAndCollisions)
            .with_system(player_input::player_input)
            .with_system(collisions::collisions),
    )
    .add_system_set(
        SystemSet::new()
            .label(RenderAndMove)
            .after(InputAndCollisions)
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render)
            .with_system(random_move::random_move),
    );
}
