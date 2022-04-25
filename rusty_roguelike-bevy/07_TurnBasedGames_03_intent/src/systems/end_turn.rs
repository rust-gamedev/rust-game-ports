use crate::prelude::*;

pub fn end_turn(mut turn_state: ResMut<State<TurnState>>) {
    use TurnState::*;

    let new_state = match turn_state.current() {
        AwaitingInput => return,
        PlayerTurn => MonsterTurn,
        MonsterTurn => AwaitingInput,
    };

    // Stack-based states management is also supported, however, in this case, the flow is a cycle,
    // which a stack is not appropriate for.

    turn_state.set(new_state).unwrap();
}
