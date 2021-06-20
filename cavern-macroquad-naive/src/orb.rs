use crate::robot::RobotType;

pub struct Orb {
    pub timer: i32,
    pub y: i32,
    pub trapped_enemy_type: Option<RobotType>,
}

impl Orb {
    pub fn update(&mut self) {
        eprintln!("WRITEME: Orb#update");
    }

    pub fn draw(&self) {
        eprintln!("WRITEME: Orb#draw");
    }
}
