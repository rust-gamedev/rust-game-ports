use fyrox::{core::algebra::Vector2, scene::Scene};

use crate::{anchor::Anchor, media::Media};

//# The MyActor class extends Pygame Zero's Actor class by providing the attribute 'vpos', which stores the object's
//# current position using Pygame's Vector2 class. All code should change or read the position via vpos, as opposed to
//# Actor's x/y or pos attributes. When the object is drawn, we set self.pos (equivalent to setting both self.x and
//# self.y) based on vpos, but taking scrolling into account.
pub trait MyActor {
    fn vpos(&self) -> Vector2<f32>;
    fn img_base(&self) -> &'static str;
    fn img_indexes(&self) -> &[u8];
    fn anchor(&self) -> Anchor;

    //# We draw with the supplied offset to enable scrolling
    fn draw(&self, scene: &mut Scene, media: &mut Media, offset_x: f32, offset_y: f32, z: f32) {
        let pos_x = self.vpos().x - offset_x;
        let pos_y = self.vpos().y - offset_y;

        media.draw_image(
            scene,
            self.img_base(),
            &self.img_indexes(),
            pos_x,
            pos_y,
            z,
            self.anchor(),
        );
    }
}
