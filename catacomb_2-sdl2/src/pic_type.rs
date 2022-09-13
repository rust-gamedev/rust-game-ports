use serdine::derive::Deserialize;

#[derive(Clone, Copy, Deserialize)]
pub struct pictype {
    pub width: i16,
    pub height: i16,
    pub shapeptr: u32,
    pub name: [i8; 8],
}
