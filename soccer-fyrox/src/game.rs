use crate::{controls::Controls, team::Team};

pub const DEFAULT_DIFFICULTY: u8 = 2;

pub struct Game {
    pub teams: Vec<Team>,
    difficulty: u8,
    pub score_timer: i32,
}

impl Game {
    pub fn new(
        p1_controls: Option<Controls>,
        p2_controls: Option<Controls>,
        difficulty: u8,
    ) -> Self {
        let teams = vec![Team::new(p1_controls), Team::new(p2_controls)];
        let score_timer = 0;

        Self {
            teams,
            difficulty,
            score_timer,
        }
    }

    pub fn update(&mut self) {
        // WRITEME
    }
}
