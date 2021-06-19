pub struct Player {
    pub lives: i32,
}

impl Player {
    pub fn new() -> Self {
        Self { lives: 2 }
    }

    pub fn reset(&mut self) {
        eprintln!("WRITEME: Player#reset");
    }
}
