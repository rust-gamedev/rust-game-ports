use crate::prelude::*;

const ANCHOR: Vector2<f32> = Vector2::new(25., 37.);

//# Speeds for players in various situations. Speeds including 'BASE' can be boosted by the speed_boost difficulty
//# setting (only for players on a computer-controlled team)
pub const PLAYER_DEFAULT_SPEED: f32 = 2.0;
pub const CPU_PLAYER_WITH_BALL_BASE_SPEED: f32 = 2.6;
pub const PLAYER_INTERCEPT_BALL_SPEED: f32 = 2.75;
pub const LEAD_PLAYER_BASE_SPEED: f32 = 2.9;
pub const HUMAN_PLAYER_WITH_BALL_SPEED: f32 = 3.0;
pub const HUMAN_PLAYER_WITHOUT_BALL_SPEED: f32 = 3.3;

#[my_actor_based]
#[derive(Clone)]
pub struct Player {
    pub peer: Handle<Player>,
    pub mark: Target,
    pub lead: Option<f32>,
    home: Vector2<f32>,
    pub team: u8,
    dir: u8,
    anim_frame: i8,
    pub timer: i32,
    pub shadow: BareActor,
}

impl Player {
    pub fn new(x: f32, y: f32, team: u8) -> Self {
        //# Player objects are recreated each time there is a kickoff
        //# Team will be 0 or 1
        //# The x and y values supplied represent our 'home' position - the place we'll return to by default when not near
        //# the ball. However, on creation, we want players to be in their kickoff positions, which means all players from
        //# team 0 will be below the halfway line, and players from team 1 above. The player chosen to actually do the
        //# kickoff is moved to be alongside the centre spot after the player objects have been created.

        //# Calculate our initial position for kickoff by halving y, adding 550 and then subtracting either 400 for
        //# team 1, or nothing for team 0
        let kickoff_y = (y / 2.) + 550. - (team as f32 * 400.);

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

        let shadow = BareActor::new(BLANK_IMAGE, Anchor::Custom(ANCHOR));

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
        (ball.vpos.y - self.home.y).abs() < 400.
    }

    pub fn update(
        &mut self,
        teams: &[Team],
        kickoff_player: Option<Handle<Player>>,
        self_handle: Handle<Player>,
        ball: &Ball,
    ) {
        //# decrement holdoff timer
        self.timer -= 1;

        //# One of the main jobs of this method is to decide where the player will run to, and at what speed.
        //# The default is to run slowly towards home position, but target and speed may be overwritten in the code below
        let target = self.home.clone(); //# Take a copy of home position
        let speed = PLAYER_DEFAULT_SPEED;

        //# Some shorthand variables to make the code below a bit easier to follow
        let my_team = &teams[self.team as usize];
        let pre_kickoff = kickoff_player.is_some();
        let i_am_kickoff_player = Some(self_handle) == kickoff_player;

        // WRITEME

        let suffix0 = self.dir;
        let suffix1 = (self.anim_frame.div_euclid(18) + 1) as u8; //# todo

        self.img_base = "player";
        self.img_indexes = vec![self.team, suffix0, suffix1];
        self.shadow.img_base = "players";
        self.shadow.img_indexes = vec![suffix0, suffix1];

        //# Update shadow position to track player
        self.shadow.vpos = self.vpos.clone();
    }
}
