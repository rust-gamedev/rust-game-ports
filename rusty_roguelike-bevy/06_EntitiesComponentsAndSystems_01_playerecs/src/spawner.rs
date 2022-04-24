use crate::prelude::*;

pub fn spawn_player(world: &mut World, pos: Point) {
    world.spawn().insert_bundle((
        Player,
        PointC(pos),
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
