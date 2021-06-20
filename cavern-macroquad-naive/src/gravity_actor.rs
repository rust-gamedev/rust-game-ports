use crate::collide_actor::CollideActor;

pub trait GravityActor: CollideActor {
    fn vel_y(&self) -> i32;
    fn vel_y_mut(&mut self) -> &mut i32;
    fn landed(&self) -> bool;
    fn landed_mut(&mut self) -> &mut bool;

    fn update(&mut self) {
        eprintln!("WRITEME: GravityActor#update");
    }
}
