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
fn targetable(target: &Player, source: &Player, game: &Game) -> bool {
    //# Find normalised (unit) vector v0 and distance d0 from source to target
    let (v0, d0) = safe_normalise(&(target.vpos - source.vpos));

    //# If source player is on a computer-controlled team, avoid passes which are likely to be intercepted
    //# (If source is player-controlled, that's the player's job)
    if !game.teams[source.team as usize].human() {
        //# For each player p
        for p in game.players_pool.iter() {
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
            if p.team != target.team && d1 > 0. && d1 < d0 && v0.dot(&v1) > 0.8 {
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
    target.team == source.team && d0 > 0. && d0 < 300. && v0.dot(&angle_to_vec(source.dir)) > 0.8
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
    pub fn new() -> Self {
        let vpos = Vector2::new(HALF_LEVEL_W, HALF_LEVEL_H);

        let img_base = "ball";
        let img_indexes = vec![];

        //# Velocity
        let vel = Vector2::new(0.0, 0.0);

        let owner = None;
        let timer = 0;

        let shadow = BareActor::new("balls", Anchor::Center);

        Self {
            img_base,
            img_indexes,
            vpos,
            anchor: Anchor::Center,
            vel,
            owner,
            timer,
            shadow,
        }
    }

    //# Check for collision with player p
    fn collide(&self, p: &Player) -> bool {
        //# The ball collides with p if p's hold-off timer has expired
        //# and it is DRIBBLE_DIST_X or fewer pixels away
        p.timer < 0 && (p.vpos - self.vpos).norm() <= DRIBBLE_DIST_X
    }

    pub fn update(&mut self) {
        // WRITEME
    }
}
