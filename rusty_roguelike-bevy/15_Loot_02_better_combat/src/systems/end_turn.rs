use crate::prelude::*;

pub fn end_turn(
    mut commands: Commands,
    player_query: Query<(&Health, &PointC), With<Player>>,
    amulet_query: Query<&PointC, With<AmuletOfYala>>,
    turn_state: Res<CurrentState<TurnState>>,
    map: Res<Map>,
) {
    let (player_hp, player_pos) = player_query.single();
    let mut new_state = match turn_state.0 {
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        // In the source project, AwaitingInput and GameOver return (themselves), however, they're actually
        // unreachable cases, because this system is not run in such states, and the change to their next
        // states is performed elsewhere.
        _ => unreachable!(),
    };

    let amulet_default = PointC(Point::new(-1, -1));
    let amulet_pos = amulet_query.get_single().unwrap_or(&amulet_default);

    if player_hp.current < 1 {
        new_state = TurnState::GameOver;
    }
    if player_pos.0 == amulet_pos.0 {
        new_state = TurnState::Victory;
    }
    let idx = map.point2d_to_index(player_pos.0);
    if map.tiles[idx] == TileType::Exit {
        new_state = TurnState::NextLevel;
    }

    commands.insert_resource(NextState(new_state));
}
