use crate::prelude::*;

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType, idx: usize) -> Sprite {
        match tile_type {
            TileType::Floor => RANDOM_FLOOR_TILES[idx],
            TileType::Wall => TileSet::SPRITE_WALL,
            TileType::Exit => TileSet::SPRITE_STAIRS,
        }
    }
}

pub struct ForestTheme {}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType, idx: usize) -> Sprite {
        match tile_type {
            TileType::Floor => TileSet::SPRITE_GROUND,
            TileType::Wall => RANDOM_TREE_TILES[idx],
            TileType::Exit => TileSet::SPRITE_STAIRS,
        }
    }
}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}
