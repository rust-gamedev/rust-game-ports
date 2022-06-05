use fyrox::core::algebra::Vector2;

#[derive(Clone, Copy)]
pub enum Anchor {
    Center,
    // TopLeft is equivalent to a Custom anchored at (0,0).
    TopLeft,
    Custom(Vector2<f32>),
}
