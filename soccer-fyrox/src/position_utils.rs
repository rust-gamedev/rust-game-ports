use crate::prelude::*;

//# Generate a score for a given position, where lower numbers are considered to be better.
//# This is called when a computer-controlled player with the ball is working out which direction to run in, or whether
//# to pass the ball to another player, or kick it into the goal.
//# Several things make up the final score:
//# - the distance to our own goal – further away is better
//# - the proximity of players on the other team – we want to get the ball away from them as much as possible
//# - a quadratic equation (don’t panic too much!) causing the player to favour the centre of the pitch and their opponents goal
//# - an optional handicap value which can bias the result towards or away from a particular position
pub fn cost(
    pos: Vector2<f32>,
    team: u8,
    handicap: u8,
    players_pool: &Pool<Player>,
) -> (f32, Vector2<f32>) {
    //# Get pos of our own goal. We do it this way rather than getting the pos of the actual goal object
    //# because this way gives us the pos of the goal's entrance, whereas the actual goal sprites are not anchored based
    //# on the entrances.
    let own_goal_pos = Vector2::new(HALF_LEVEL_W, if team == 1 { 78. } else { LEVEL_H - 78. });
    let inverse_own_goal_distance = 3500. / (pos - own_goal_pos).norm();

    let result = inverse_own_goal_distance
        + players_pool
            .iter()
            .filter(|p| p.team != team)
            .map(|p| 4000. / 24_f32.max((p.vpos - pos).norm()))
            .sum::<f32>()
        + ((pos.x - HALF_LEVEL_W).powi(2) / 200. - pos.y * (4. * team as f32 - 2.))
        + handicap as f32;

    (result, pos)
}
