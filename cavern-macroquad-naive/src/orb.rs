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

    #[allow(dead_code)]
    pub fn draw(&self) {
        eprintln!("WRITEME: Orb#draw");
    }
}
