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
    Render,
    PlayerPhonyMove, // phony (does nothing); see comment below
    MonsterMove,
    Collisions,
    RenderAndEndTurn,
}

pub fn build_system_sets(app: &mut App) {
    use SystemSets::*;
    use TurnState::*;

    // In Bevy, we group the systems, label them, and establish the temporal associations; this is somewhat
    // harder to visualize, but it's more structured.
    // An advantage in this case is that a common group (RenderAndEndTurn) is deduplicated.
    // A disadvantage is that we can't model complex associations.

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

    // We need the phony system set in order to share the common collisions set (and after, then render
    // and end). Without it, player collisions runs on PlayerTurn, but for this reason, another collisions
    // set needs to be placed after MonsterMove (since the player collision set can't be used in this
    // path).
    // In other words, the problem is that we can't model "run system set after (state update or other
    // system set)", so we assign a phony system set to the state update in order to model the condition,
    // which becomes (run system set after (system set A or system set B)).

    app.add_system_set(
        SystemSet::on_update(PlayerTurn).label(PlayerPhonyMove), // do nothing
    )
    .add_system_set(
        SystemSet::on_update(MonsterTurn)
            .label(MonsterMove)
            .with_system(random_move::random_move),
    );

    app.add_system_set(
        SystemSet::new()
            .label(Collisions)
            .after(PlayerPhonyMove)
            .after(MonsterMove)
            .with_system(collisions::collisions),
    )
    .add_system_set(
        SystemSet::new()
            .label(RenderAndEndTurn)
            .after(Collisions)
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render)
            .with_system(end_turn::end_turn),
    );
}
