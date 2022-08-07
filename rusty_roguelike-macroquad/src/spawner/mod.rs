mod template;

use crate::prelude::*;
use template::Templates;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: WHITE,
            sprite: TileSet::SPRITE_PLAYER,
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(8),
        Damage(1),
    ));
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: WHITE,
            sprite: TileSet::SPRITE_AMULET,
        },
        Name("Amulet of Yala".to_string()),
    ));
}

pub async fn spawn_level(ecs: &mut World, level: usize, spawn_points: &[Point]) {
    let template = Templates::load().await;
    template.spawn_entities(ecs, level, spawn_points);
}
