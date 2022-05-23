use crate::controls::Controls;

pub struct Team {
    controls: Controls,
}

impl Team {
    pub fn new(controls: Controls) -> Self {
        Self { controls }
    }
}
