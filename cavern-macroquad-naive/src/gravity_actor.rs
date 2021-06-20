use crate::collide_actor::CollideActor;

pub trait GravityActor: CollideActor {
    fn update(&mut self) {
        eprintln!("WRITEME: GravityActor#update");
    }
}
