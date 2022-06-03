use crate::prelude::*;

const ANCHOR: Vector2<i16> = Vector2::new(25, 37);

#[my_actor_based]
pub struct Player {
    // We trivially solve the cyclical references problem, by erasing the references at the start of
    // each game.
    pub peer: Option<RCC<Player>>,
}

impl Player {
    pub fn new(x: i16, y: i16, team: u8) -> Self {
        //# Player objects are recreated each time there is a kickoff
        //# Team will be 0 or 1
        //# The x and y values supplied represent our 'home' position - the place we'll return to by default when not near
        //# the ball. However, on creation, we want players to be in their kickoff positions, which means all players from
        //# team 0 will be below the halfway line, and players from team 1 above. The player chosen to actually do the
        //# kickoff is moved to be alongside the centre spot after the player objects have been created.

        //# Calculate our initial position for kickoff by halving y, adding 550 and then subtracting either 400 for
        //# team 1, or nothing for team 0
        let kickoff_y = (y / 2) + 550 - (team as i16 * 400);

        let vpos = Vector2::new(x, kickoff_y);
        let img_base = "blank";
        let img_indexes = vec![];

        let peer = None;

        Self {
            vpos,
            img_base,
            img_indexes,
            anchor: Anchor::Custom(ANCHOR),
            peer,
        }
    }
}
