use crate::controls::Controls;

pub const DEFAULT_DIFFICULTY: u8 = 2;

pub struct Game {
    p1_controls: Option<Controls>,
    p2_controls: Option<Controls>,
    difficulty: u8,
}

impl Game {
    pub fn new(
        p1_controls: Option<Controls>,
        p2_controls: Option<Controls>,
        difficulty: u8,
    ) -> Self {
        Self {
            p1_controls,
            p2_controls,
            difficulty,
        }
    }

    pub fn update(&mut self) {
        // WRITEME
    }
}
