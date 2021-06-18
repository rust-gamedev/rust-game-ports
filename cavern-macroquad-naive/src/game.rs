use crate::player::Player;

pub struct Game {
    pub player: Option<Player>,
    pub timer: i32,
}

impl Game {
    pub fn new(player: Option<Player>) -> Self {
        Self { player, timer: -1 }
    }

    pub fn update(&mut self) {
        self.timer += 1;

        println!("WRITEME: Game#update");
    }

    pub fn draw(&self) {
        println!("WRITEME: Game#draw");
    }

    pub fn play_sound(&self, _name: &str) {
        println!("WRITEME: Game#play_sound");
    }
}
