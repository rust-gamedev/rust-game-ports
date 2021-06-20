pub struct Bolt {
    pub active: bool,
}

impl Bolt {
    pub fn update(&mut self) {
        eprintln!("WRITEME: Bolt#update");
    }

    #[allow(dead_code)]
    pub fn draw(&self) {
        eprintln!("WRITEME: Bolt#draw");
    }
}
