use crate::prelude::*;

// Makes it more compact to pass them around when both are needed.
pub struct Pools {
    pub players: Pool<Player>,
    pub goals: Pool<Goal>,
}

impl Pools {
    pub fn new() -> Self {
        Self {
            players: Pool::new(),
            goals: Pool::new(),
        }
    }
}
