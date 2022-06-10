use crate::prelude::*;

const ANCHOR: Vector2<f32> = Vector2::new(25., 37.);
// Defensive check.
const INVALID_TEAM: u8 = 255;

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

//# Return True if the given position is inside the level area, otherwise False
//# Takes the goals into account so you can't run through them
fn allow_movement(x: f32, y: f32) -> bool {
    if (x - HALF_LEVEL_W).abs() > HALF_LEVEL_W {
        //# Trying to walk off the left or right side of the level
        false
    } else if (x - HALF_LEVEL_W).abs() < HALF_GOAL_W + 20. {
        //# Player is within the bounds of the goals on the X axis, don't let them walk into, through or behind the goal
        //# +20 takes with of player sprite into account
        (y - HALF_LEVEL_H).abs() < HALF_PITCH_H
    } else {
        //# Player is outside the bounds of the goals on the X axis, so they can walk off the pitch and to the edge
        //# of the level
        (y - HALF_LEVEL_H).abs() < HALF_LEVEL_H
    }
}

#[my_actor_based]
pub struct Player {
    pub peer: Handle<Player>,
    pub mark: TargetHandle,
    pub lead: Option<f32>,
    home: Vector2<f32>,
    pub team: u8,
    pub dir: u8,
    anim_frame: i8,
    pub timer: i32,
    pub shadow: BareActor,
}

impl Player {
    // This doesn't fully initialize the values; reset() needs to be invoked separately. If reset()
    // is not invoked before update(), the latter will fail.
    pub fn new() -> Self {
        //# Player objects are recreated each time there is a kickoff
        //# Team will be 0 or 1
        //# The x and y values supplied represent our 'home' position - the place we'll return to by default when not near
        //# the ball. However, on creation, we want players to be in their kickoff positions, which means all players from
        //# team 0 will be below the halfway line, and players from team 1 above. The player chosen to actually do the
        //# kickoff is moved to be alongside the centre spot after the player objects have been created.

        let vpos = Vector2::new(0., 0.);
        let team = INVALID_TEAM;

        let img_base = BLANK_IMAGE;
        let img_indexes = vec![];

        let peer = Handle::NONE;
        let mark = TargetHandle::None;
        let lead = None;

        let home = Vector2::new(0., 0.);

        //# Facing direction: 0 = up, 1 = top right, up to 7 = top left
        let dir = 0;

        //# Animation frame
        let anim_frame = -1;

        let timer = 0;

        let shadow = BareActor::new(BLANK_IMAGE, Anchor::Custom(ANCHOR));

        //# Used when DEBUG_SHOW_TARGETS is on
        //self.debug_target = Vector2(0, 0)

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

    pub fn reset(&mut self, x: f32, y: f32, team: u8) {
        self.team = team;

        //# Calculate our initial position for kickoff by halving y, adding 550 and then subtracting either 400 for
        //# team 1, or nothing for team 0
        let kickoff_y = (y / 2.) + 550. - (team as f32 * 400.);

        self.vpos = Vector2::new(x, kickoff_y);

        //# Remember home position, where we'll stand by default if we're not active (i.e. far from the ball)
        self.home = Vector2::new(x, y);
    }

    // An option is to pass all the Game fields individually, but this is simpler.
    //
    // this implementation is the simplest (no tickets passed around), but 1. is also a simple alternative.
    //
    pub fn update(
        state: State,
        player_h: Handle<Player>,
        game: &mut Game,
        input: &InputController,
    ) {
        let player = game.pools.players.borrow_mut(player_h);
        let ball = &game.ball[&state];
        let kickoff_player = game.kickoff_player.get(&state);

        if player.team == INVALID_TEAM {
            panic!("Player not reset before updating!");
        }

        player.timer -= 1;

        // Can't keep mutably borrowed over the whole function; mutably reborrowed at the end.
        let player = game.pools.players.borrow(player_h);

        //# One of the main jobs of this method is to decide where the player will run to, and at what speed.
        //# The default is to run slowly towards home position, but target and speed may be overwritten in the code below
        let mut target = player.home.clone(); //# Take a copy of home position
        let mut speed = PLAYER_DEFAULT_SPEED;

        //# Some shorthand variables to make the code below a bit easier to follow
        let my_team = &game.teams[player.team as usize];
        let pre_kickoff = kickoff_player.is_some();
        let i_am_kickoff_player = Some(&player_h) == kickoff_player;

        if Some(player_h) == game.teams[player.team as usize].active_control_player
            && my_team.human()
            && (!pre_kickoff || i_am_kickoff_player)
        {
            //# This player is the currently active player for its team, and is player-controlled, and either we're not
            //# currently waiting for kickoff, or this player is the designated kickoff self.
            //# The last part of the condition ensures that in a 2 player game, player 2 can't make their active player
            //# run around while waiting for player 1 to do the kickoff (and vice versa)

            //# A player with the ball runs slightly more slowly than one without
            speed = if ball.owner == Some(player_h) {
                HUMAN_PLAYER_WITH_BALL_SPEED
            } else {
                HUMAN_PLAYER_WITHOUT_BALL_SPEED
            };

            //# Find target by calling the controller for the player's team todo comment
            target = player.vpos + my_team.controls.as_ref().unwrap().move_player(speed, input);
        } else if let Some(ball_owner_h) = ball.owner {
            let ball_owner = game.pools.players.borrow(ball_owner_h);

            //# Someone has the ball - is it me?
            if ball_owner_h == player_h {
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
                let costs = (-2..3).map(|d| {
                    // angle_to_vec's documentation in the source says that the range of values is from
                    // 0 to 7, but this formula sometimes generate negative values (-2/-1). Such values
                    // yield correct results (as the formula is essentialy mod(8)), however, it's not
                    // clear if it was the intention of the author not to perform mod(8). From the port
                    // perspective though, this is a bug, since unintented wraparound causes debug panic.
                    let angle = (player.dir as i8 + d).rem_euclid(8) as u8;
                    cost(
                        player.vpos + angle_to_vec(angle) * 3.,
                        player.team,
                        d.abs() as u8,
                        &game.pools.players,
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
                (_, target) = costs
                    .min_by(|cost1, cost2| cost1.0.partial_cmp(&cost2.0).unwrap())
                    .unwrap();

                //# speed depends on difficulty
                speed = CPU_PLAYER_WITH_BALL_BASE_SPEED + game.difficulty.speed_boost
            } else if ball_owner.team == player.team {
                //# Ball is owned by another player on our team
                if player.active(&ball) {
                    //# If I'm near enough to the ball, try to run somewhere useful, and unique to this player - we
                    //# don't want all players running to the same place. Target is halfway between home and a point
                    //# 400 pixels ahead of the ball. Team 0 are trying to score in the goal at the top of the
                    //# pitch, team 1 the goal at the bottom
                    let direction = if player.team == 0 { -1. } else { 1. };
                    target.x = (ball.vpos.x + target.x) / 2.;
                    target.y = (ball.vpos.y + 400. * direction + target.y) / 2.;
                }
                //# If we're not active, we'll do the default action of moving towards our home position
            } else {
                let mark_active = player.mark.load(&game.pools).active(&ball);
                let mark_vpos = player.mark.load(&game.pools).vpos();

                //# Ball is owned by a player on the opposite team
                if player.lead.is_some() {
                    //# We are one of the players chosen to pursue the owner

                    //# Target a position in front of the ball's owner, the distance based on the value of lead, while
                    //# making sure we keep just inside the pitch
                    target = ball_owner.vpos + angle_to_vec(ball_owner.dir) * player.lead.unwrap();

                    //# Stay on the pitch
                    target.x = target.x.clamp(AI_MIN_X, AI_MAX_X);
                    target.y = target.y.clamp(AI_MIN_Y, AI_MAX_Y);

                    // Bug here, fixed (was: `other_team = 1 if player.team == 0 else 1`)
                    let other_team = if player.team == 0 { 1 } else { 0 };
                    speed = LEAD_PLAYER_BASE_SPEED;
                    if game.teams[other_team].human() {
                        speed += game.difficulty.speed_boost;
                    }
                } else if mark_active {
                    //# The player or goal we've been chosen to mark is active

                    if my_team.human() {
                        //# If I'm on a human team, just run towards the ball.
                        //# We don't do the marking behaviour below for human teams for a number of reasons. Try changing
                        //# the code to see how the game feels when marking behaviour applies to both human and computer
                        //# teams.
                        target = ball.vpos.clone();
                    } else {
                        //# Get vector between the ball and whatever we're marking
                        let (nvec, mut length) = safe_normalise(&(ball.vpos - mark_vpos));

                        //# Alter length to choose a position in between the ball and whatever we're marking
                        //# We don't apply this behaviour for human teams - in that case we just run straight at the ball
                        if player.mark.is_goal() {
                            //# If I'm currently the goalie, get in between the ball and goal, and don't get too far
                            //# from the goal
                            length = 150_f32.min(length);
                        } else {
                            //# Otherwise, just get halfway between the ball and whoever I'm marking
                            length /= 2.;
                        }

                        target = mark_vpos + nvec * length
                    }
                }
            }
        } else {
            //# No-one has the ball

            //# If we’re pre-kickoff and I’m the kickoff player, OR if we’re not pre-kickoff and I’m active
            if (pre_kickoff && i_am_kickoff_player) || (!pre_kickoff && player.active(&ball)) {
                //# Try to intercept the ball
                //# Deciding where to go to achieve this is harder than you might think. You can't target the ball's
                //# current location, because (assuming it's moving) by the time you get there it'll have moved on, so
                //# you'll always be trailing behind it. And you can't target where it's going to end up after rolling to
                //# a halt, because you might end up getting there before it and just be standing around waiting for it to
                //# get there. What we want to do is find a target which allows us to intercept the ball along its path in
                //# the minimum possible time and distance.
                //# The code below simulates the ball's movement over a series of frames, working out where it would be
                //# after each frame. We also work out how far the player could have moved at each frame, and whether
                //# that distance would be enough to reach the currently simulated location of the ball.
                target = ball.vpos.clone(); //# current simulated location of ball
                let mut vel = ball.vel.clone(); //# ball velocity - slows down each frame due to friction
                let mut frame = 0;

                //# DRIBBLE_DIST_X is the distance at which a player can gain control of the ball.
                //# vel.length() > 0.5 ensures we don't keep simulating frames for longer than necessary - once the ball
                //# is moving that slowly, it's not going to move much further, so there's no point in simulating dozens
                //# more frames of very tiny movements. If you experience a decreased frame rate when no one has the ball,
                //# try increasing 0.5 to a higher number.
                while (target - player.vpos).norm()
                    > PLAYER_INTERCEPT_BALL_SPEED * frame as f32 + DRIBBLE_DIST_X
                    && vel.norm() > 0.5
                {
                    target += vel;
                    vel *= DRAG;
                    frame += 1;
                }

                speed = PLAYER_INTERCEPT_BALL_SPEED;
            } else if pre_kickoff {
                //# Waiting for kick-off, but we're not the kickoff player
                //# Just stay where we are. Without this we'd run to our home position, but that is different from
                //# our position at kickoff (where all players are on their team's side of the pitch)
                target.y = player.vpos.y;
            }
        }

        //# Get direction vector and distance beteen current pos and target pos
        //# vec[0] and vec[1] will be the x and y components of the vector
        let (vek, mut distance) = safe_normalise(&(target - player.vpos));

        //self.debug_target = Vector2(target)

        let target_dir;

        let player = game.pools.players.borrow_mut(player_h);

        //# Check to see if we're already at the target position
        if distance > 0. {
            //# Limit movement to our max speed
            distance = distance.min(speed);

            //# Set facing direction based on the direction we're moving
            target_dir = vec_to_angle(vek);

            //# Update the x and y components of the player's position - but don't allow them to go off the edge of the
            //# level. Processing the x and y components separately allows the player to slide along the edge when trying
            //# to move diagonally off the edge of the level.
            if allow_movement(player.vpos.x + vek.x * distance, player.vpos.y) {
                player.vpos.x += vek.x * distance;
            }
            if allow_movement(player.vpos.x, player.vpos.y + vek.y * distance) {
                player.vpos.y += vek.y * distance;
            }

            //# todo
            player.anim_frame = ((player.anim_frame as f32 + distance.max(1.5)) % 72.) as i8;
        } else {
            //# Already at target position - just turn to face the ball
            target_dir = vec_to_angle(ball.vpos - player.vpos);
            player.anim_frame = -1;
        }

        //# Update facing direction - each frame, move one step towards the target direction
        //# This code essentially says that if the target direction is the same as the current direction, there should
        //# be no change; if target is between 1 and 4 steps clockwise from current, we should rotate one step clockwise,
        //# and if it's between 1 and 3 steps anticlockwise (which can also be thought of as 5 to 7 steps clockwise), we
        //# should rotate one step anticlockwise - which is equivalent to stepping 7 steps clockwise
        let dir_diff = target_dir as i8 - player.dir as i8;
        player.dir = (player.dir + [0, 1, 1, 1, 1, 7, 7, 7][dir_diff.rem_euclid(8) as usize]) % 8;

        let suffix0 = player.dir;
        let suffix1 = (player.anim_frame.div_euclid(18) + 1) as u8; //# todo

        player.img_base = "player";
        player.img_indexes = vec![player.team, suffix0, suffix1];
        player.shadow.img_base = "players";
        player.shadow.img_indexes = vec![suffix0, suffix1];

        //# Update shadow position to track player
        player.shadow.vpos = player.vpos.clone();
    }
}

impl Target for Player {
    fn active(&self, ball: &Ball) -> bool {
        //# Is ball within 400 pixels on the Y axis? If so I'll be considered active, meaning I'm currently doing
        //# something useful in the game like trying to get the ball. If I'm not active, I'll either mark another player,
        //# or just stay at my home position
        (ball.vpos.y - self.home.y).abs() < 400.
    }
    fn team(&self) -> u8 {
        self.team
    }
}
