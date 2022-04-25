use crate::prelude::*;

pub fn entity_render(query: Query<(&PointC, &Render)>, camera: Res<Camera>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    for (pos, render) in query.iter() {
        draw_batch.set(pos.0 - offset, render.color, render.glyph);
    }
    draw_batch.submit(5000).expect("Batch error");
}
