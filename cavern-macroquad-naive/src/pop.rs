pub struct Pop {
    pub timer: i32,
}

impl Pop {
    pub fn update(&mut self) {
        eprintln!("WRITEME: Pop#update");
    }

    #[allow(dead_code)]
    pub fn draw(&self) {
        eprintln!("WRITEME: Pop#draw");
    }
}
