use crate::prelude::*;

pub fn combat(
    mut commands: Commands,
    mut attack_events: EventReader<WantsToAttack>,
    mut health_query: Query<&mut Health>,
    player_query: Query<&Player>,
) {
    // We can conveniently iterate the message reader, and destructure the message.
    for WantsToAttack { victim, .. } in attack_events.iter() {
        let is_player = player_query.get(*victim).is_ok();

        if let Ok(mut health) = health_query.get_mut(*victim) {
            health.current -= 1;
            if health.current < 1 && !is_player {
                commands.entity(*victim).despawn();
            }
        }
    }
}
