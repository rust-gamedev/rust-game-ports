use crate::prelude::*;
mod template;
use template::Templates;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.spawn().insert_bundle((
        Player { map_level: 0 },
        PointC(pos),
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(8),
    ));
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}

pub fn spawn_amulet_of_yala(world: &mut World, pos: Point) {
    world.spawn().insert_bundle((
        Item,
        AmuletOfYala,
        PointC(pos),
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}
