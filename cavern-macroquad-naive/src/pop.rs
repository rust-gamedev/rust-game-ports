pub struct Pop {
    pub timer: i32,
}

impl Pop {
    pub fn update(&mut self) {
        eprintln!("WRITEME: Pop#update");
    }

    pub fn draw(&self) {
        eprintln!("WRITEME: Pop#draw");
    }
}
