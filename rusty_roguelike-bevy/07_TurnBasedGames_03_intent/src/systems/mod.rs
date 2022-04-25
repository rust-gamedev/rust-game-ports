use crate::prelude::*;

mod collisions;
mod end_turn;
mod entity_render;
mod map_render;
mod movement;
mod player_input;
mod random_move;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum SystemSets {
    PlayerInput,
    Movement,
    Collision,
    RandomMove,
}

pub fn build_system_sets(app: &mut App) {
    use SystemSets::*;
    use TurnState::*;

    // In Bevy, we group the systems, label them, and establish the temporal associations; this is somewhat
    // harder to visualize, but it's more flexible.

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
            .label(Movement)
            .with_system(movement::movement),
    )
    .add_system_set(
        SystemSet::on_update(PlayerTurn)
            .label(Collision)
            .after(Movement)
            .with_system(collisions::collisions),
    )
    .add_system_set(
        SystemSet::on_update(PlayerTurn)
            .after(Collision)
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render)
            .with_system(end_turn::end_turn),
    );

    app.add_system_set(
        SystemSet::on_update(MonsterTurn)
            .label(RandomMove)
            .with_system(random_move::random_move),
    )
    .add_system_set(
        SystemSet::on_update(MonsterTurn)
            .label(Movement)
            .after(RandomMove)
            .with_system(movement::movement),
    )
    .add_system_set(
        SystemSet::on_update(MonsterTurn)
            .after(Movement)
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render)
            .with_system(end_turn::end_turn),
    );
}
