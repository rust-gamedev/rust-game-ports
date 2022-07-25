use crate::prelude::*;

const PITCH_BOUNDS_X: (f32, f32) = (HALF_LEVEL_W - HALF_PITCH_W, HALF_LEVEL_W + HALF_PITCH_W);
const PITCH_BOUNDS_Y: (f32, f32) = (HALF_LEVEL_H - HALF_PITCH_H, HALF_LEVEL_H + HALF_PITCH_H);

const GOAL_BOUNDS_X: (f32, f32) = (HALF_LEVEL_W - HALF_GOAL_W, HALF_LEVEL_W + HALF_GOAL_W);
const GOAL_BOUNDS_Y: (f32, f32) = (
    HALF_LEVEL_H - HALF_PITCH_H - GOAL_DEPTH,
    HALF_LEVEL_H + HALF_PITCH_H + GOAL_DEPTH,
);

const PITCH_RECT: Rect = Rect::new(
    PITCH_BOUNDS_X.0,
    PITCH_BOUNDS_Y.0,
    HALF_PITCH_W * 2.,
    HALF_PITCH_H * 2.,
);
const GOAL_0_RECT: Rect = Rect::new(GOAL_BOUNDS_X.0, GOAL_BOUNDS_Y.0, GOAL_WIDTH, GOAL_DEPTH);
const GOAL_1_RECT: Rect = Rect::new(
    GOAL_BOUNDS_X.0,
    GOAL_BOUNDS_Y.1 - GOAL_DEPTH,
    GOAL_WIDTH,
    GOAL_DEPTH,
);

//# ball physics for one axis
fn ball_physics(mut pos: f32, mut vel: f32, bounds: (f32, f32)) -> (f32, f32) {
    //# Add velocity to position
    pos += vel;

    //# Check if ball is out of bounds, and bounce if so
    if pos < bounds.0 || pos > bounds.1 {
        (pos, vel) = (pos - vel, -vel)
    }

    //# Return new position and velocity, applying drag
    (pos, vel * DRAG)
}

//# Work out number of physics steps for ball to travel given distance
fn steps(mut distance: f32) -> u16 {
    //# Initialize step count and initial velocity
    let (mut steps, mut vel) = (0, KICK_STRENGTH);

    //# Run physics until distance reached or ball is nearly stopped
    while distance > 0. && vel > 0.25 {
        (distance, steps, vel) = (distance - vel, steps + 1, vel * DRAG)
    }

    steps
}

//# Calculate if player 'target' is a good target for a pass from player 'source'
//# target can also be a goal
// The source project has a mistake - 'target' can also be a Goal.
fn targetable(
    target: &dyn Target,
    source: &Player,
    teams: &[Team],
    players_pool: &Pool<Player>,
) -> bool {
    //# Find normalised (unit) vector v0 and distance d0 from source to target
    let (v0, d0) = safe_normalise(&(target.vpos() - source.vpos));

    //# If source player is on a computer-controlled team, avoid passes which are likely to be intercepted
    //# (If source is player-controlled, that's the player's job)
    if !teams[source.team as usize].human() {
        //# For each player p
        for p in players_pool.iter() {
            //# Find normalised vector v1 and distance d1 from source to p
            let (v1, d1) = safe_normalise(&(p.vpos - source.vpos));

            //# If p is on the other team, and between source and target, and at a similiar
            //# angular position, target is not a good target
            //# Multiplying two vectors together invokes an operation known as dot product. It is calculated by
            //# multiplying the X components of each vector, then multiplying the Y components, then adding the two
            //# resulting numbers. When each of the input vectors is a unit vector (i.e. with a length of 1, as returned
            //# from the safe_normalise function), the result of which is a number between -1 and 1. In this case we use
            //# the result to determine whether player 'p' (vector v1) is in roughly the same direction as player 'target'
            //# (vector v0), from the point of view of player 'source'.
            if p.team != target.team() && d1 > 0. && d1 < d0 && v0.dot(&v1) > 0.8 {
                return false;
            }
        }
    }

    //# If target is on the same team, and ahead of source, and not too far away, and source is facing
    //# approximately towards target (another dot product operation), then target is a good target.
    //# The dot product operation (multiplying two unit vectors) is used to determine whether (and to what extent) the
    //# source player is facing towards the target player. A value of 1 means target is directly ahead of source; -1
    //# means they are directly behind; 0 means they are directly to the left or right.
    //# See above for more explanation of dot product
    target.team() == source.team && d0 > 0. && d0 < 300. && v0.dot(&angle_to_vec(source.dir)) > 0.8
}

//# Get average of two numbers; if the difference between the two is less than 1,
//# snap to the second number. Used in Ball.update()
fn avg(a: f32, b: f32) -> f32 {
    if (b - a).abs() < 1. {
        b
    } else {
        (a + b) / 2.
    }
}

fn on_pitch(x: f32, y: f32) -> bool {
    //# Only used when dribbling
    PITCH_RECT.collidepoint(x, y)
        || GOAL_0_RECT.collidepoint(x, y)
        || GOAL_1_RECT.collidepoint(x, y)
}

#[my_actor_based]
pub struct Ball {
    pub vel: Vector2<f32>,
    pub owner: Option<Handle<Player>>,
    timer: i32,
    pub shadow: BareActor,
}

impl Ball {
    pub fn new(graph: &mut Graph) -> Self {
        let img_base = "ball";
        let img_indexes = vec![];

        // Placeholders - reset below.
        //
        let vpos = Vector2::zero();
        let vel = Vector2::zero();
        let owner = None;
        let timer = 0;

        let shadow = BareActor::new("balls", None, Anchor::Center, graph);
        let rectangle_h = RectangleBuilder::new(BaseBuilder::new()).build(graph);

        let mut instance = Self {
            img_base,
            img_indexes,
            vpos,
            anchor: Anchor::Center,
            vel,
            owner,
            timer,
            shadow,
            rectangle_h,
        };

        instance.reset();

        instance
    }

    pub fn reset(&mut self) {
        self.vpos = Vector2::new(HALF_LEVEL_W, HALF_LEVEL_H);
        self.vel = Vector2::new(0.0, 0.0);
        self.owner = None;
        self.timer = 0;
    }

    //# Check for collision with player p
    fn collide(&self, p: &Player) -> bool {
        //# The ball collides with p if p's hold-off timer has expired
        //# and it is DRIBBLE_DIST_X or fewer pixels away
        p.timer < 0 && (p.vpos - self.vpos).norm() <= DRIBBLE_DIST_X
    }

    // We can't pass `&mut game.ball` and `&mut game` at the same time, so we just just make this a
    // function, and call it a day :)
    pub fn update(game: &mut Game, input: &InputController, scene: &mut Scene, media: &Media) {
        let ball = &mut game.ball;
        ball.timer -= 1;

        //# If the ball has an owner, it's being dribbled, so its position is
        //# based on its owner's position
        if let Some(owner_h) = ball.owner {
            let owner = game.pools.players.borrow_mut(owner_h);
            //# Calculate new ball position for dribbling
            //# Our target position will be a point just ahead of our owner. However, we don't want to just snap to that
            //# position straight away. We want to transition to it over several frames, so we take the average of our
            //# current position and the target position. We also use slightly different offsets for the X and Y axes,
            //# to reflect that that the game's perspective is not completely top-down - so the positions the ball can
            //# take in relation to the player should form an ellipse instead of a circle.
            //# todo explain maths
            let new_x = avg(ball.vpos.x, owner.vpos.x + DRIBBLE_DIST_X * sin(owner.dir));
            let new_y = avg(ball.vpos.y, owner.vpos.y - DRIBBLE_DIST_Y * cos(owner.dir));

            if on_pitch(new_x, new_y) {
                //# New position is on the pitch, so update
                ball.vpos = Vector2::new(new_x, new_y);
            } else {
                //# New position is off the pitch, so player loses the ball
                //# Set hold-off timer so player can't immediately reacquire the ball
                owner.timer = 60;

                //# Give ball small velocity in player's direction of travel
                ball.vel = angle_to_vec(owner.dir) * 3.;

                //# Un-set owner
                ball.owner = None;
            }
        } else {
            //# Run physics, one axis at a time

            //# If ball is vertically inside the goal, it can only go as far as the
            //# sides of the goal - otherwise it can go all the way to the sides of
            //# the pitch
            let bounds_x = if (ball.vpos.y - HALF_LEVEL_H).abs() > HALF_PITCH_H {
                GOAL_BOUNDS_X
            } else {
                PITCH_BOUNDS_X
            };

            //# If ball is horizontally inside the goal, it can go all the way to
            //# the back of the net - otherwise it can only go up to the end of
            //# the pitch
            let bounds_y = if (ball.vpos.x - HALF_LEVEL_W).abs() < HALF_GOAL_W {
                GOAL_BOUNDS_Y
            } else {
                PITCH_BOUNDS_Y
            };

            (ball.vpos.x, ball.vel.x) = ball_physics(ball.vpos.x, ball.vel.x, bounds_x);
            (ball.vpos.y, ball.vel.y) = ball_physics(ball.vpos.y, ball.vel.y, bounds_y);
        }

        //# Update shadow position to track ball
        ball.shadow.vpos = ball.vpos;

        let mut ball_owner_r = ball
            .owner
            .map(|owner_h| game.pools.players.take_reserve(owner_h));

        //# Search for a player that can acquire the ball
        for target in game.pools.players.iter() {
            //# A player can acquire the ball if the ball has no owner, or the player is on the other team
            //# from the owner, and collides with the ball
            // Restructured the condition, in order to accommodate the Rust approach.
            if !ball_owner_r.is_some_and(|(_, ball_owner)| ball_owner.team == target.team)
                && ball.collide(target)
            {
                if let Some((_, ball_owner)) = &mut ball_owner_r {
                    //# New player is taking the ball from previous owner
                    //# Set hold-off timer so previous owner can't immediately reacquire the ball
                    ball_owner.timer = 60;
                }

                //# Set hold-off timer (dependent on difficulty) to limit rate at which
                //# computer-controlled players can pass the ball
                ball.timer = game.difficulty.holdoff_timer as i32;

                //# Update owner, and controllable player for player's team, to player
                ball.owner = Some(game.pools.players.handle_of(target));
                game.teams[target.team as usize].active_control_player = ball.owner;
            }
        }

        if let Some((ball_owner_t, ball_owner)) = ball_owner_r {
            game.pools.players.put_back(ball_owner_t, ball_owner);
        }

        //# If the ball has an owner, it's time to decide whether to kick it
        if let Some(owner_h) = ball.owner {
            let ball_owner = game.pools.players.borrow(owner_h);
            let team = &game.teams[ball_owner.team as usize];

            //# Find the closest targetable player or goal (could be None)
            //# First we create a list of all players/goals which can be targeted

            let mut targetable_players = game
                .pools
                .players
                .iter()
                .filter(|p| {
                    p.team == ball_owner.team
                        && targetable(*p, ball_owner, &game.teams, &game.pools.players)
                })
                .map(|p| TargetHandle::Player(game.pools.players.handle_of(p)))
                .collect::<Vec<_>>();

            targetable_players.extend(
                game.pools
                    .goals
                    .iter()
                    .filter(|p| {
                        p.team() == ball_owner.team
                            && targetable(*p, ball_owner, &game.teams, &game.pools.players)
                    })
                    .map(|p| TargetHandle::Goal(game.pools.goals.handle_of(p))),
            );

            let target = if targetable_players.len() > 0 {
                //# Choose the nearest one
                //# dist_key returns a function which gets the distance of the ball owner from whichever player or goal (p)
                //# the sorted function is currently assessing
                targetable_players.iter().min_by(|p1, p2| {
                    dist_key(
                        &p1.load(&game.pools).vpos(),
                        &p2.load(&game.pools).vpos(),
                        ball_owner.vpos,
                    )
                })
                //game.debug_shoot_target = target.vpos
            } else {
                None
            };
            let do_shoot = if team.human() {
                //# If the owner is player-controlled, we kick if the player hits their kick key
                team.controls.as_ref().unwrap().shoot(input)
            } else {
                //# If the owner is computer-controlled, we kick if the ball's hold-off timer has expired
                //# and there is a targetable player or goal, and the targetable player or goal is in a more
                //# favourable location (according to cost()) than the owner's location
                ball.timer <= 0
                    && target.is_some_and(|target| {
                        cost(
                            target.load(&game.pools).vpos(),
                            ball_owner.team,
                            0,
                            &game.pools.players,
                        ) < cost(ball_owner.vpos, ball_owner.team, 0, &game.pools.players)
                    })
            };

            if do_shoot {
                //# play a random kick effect

                media.play_sound(scene, "goal", &[thread_rng().gen_range(0..2)]);
                // game.play_sound("kick", 4);

                // Initialize to a phony; the compiler (appropriately) thinks that can be left uninitialized.
                let mut vek = Vector2::zero();

                let target = if let Some(target) = target {
                    //# If there is a targetable player or goal, kick towards it

                    //# If the owner is player-controlled, we assume the player will continue to hold the same direction
                    //# keys down after the pass, so the target  will start moving in the same direction as the
                    //# current owner; on this assumption, we will kick the ball slightly ahead of the target player's
                    //# current position,  through a process of iterative refinement

                    //# If the owner is computer-controlled, or the target is a goal, we only execute the loop once and
                    //# so do not apply lead, as there are no keys being held down and goals don't move.

                    let mut r = 0.;

                    //# Decide how many times we're going to go through the loop - the more times, the more accurate
                    let iterations = if team.human() && target.is_player() {
                        8
                    } else {
                        1
                    };

                    for _ in 0..iterations {
                        //# In the first loop, t will simply be the position of the targeted player or goal.
                        //# In subsequent loops (if there are any), it will represent a position which is at the
                        //# target's feet plus a bit further in whichever direction the player is currently pressing.
                        let t = target.load(&game.pools).vpos() + angle_to_vec(ball_owner.dir) * r;

                        //# Get direction vector and distance between target pos and us
                        let (vek_copy, length) = safe_normalise(&(t - ball.vpos));
                        vek = vek_copy;

                        //# The steps function works out the number of physics steps the ball will take to travel
                        //# the given distance
                        //# todo r
                        r = HUMAN_PLAYER_WITHOUT_BALL_SPEED * steps(length) as f32
                    }

                    *target
                } else {
                    //# We're not targeting a player or goal, so just kick the ball straight ahead

                    //# Get direction vector
                    vek = angle_to_vec(ball_owner.dir);

                    //# Make a rough guess at which player the ball might end up closest to so, we can set them as the new
                    //# active player. Pick a point 250 pixels ahead and find the nearest player to that.

                    let closest_player = game
                        .pools
                        .players
                        .iter()
                        .filter(|p| p.team == ball_owner.team)
                        .min_by(|p1, p2| dist_key(&p1.vpos, &p2.vpos, ball.vpos + (vek * 250.)))
                        .unwrap();

                    TargetHandle::Player(game.pools.players.handle_of(closest_player))
                };
                if let TargetHandle::Player(target) = target {
                    //# If we just kicked the ball towards a player, make that player the new active player for this team
                    game.teams[ball_owner.team as usize].active_control_player = Some(target);
                }

                // Reborrow mutably, otherwise there would be a mutable and immutable references to
                // a Player.
                let ball_owner = game.pools.players.borrow_mut(owner_h);
                ball_owner.timer = 10; //# Owner can't regain the ball for at least 10 frames

                //# Set velocity
                ball.vel = vek * KICK_STRENGTH;

                //# We no longer have an owner
                ball.owner = None
            }
        }
    }
}
