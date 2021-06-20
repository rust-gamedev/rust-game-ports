pub struct Fruit {
    pub x: i32,
    pub y: i32,
    pub time_to_live: i32,
}

impl Fruit {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            time_to_live: 500, // Counts down to zero
        }
    }

    pub fn update(&mut self) {
        eprintln!("WRITEME: Fruit#update");
    }

    pub fn draw(&self) {
        eprintln!("WRITEME: Fruit#draw");
    }
}
