pub struct Bolt {}

impl Bolt {
    #[allow(dead_code)]
    pub fn update(&mut self) {
        eprintln!("WRITEME: Bolt#update");
    }

    #[allow(dead_code)]
    pub fn draw(&self) {
        eprintln!("WRITEME: Bolt#draw");
    }
}
