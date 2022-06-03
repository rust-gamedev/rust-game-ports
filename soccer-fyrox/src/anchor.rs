use fyrox::core::algebra::Vector2;

#[derive(Clone, Copy)]
pub enum Anchor {
    Center,
    Custom(Vector2<i16>),
}
