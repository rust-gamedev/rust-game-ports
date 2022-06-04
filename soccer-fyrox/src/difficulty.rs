pub const DIFFICULTY: [Difficulty; 3] = [
    Difficulty::new(false, false, 0.0, 120),
    Difficulty::new(false, true, 0.1, 90),
    Difficulty::new(true, true, 0.2, 60),
];

#[derive(Clone, Copy)]
pub struct Difficulty {
    pub goalie_enabled: bool,
    //# When a player has the ball, either one or two players will be chosen from the other team to try to intercept
    //# the ball owner. Those players will have their 'lead' attributes set to a number indicating how far ahead of the
    //# ball they should try to run. (If they tried to go to where the ball is currently, they'd always trail behind)
    //# This attribute determines whether there should be one or two lead players
    second_lead_enabled: bool,
    //# Speed boost to apply to CPU-team players in certain circumstances
    speed_boost: f32,
    //# Hold-off timer limits rate at which computer-controlled players can pass the ball
    holdoff_timer: u32,
}

impl Difficulty {
    pub const fn new(
        goalie_enabled: bool,
        second_lead_enabled: bool,
        speed_boost: f32,
        holdoff_timer: u32,
    ) -> Self {
        Self {
            goalie_enabled,
            second_lead_enabled,
            speed_boost,
            holdoff_timer,
        }
    }
}
