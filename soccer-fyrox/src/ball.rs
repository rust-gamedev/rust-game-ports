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

    pub fn update(&mut self) {
        // WRITEME
    }
}
