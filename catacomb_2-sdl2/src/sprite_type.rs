use serdine::derive::Deserialize;

#[derive(Clone, Copy, Deserialize)]
pub struct spritetype {
    pub width: i16,
    pub height: i16,
    pub shapeptr: u32,
    pub maskptr: u32,
    pub xl: i16,
    pub yl: i16,
    pub xh: i16,
    pub yh: i16,
    pub name: [i8; 12],
}
