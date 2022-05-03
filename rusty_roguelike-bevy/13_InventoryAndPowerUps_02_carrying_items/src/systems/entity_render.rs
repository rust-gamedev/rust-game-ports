use crate::prelude::*;

pub fn entity_render(
    renderables: Query<(&PointC, &Render)>,
    player_fov_query: Query<&FieldOfView, With<Player>>,
    camera: Res<Camera>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    let player_fov = player_fov_query.single();

    for (pos, render) in renderables.iter() {
        if player_fov.visible_tiles.contains(&pos.0) {
            draw_batch.set(pos.0 - offset, render.color, render.glyph);
        }
    }

    draw_batch.submit(5000).expect("Batch error");
}
