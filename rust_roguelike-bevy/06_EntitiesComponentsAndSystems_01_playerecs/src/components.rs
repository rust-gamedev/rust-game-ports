pub use crate::prelude::*;

#[derive(Component)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Component)]
pub struct Player;

// Every component needs to be derived, so for external types, a wrapper type is needed.
#[derive(Component)]
pub struct PointC(pub Point);
