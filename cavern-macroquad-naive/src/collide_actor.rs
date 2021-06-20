use crate::actor::Actor;

pub trait CollideActor: Actor {
    fn move_(&mut self, _dx: i32, _dy: i32, _speed: i32) -> bool {
        eprintln!("WRITEME: CollideActor#move");
        true
    }
}
