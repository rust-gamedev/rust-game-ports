use crate::prelude::*;

mod collisions;
mod entity_render;
mod map_render;
mod movement;
mod next_step;
mod player_input;
mod random_move;

pub fn build_system_sets(app: &mut App) {
    use GameStep::*;

    // As of v0.7, Bevy has two fundamental deficiencies - it lacks on-demand flushing (it flushes only
    // at the end of the frame), and states and stages can't be mixed.
    // Without on-demand flushing, a system in a serialized sequence won't detect changes requested
    // by the previous ones. A workaround is to execute only the set of systems associated to a single
    // state for each frame (via `SystemSet::on_update(state)` filter), however, the state can't be
    // updated during the CoreStage::Update stage (the default), otherwise, the next system set will
    // kick off immediately, without waiting for the flush; in turn, a workaround could be to perform
    // the state change in a separate stage, but this requires mixing states and stages, which is currently
    // unsupported (likely, it doesn't work even if state filtering is performed by if/then/else tests
    // inside each system).
    //
    // In order to model this design, the `iyes_loopless` crate is required.
    //
    // Bevy's translation of Legion's schedulers and flushes are (iyes_loopless) states; besides supporting
    // mixing stages and states, on state change, the frame is flushed (note that this will, in turn,
    // cause issues with the Bevy events system).
    //
    // In order to translate the concept, we need to:
    //
    // - divide the systems into groups
    // - create an enum for each group
    // - run each set of systems during the corresponding state
    // - add a state change system
    // - perform rendering independently from the system set
    //
    // Compared to Legion, this is a non neglibile amount of boilerplate (and side effects), although
    // the logic is straightforward.
    // The state change system is also be required by Legion, but the amount of states is smaller.
    //
    // Finally, a side effect of iyes_loopless is that, since a state_change flushes the frame, and
    // the library we use is set to run at a fixed amount of frames per second, the port is "slower",
    // as it requires multiple frames for a full cycle. This is easy to improve, but this will be done
    // separately.

    app.add_system_set(
        SystemSet::new()
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render),
    );

    app.add_system(
        player_input::player_input.run_in_state(AwaitingInput),
        // The next step is set inside AwaitingInput
    );

    app.add_system_set(
        ConditionSet::new()
            .run_in_state(MovePlayer)
            .with_system(movement::movement)
            .with_system(next_step::next_step)
            .into(),
    );

    app.add_system_set(
        ConditionSet::new()
            .run_in_state(Collisions)
            .with_system(collisions::collisions)
            .with_system(next_step::next_step)
            .into(),
    );

    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GenerateMonsterMoves)
            .with_system(random_move::random_move)
            .with_system(next_step::next_step)
            .into(),
    );

    app.add_system_set(
        ConditionSet::new()
            .run_in_state(MoveMonsters)
            .with_system(movement::movement)
            .with_system(next_step::next_step)
            .into(),
    );
}
