use crate::prelude::*;

mod collisions;
mod end_turn;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum SystemSets {
    PlayerInput,
    MonsterMove,
    // A label can be associated to multiple systems, so we can use it for systems sets running during
    // different states, as long as we filter system sets by state (see below).
    Collisions,
}

pub fn build_system_sets(app: &mut App) {
    use SystemSets::*;
    use TurnState::*;

    // In Bevy, we group the systems, label them, and establish the temporal associations; this is somewhat
    // harder to visualize, but it's more flexible.
    //
    // WATCH OUT! `before()`/`after()` are not execution constraints; they only determine ordering! The
    // state stage callback (in this case, `SystemSet::on_update`) must be specified, otherwise, the
    // given system will always run.

    app.add_system_set(
        SystemSet::on_update(AwaitingInput)
            .label(PlayerInput)
            .with_system(player_input::player_input),
    )
    .add_system_set(
        SystemSet::on_update(AwaitingInput)
            .after(PlayerInput)
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render),
    );

    app.add_system_set(
        SystemSet::on_update(PlayerTurn)
            .label(Collisions)
            .with_system(collisions::collisions),
    )
    .add_system_set(
        SystemSet::on_update(PlayerTurn)
            .after(Collisions)
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render)
            .with_system(end_turn::end_turn),
    );

    app.add_system_set(
        SystemSet::on_update(MonsterTurn)
            .label(MonsterMove)
            .with_system(random_move::random_move),
    )
    .add_system_set(
        SystemSet::on_update(MonsterTurn)
            .label(Collisions)
            .after(MonsterMove)
            .with_system(collisions::collisions),
    )
    .add_system_set(
        SystemSet::on_update(MonsterTurn)
            .after(Collisions)
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render)
            .with_system(end_turn::end_turn),
    );
}
