pub struct Fruit {}

impl Fruit {
    #[allow(dead_code)]
    pub fn update(&mut self) {
        eprintln!("WRITEME: Fruit#update");
    }

    #[allow(dead_code)]
    pub fn draw(&self) {
        eprintln!("WRITEME: Fruit#draw");
    }
}
