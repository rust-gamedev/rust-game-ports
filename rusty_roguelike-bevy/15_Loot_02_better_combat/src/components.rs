use crate::prelude::*;
use std::collections::HashSet;

// Every component needs to be derived, so for external types, a wrapper type is needed.
#[derive(Component)]
pub struct PointC(pub Point);

#[derive(Component)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Component)]
pub struct Player {
    pub map_level: u32,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct AmuletOfYala;

#[derive(Component)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Component)]
pub struct ProvidesDungeonMap;

#[derive(Component)]
pub struct MovingRandomly;

#[derive(Component)]
pub struct ChasingPlayer;

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Carried(pub Entity);

#[derive(Component)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}
