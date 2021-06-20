use crate::{actor::Actor, orb::Orb, WIDTH};

#[derive(Default)]
pub struct Player {
    pub lives: i32,
    pub score: i32,
    pub vel_y: f32,
    pub direction_x: i32, // -1 = left, 1 = right
    pub fire_timer: i32,
    pub hurt_timer: i32,
    pub health: i32,
    pub blowing_orb: Option<Orb>,

    // Actor trait
    pub x: i32,
    pub y: i32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            lives: 2,
            score: 0,
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        self.x = WIDTH / 2;
        self.y = 100;
        self.vel_y = 0.;
        self.direction_x = 1; // -1 = left, 1 = right
        self.fire_timer = 0;
        self.hurt_timer = 100; // Invulnerable for this many frames
        self.health = 3;
        self.blowing_orb = None;
    }

    pub fn update(&mut self) {
        eprintln!("WRITEME: Player#update");
    }

    pub fn draw(&self) {
        eprintln!("WRITEME: Player#draw");
    }
}

impl Actor for Player {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn y_mut(&mut self) -> &mut i32 {
        &mut self.y
    }

    fn image(&self) -> macroquad::prelude::Texture2D {
        todo!()
    }

    fn anchor(&self) -> crate::actor::Anchor {
        todo!()
    }
}
