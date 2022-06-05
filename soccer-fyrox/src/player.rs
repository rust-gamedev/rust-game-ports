use crate::prelude::*;

const ANCHOR: Vector2<i16> = Vector2::new(25, 37);

#[my_actor_based]
pub struct Player {
    pub peer: Handle<Player>,
    pub mark: Target,
    pub lead: Option<Handle<Player>>,
    home: Vector2<i16>,
    pub team: u8,
    dir: u8,
    anim_frame: i8,
    pub timer: i32,
    pub shadow: BareActor,
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
        let img_base = BLANK_IMAGE;
        let img_indexes = vec![];

        let peer = Handle::NONE;
        let mark = Target::None;
        let lead = None;

        //# Remember home position, where we'll stand by default if we're not active (i.e. far from the ball)
        let home = Vector2::new(x, y);

        //# Facing direction: 0 = up, 1 = top right, up to 7 = top left
        let dir = 0;

        //# Animation frame
        let anim_frame = -1;

        let timer = 0;

        let shadow = BareActor::new(Anchor::Custom(ANCHOR));

        Self {
            vpos,
            img_base,
            img_indexes,
            anchor: Anchor::Custom(ANCHOR),
            peer,
            mark,
            lead,
            home,
            team,
            dir,
            anim_frame,
            timer,
            shadow,
        }
    }

    pub fn active(&self, ball: &Ball) -> bool {
        //# Is ball within 400 pixels on the Y axis? If so I'll be considered active, meaning I'm currently doing
        //# something useful in the game like trying to get the ball. If I'm not active, I'll either mark another player,
        //# or just stay at my home position
        (ball.vpos.y - self.home.y).abs() < 400
    }
}
