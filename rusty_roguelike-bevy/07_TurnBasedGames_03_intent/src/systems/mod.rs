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

    // Specifying the addition to CoreStage::Update is redundant, but it makes the structure clearer.

    // In the port design, rendering is done at the beginning of each frame, rather than at the end.
    // The difference is significant, architecturally; if rendering is performed at the end, it requires
    // a flush (therefore, another step, in our design); by placing it at the beginning, it takes advantage
    // of the flush performed at the end of the previous frame.
    // In the overall steps sequence, the location is essentially the same, so there are no (bad) side
    // effects.
    //
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
            .run_in_state(GenerateMonstersMovements)
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
