use crate::prelude::*;

pub fn end_turn(mut turn_state: ResMut<State<TurnState>>) {
    use TurnState::*;

    let new_state = match turn_state.current() {
        // In the source project, AwaitingInput returns AwaitingInput, however, it's actually an unreachable
        // case, because the change to the next state (PlayerTurn) is performed in the `player_input` system.
        AwaitingInput => unreachable!(),
        PlayerTurn => MonsterTurn,
        MonsterTurn => AwaitingInput,
    };

    // Stack-based states management is also supported, however, in this case, the flow is a cycle,
    // which a stack is not appropriate for.

    turn_state.set(new_state).unwrap();
}
