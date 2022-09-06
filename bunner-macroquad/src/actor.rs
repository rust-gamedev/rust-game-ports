pub trait Actor {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn width(&self) -> i32;
    fn update(&mut self);
    fn draw(&self, offset_x: i32, offset_y: i32);
}
