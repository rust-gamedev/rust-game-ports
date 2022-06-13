use fyrox::resource::texture::{Texture, TextureKind};

use crate::prelude::*;

use crate::anchor::Anchor;

pub fn to_fyrox_coordinates(
    std_x: f32,
    std_y: f32,
    z: f32,
    anchor: Anchor,
    texture: &Texture,
) -> (Vector3<f32>, Vector2<f32>) {
    let texture_kind = texture.data_ref().kind();

    if let TextureKind::Rectangle {
        width: texture_width,
        height: texture_height,
    } = texture_kind
    {
        use Anchor::*;
        let (texture_width, texture_height) = (texture_width as f32, texture_height as f32);

        // As a base, we start with the top left corner of the screen, and we subtract the "standard"
        // coordinates, since they go to the opposite direction to the Fyrox ones.
        //
        let (mut fyrox_x, mut fyrox_y) = (WIDTH / 2. - std_x, HEIGHT / 2. - std_y);

        match anchor {
            Center => {
                // Do nothing
            }
            TopLeft => {
                // Shift the texture, to the bottom right, of half texture.
                //
                fyrox_x = fyrox_x - texture_width / 2.;
                fyrox_y = fyrox_y - texture_height / 2.;
            }
            Custom(anchor) => {
                // Shift bottom right like TopLeft, then shift top left according to the anchor.
                //
                fyrox_x = fyrox_x - texture_width / 2. + anchor.x;
                fyrox_y = fyrox_y - texture_height / 2. + anchor.y;
            }
        };

        (
            Vector3::new(fyrox_x, fyrox_y, z),
            Vector2::new(texture_width, texture_height),
        )
    } else {
        panic!("Texture is not a rectangle!")
    }
}
