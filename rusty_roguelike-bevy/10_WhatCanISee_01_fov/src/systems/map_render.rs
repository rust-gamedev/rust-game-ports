use crate::prelude::*;

pub fn map_render(
    player_fov_query: Query<&FieldOfView, With<Player>>,
    (map, camera): (Res<Map>, Res<Camera>),
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let player_fov = player_fov_query.single();

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) && player_fov.visible_tiles.contains(&pt) {
                let idx = map_idx(x, y);
                match map.tiles[idx] {
                    TileType::Floor => {
                        draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), to_cp437('.'));
                    }
                    TileType::Wall => {
                        draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), to_cp437('#'));
                    }
                }
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
