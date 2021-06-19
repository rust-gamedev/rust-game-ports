#[derive(Clone, Copy)]
pub enum RobotType {
    Aggressive,
    Normal,
}

#[allow(dead_code)]
pub struct Robot {
    pub type_: RobotType,
}
