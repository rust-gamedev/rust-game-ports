pub trait Actor {
    // Info necessary to draw: name+indexes, x, y.
    //
    fn draw_info(&self) -> (&'static str, Vec<u8>, i16, i16);
}
