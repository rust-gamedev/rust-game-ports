use crate::prelude::*;

#[derive(Component)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct MovingRandomly;

// Every component needs to be derived, so for external types, a wrapper type is needed.
#[derive(Component)]
pub struct PointC(pub Point);
