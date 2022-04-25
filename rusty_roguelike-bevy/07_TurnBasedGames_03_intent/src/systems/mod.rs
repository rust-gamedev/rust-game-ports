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
    Render,
    PlayerMovement,
    Collision,
    RandomMove,
    MonsterMovement,
    RenderAndEndTurn,
}

pub fn build_system_sets(app: &mut App) {
    use SystemSets::*;
    use TurnState::*;

    // In Bevy, we group the systems, label them, and establish the temporal associations; this is somewhat
    // harder to visualize, but it's more structured.
    // An advantage in this case is that a common group (RenderAndEndTurn) is deduplicated.
    // A disadvantage is that we can't model complex associations.

    // State: AwaitingInput

    app.add_system_set(
        SystemSet::on_update(AwaitingInput)
            .label(PlayerInput)
            .with_system(player_input::player_input),
    )
    .add_system_set(
        SystemSet::new()
            .label(Render)
            .after(PlayerInput)
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render),
    );

    // State: PlayerTurn (without rendering)

    app.add_system_set(
        SystemSet::on_update(PlayerTurn)
            .label(PlayerMovement)
            .with_system(movement::movement),
    )
    .add_system_set(
        SystemSet::new()
            .label(Collision)
            .with_system(collisions::collisions),
    );

    // State: MonsterTurn (without rendering)

    app.add_system_set(
        SystemSet::on_update(MonsterTurn)
            .label(RandomMove)
            .with_system(random_move::random_move),
    )
    .add_system_set(
        SystemSet::new()
            .label(MonsterMovement)
            .after(RandomMove)
            .with_system(movement::movement),
    );

    // Rendering (shared between PlayerTurn and MonsterTurn)

    app.add_system_set(
        SystemSet::new()
            .label(RenderAndEndTurn)
            .after(Collision)
            .after(MonsterMovement)
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render)
            .with_system(end_turn::end_turn),
    );
}
