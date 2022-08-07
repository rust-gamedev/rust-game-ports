use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(
    ecs: &SubWorld,
    #[resource] camera: &CameraView,
    #[resource] tileset: &TileSet,
) {
    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let offset = Point::new(camera.left_x, camera.top_y);
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            let local_pos = *pos - offset;
            tileset.draw_tile(render.sprite, render.color, local_pos.x, local_pos.y);
        });
}
