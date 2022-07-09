use crate::prelude::*;

pub fn end_turn(
    mut commands: Commands,
    player_query: Query<(&Health, &PointC), With<Player>>,
    amulet_query: Query<&PointC, With<AmuletOfYala>>,
    turn_state: Res<TurnState>,
) {
    let (player_hp, player_pos) = player_query.single();
    let mut new_state = match *turn_state {
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        // In the source project, AwaitingInput and GameOver return (themselves), however, they're actually
        // unreachable cases, because this system is not run in such states, and the change to their next
        // states is performed elsewhere.
        _ => unreachable!(),
    };

    let amulet_pos = amulet_query.single();

    if player_hp.current < 1 {
        new_state = TurnState::GameOver;
    }
    if player_pos.0 == amulet_pos.0 {
        new_state = TurnState::Victory;
    }

    commands.insert_resource(new_state);
}
