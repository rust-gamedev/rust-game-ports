use crate::prelude::*;

#[my_actor_based]
pub struct Ball {
    vel: Vector2<f32>,
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
