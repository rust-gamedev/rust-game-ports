use crate::controls::Controls;

pub struct Team {
    controls: Option<Controls>,
    pub score: u8,
}

impl Team {
    pub fn new(controls: Option<Controls>) -> Self {
        let score = 0;

        Self { controls, score }
    }

    pub fn human(&self) -> bool {
        self.controls.is_some()
    }
}
