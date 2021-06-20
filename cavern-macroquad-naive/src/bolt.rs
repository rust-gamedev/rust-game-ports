pub struct Bolt {
    pub active: bool,
}

impl Bolt {
    pub fn update(&mut self) {
        eprintln!("WRITEME: Bolt#update");
    }

    pub fn draw(&self) {
        eprintln!("WRITEME: Bolt#draw");
    }
}
