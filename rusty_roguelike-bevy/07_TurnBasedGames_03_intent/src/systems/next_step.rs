use crate::prelude::*;

// This is end_turn() in the source project; here, it's next_step(), because the semantics are different.
//
pub fn next_step(mut commands: Commands, turn_state: Res<CurrentState<GameStep>>) {
    use GameStep::*;

    let new_state = match turn_state.0 {
        // In the source project, AwaitingInput returns AwaitingInput, however, it's actually an unreachable
        // case, because the change to the next state (PlayerTurn) is performed in the `player_input` system.
        AwaitingInput => unreachable!(),
        MovePlayer => Collisions,
        Collisions => GenerateMonsterMoves,
        GenerateMonsterMoves => MoveMonsters,
        MoveMonsters => AwaitingInput,
    };

    commands.insert_resource(NextState(new_state));
}
