use crate::prelude::*;

pub struct Goal {
    x: i16,
    y: i16,
    team: u8,
}

impl Goal {
    pub fn new(team: u8) -> Self {
        let x = HALF_LEVEL_W;
        let y = if team == 0 { 0 } else { LEVEL_H };

        Self { x, y, team }
    }
}

impl Actor for Goal {
    fn draw_info(&self) -> (&'static str, Vec<u8>, i16, i16) {
        ("base", vec![self.team], self.x, self.y)
    }
}
