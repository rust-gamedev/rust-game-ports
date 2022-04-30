use crate::prelude::*;

pub fn combat(
    mut commands: Commands,
    mut attack_events: EventReader<WantsToAttack>,
    mut health_query: Query<&mut Health>,
) {
    // We can conveniently iterate the message reader, and destructure the message.
    for WantsToAttack { victim, .. } in attack_events.iter() {
        if let Ok(mut health) = health_query.get_mut(*victim) {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.entity(*victim).despawn();
            }
            println!("Health after attack: {}", health.current);
        }
    }
}
