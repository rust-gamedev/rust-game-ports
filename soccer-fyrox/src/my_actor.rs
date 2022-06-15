use crate::prelude::*;

//# The MyActor class extends Pygame Zero's Actor class by providing the attribute 'vpos', which stores the object's
//# current position using Pygame's Vector2 class. All code should change or read the position via vpos, as opposed to
//# Actor's x/y or pos attributes. When the object is drawn, we set self.pos (equivalent to setting both self.x and
//# self.y) based on vpos, but taking scrolling into account.
pub trait MyActor {
    fn vpos(&self) -> Vector2<f32>;
    fn vpos_mut(&mut self) -> &mut Vector2<f32>;
    fn img_base(&self) -> &'static str;
    fn img_indexes(&self) -> &[u8];
    fn anchor(&self) -> Anchor;
    fn rectangle_h(&self) -> Handle<Node>;

    fn prepare_draw(&self, scene: &mut Scene, media: &mut Media, z: f32) {
        let texture = media.image(self.img_base(), &self.img_indexes());
        let (fyrox_coords, texture_dims) =
            to_fyrox_coordinates(self.vpos().x, self.vpos().y, z, self.anchor(), &texture);

        let frame = scene.graph[self.rectangle_h()].as_rectangle_mut();

        frame.set_texture(Some(texture));
        frame.set_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(fyrox_coords.x, fyrox_coords.y, z))
                .with_local_scale(Vector3::new(texture_dims.x, texture_dims.y, f32::EPSILON))
                .build(),
        );
    }
}
