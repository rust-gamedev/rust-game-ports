#[derive(Clone, Copy)]
pub enum RobotType {
    Aggressive,
    Normal,
}

#[allow(dead_code)]
pub struct Robot {
    pub type_: RobotType,
}

impl Robot {
    #[allow(dead_code)]
    pub fn update(&mut self) {
        eprintln!("WRITEME: Robot#update");
    }

    #[allow(dead_code)]
    pub fn draw(&self) {
        eprintln!("WRITEME: Robot#draw");
    }
}
