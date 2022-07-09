use crate::prelude::*;

pub fn end_turn(
    mut commands: Commands,
    player_query: Query<&Health, With<Player>>,
    turn_state: Res<TurnState>,
) {
    let mut new_state = match *turn_state {
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        // In the source project, AwaitingInput and GameOver return (themselves), however, they're actually
        // unreachable cases, because this system is not run in such states, and the change to their next
        // states is performed elsewhere.
        _ => unreachable!(),
    };

    if player_query.single().current < 1 {
        new_state = TurnState::GameOver;
    }

    commands.insert_resource(new_state);
}
