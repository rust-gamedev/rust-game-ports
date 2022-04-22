use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, pos: Point) {
    commands.spawn_bundle((
        Player,
        PointC(pos),
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
