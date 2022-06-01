use crate::prelude::*;

pub struct Player {
    // We trivially solve the cyclical references problem, by erasing the references at the start of
    // each game.
    pub peer: Option<RCC<Player>>,
}

impl Player {
    pub fn new(_x: i16, _y: i16, _team: u8) -> Self {
        let peer = None;

        Self { peer }
    }
}
