pub struct Bolt {
    pub pos: (i32, i32),
    pub direction_x: i32,
    pub active: bool,
}

impl Bolt {
    pub fn new(x: i32, y: i32, direction_x: i32) -> Self {
        Self {
            pos: (x, y),
            direction_x,
            active: true,
        }
    }

    pub fn update(&mut self) {
        eprintln!("WRITEME: Bolt#update");
    }

    pub fn draw(&self) {
        eprintln!("WRITEME: Bolt#draw");
    }
}
