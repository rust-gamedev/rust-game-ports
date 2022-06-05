use crate::prelude::*;

const ANCHOR: Vector2<f32> = Vector2::new(25., 37.);

pub const AI_MIN_X: f32 = 78.;
pub const AI_MAX_X: f32 = LEVEL_W - 78.;
pub const AI_MIN_Y: f32 = 98.;
pub const AI_MAX_Y: f32 = LEVEL_H - 98.;

//# Speeds for players in various situations. Speeds including 'BASE' can be boosted by the speed_boost difficulty
//# setting (only for players on a computer-controlled team)
pub const PLAYER_DEFAULT_SPEED: f32 = 2.0;
pub const CPU_PLAYER_WITH_BALL_BASE_SPEED: f32 = 2.6;
pub const PLAYER_INTERCEPT_BALL_SPEED: f32 = 2.75;
pub const LEAD_PLAYER_BASE_SPEED: f32 = 2.9;
pub const HUMAN_PLAYER_WITH_BALL_SPEED: f32 = 3.0;
pub const HUMAN_PLAYER_WITHOUT_BALL_SPEED: f32 = 3.3;

//# Generate a score for a given position, where lower numbers are considered to be better.
//# This is called when a computer-controlled player with the ball is working out which direction to run in, or whether
//# to pass the ball to another player, or kick it into the goal.
//# Several things make up the final score:
//# - the distance to our own goal – further away is better
//# - the proximity of players on the other team – we want to get the ball away from them as much as possible
//# - a quadratic equation (don’t panic too much!) causing the player to favour the centre of the pitch and their opponents goal
//# - an optional handicap value which can bias the result towards or away from a particular position
fn cost(
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
        input: &InputController,
        players_pool: &Pool<Player>,
        goals_pool: &Pool<Goal>,
        difficulty: &Difficulty,
    ) {
        //# decrement holdoff timer
        self.timer -= 1;

        //# One of the main jobs of this method is to decide where the player will run to, and at what speed.
        //# The default is to run slowly towards home position, but target and speed may be overwritten in the code below
        let mut target = self.home.clone(); //# Take a copy of home position
        let mut speed = PLAYER_DEFAULT_SPEED;

        //# Some shorthand variables to make the code below a bit easier to follow
        let my_team = &teams[self.team as usize];
        let pre_kickoff = kickoff_player.is_some();
        let i_am_kickoff_player = Some(self_handle) == kickoff_player;

        if Some(self_handle) == teams[self.team as usize].active_control_player
            && my_team.human()
            && (!pre_kickoff || i_am_kickoff_player)
        {
            //# This player is the currently active player for its team, and is player-controlled, and either we're not
            //# currently waiting for kickoff, or this player is the designated kickoff player.
            //# The last part of the condition ensures that in a 2 player game, player 2 can't make their active player
            //# run around while waiting for player 1 to do the kickoff (and vice versa)

            //# A player with the ball runs slightly more slowly than one without
            speed = if ball.owner == Some(self_handle) {
                HUMAN_PLAYER_WITH_BALL_SPEED
            } else {
                HUMAN_PLAYER_WITHOUT_BALL_SPEED
            };

            //# Find target by calling the controller for the player's team todo comment
            target = self.vpos + my_team.controls.as_ref().unwrap().move_player(speed, input);
        } else if ball.owner.is_some() {
            //# Someone has the ball - is it me?
            if ball.owner == Some(self_handle) {
                //# We are the owner, and are computer-controlled (otherwise we would have taken the other arm
                //# of the top-level if statement)

                //# Evaluate five positions (left 90, left 45, ahead, right 45, right 90)
                //# target is the one with the lowest value of cost()
                //# List comprehension steps through the angles: -2 to 2, where 0 is up, 1 is up & right, etc
                //# For each angle 'd', we call the cost function with a position, which is 3 pixels from the
                //# current position, if the player were to move in the direction of d. We also pass cost() our team number.
                //# The last parameter, abs(d), introduces a tendency for the player to continue running forward. Try
                //# multiplying it by 3 or 4 to see what happens!

                //# First, create a list of costs for each of the 5 tested positions - a lower number is better. Each
                //# element is a tuple containing the cost and the position that cost relates to.
                let costs = (0..5).map(|d| {
                    let d = d as i8 - 2;
                    // TODO (port): verify that self.dir is always > 2, since angle_to_vec expects the range 0..=7.
                    cost(
                        self.vpos + angle_to_vec((self.dir as i8 + d) as u8) * 3.,
                        self.team,
                        d.abs() as u8,
                        &players_pool,
                    )
                });

                //# Then choose the element with the lowest cost. We use min() to find the element with the lowest value.
                //# min uses < to compare pairs of elements. Each element of costs is a tuple with two elements (a cost
                //# value and the target position). When comparing a pair of tuples using <, Python first compares the
                //# first element of each tuple. If they're different, that's what determines which tuple is considered to
                //# have a lower value. If they're the same, Python moves on to looking at the next element. However, this
                //# can lead to a crash in this case as the target position is an instance of the Vector2 class, which
                //# does not support comparisons using <. In practice it's rare for two positions to have the same cost
                //# value, but it's nevertheless prudent to eliminate the risk. The solution we chosen is to use the
                //# optional 'key' parameter for min, telling the function to only use the first element of each tuple
                //# for the comparisons.
                //# When min finds the tuple with the minimum cost value, we extract the target pos (which is what we
                //# actually care about) and discard the actual cost value - hence the '_' dummy variable
                let target = costs
                    .map(|(_, pos)| pos)
                    .min_by(|pos1, pos2| pos1[0].partial_cmp(&pos2[0]).unwrap())
                    .unwrap();

                //# speed depends on difficulty
                speed = CPU_PLAYER_WITH_BALL_BASE_SPEED + difficulty.speed_boost
            }
        } else if players_pool.borrow(ball.owner.unwrap()).team == self.team {
            //# Ball is owned by another player on our team
            if self.active(ball) {
                //# If I'm near enough to the ball, try to run somewhere useful, and unique to this player - we
                //# don't want all players running to the same place. Target is halfway between home and a point
                //# 400 pixels ahead of the ball. Team 0 are trying to score in the goal at the top of the
                //# pitch, team 1 the goal at the bottom
                let direction = if self.team == 0 { -1. } else { 1. };
                target.x = (ball.vpos.x + target.x) / 2.;
                target.y = (ball.vpos.y + 400. * direction + target.y) / 2.;
            }
            //# If we're not active, we'll do the default action of moving towards our home position
        } else {
            //# Ball is owned by a player on the opposite team
            if self.lead.is_some() {
                let ball_owner = players_pool.borrow(ball.owner.unwrap());
                //# We are one of the players chosen to pursue the owner

                //# Target a position in front of the ball's owner, the distance based on the value of lead, while
                //# making sure we keep just inside the pitch
                target = ball_owner.vpos + angle_to_vec(ball_owner.dir) * self.lead.unwrap();

                //# Stay on the pitch
                target.x = target.x.clamp(AI_MIN_X, AI_MAX_X);
                target.y = target.y.clamp(AI_MIN_Y, AI_MAX_Y);

                // Bug here, fixed (was: `other_team = 1 if self.team == 0 else 1`)
                let other_team = if self.team == 0 { 1 } else { 0 };
                speed = LEAD_PLAYER_BASE_SPEED;
                if teams[other_team].human() {
                    speed += difficulty.speed_boost;
                }
            } else if self.mark.active(players_pool, goals_pool, ball) {
                //# The player or goal we've been chosen to mark is active

                if my_team.human() {
                    //# If I'm on a human team, just run towards the ball.
                    //# We don't do the marking behaviour below for human teams for a number of reasons. Try changing
                    //# the code to see how the game feels when marking behaviour applies to both human and computer
                    //# teams.
                    target = ball.vpos.clone();
                } else {
                    //# Get vector between the ball and whatever we're marking
                    let (nvec, length) =
                        safe_normalise(&(ball.vpos - self.mark.vpos(players_pool, goals_pool)));

                    //# Alter length to choose a position in between the ball and whatever we're marking
                    //# We don't apply this behaviour for human teams - in that case we just run straight at the ball
                    if self.mark.is_goal() {
                        //# If I'm currently the goalie, get in between the ball and goal, and don't get too far
                        //# from the goal
                        length = 150_f32.min(length)
                    } else {
                        //# Otherwise, just get halfway between the ball and whoever I'm marking
                        length /= 2.;
                    }

                    target = self.mark.vpos(players_pool, goals_pool) + nvec * length
                }
            }
        }

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
