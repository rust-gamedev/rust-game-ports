#[derive(Clone, Copy)]
pub enum RobotType {
    Aggressive,
    Normal,
}

#[allow(dead_code)]
pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub type_: RobotType,
    pub alive: bool,
}

impl Robot {
    pub fn new(x: i32, y: i32, type_: RobotType) -> Self {
        Self {
            x,
            y,
            type_,
            alive: true,
        }
    }

    pub fn update(&mut self) {
        eprintln!("WRITEME: Robot#update");
    }

    pub fn draw(&self) {
        eprintln!("WRITEME: Robot#draw");
    }
}
