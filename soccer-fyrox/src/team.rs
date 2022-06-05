use crate::prelude::*;

pub struct Team {
    pub controls: Option<Controls>,
    pub score: u8,
    pub active_control_player: Option<Handle<Player>>,
}

impl Team {
    pub fn new(controls: Option<Controls>) -> Self {
        let score = 0;
        let active_control_player = None;

        Self {
            controls,
            score,
            active_control_player,
        }
    }

    pub fn human(&self) -> bool {
        self.controls.is_some()
    }
}
