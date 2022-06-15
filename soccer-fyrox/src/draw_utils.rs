use fyrox::gui::image::{Image, ImageBuilder};
use fyrox::gui::widget::WidgetBuilder;
use fyrox::gui::{UiNode, UserInterface};
use fyrox::resource::texture::{Texture, TextureKind};
use fyrox::utils::into_gui_texture;

use crate::prelude::*;

use crate::anchor::Anchor;

// Draws the image (loads the texture, adds the node to the scene, and links it to the root).
//
// The coordinates ("std" = "standard") are the typical orientation used for 2d libraries (center
// at top left, x -> right, y -> down).
//
// This is difficult to name, since the semantics of Fyrox and (simple) 2d games are different.
//
pub fn add_image_node(
    media: &Media,
    scene: &mut Scene,
    base: &str,
    indexes: &[u8],
    std_x: f32,
    std_y: f32,
    z: f32,
    anchor: Anchor,
) {
    if base == BLANK_IMAGE {
        return;
    }

    let texture = media.image(base, indexes);
    let (fyrox_coords, texture_dims) = to_fyrox_coordinates(std_x, std_y, z, anchor, &texture);

    RectangleBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(fyrox_coords.x, fyrox_coords.y, z))
                .with_local_scale(Vector3::new(texture_dims.x, texture_dims.y, f32::EPSILON))
                .build(),
        ),
    )
    .with_texture(texture)
    .build(&mut scene.graph);
}

pub fn build_blank_widget(
    media: &Media,
    base: &str,
    indexes: &[u8],
    x: f32,
    y: f32,
    user_interface: &mut UserInterface,
) -> Handle<UiNode> {
    let texture_kind = media.image(base, indexes).data_ref().kind();

    if let TextureKind::Rectangle {
        width: texture_width,
        height: texture_height,
    } = texture_kind
    {
        let widget_h = ImageBuilder::new(
            WidgetBuilder::new()
                .with_width(texture_width as f32)
                .with_height(texture_height as f32)
                .with_desired_position(Vector2::new(x, y))
                .with_opacity(Some(0.0)),
        )
        .build(&mut user_interface.build_ctx());

        widget_h
    } else {
        panic!()
    }
}

pub fn draw_widget(
    widget_h: Handle<UiNode>,
    media: &Media,
    base: &str,
    indexes: &[u8],
    x: f32,
    y: f32,
    user_interface: &mut UserInterface,
) {
    let texture = media.image(base, indexes);

    let mut context = user_interface.build_ctx();
    let widget = context
        .try_get_node_mut(widget_h)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<Image>()
        .unwrap();

    widget.set_opacity(Some(1.0));
    widget.set_desired_local_position(Vector2::new(x, y));
    widget.set_texture(into_gui_texture(texture));
}

pub fn disable_widget(widget_h: Handle<UiNode>, user_interface: &mut UserInterface) {
    let mut context = user_interface.build_ctx();
    let widget = context
        .try_get_node_mut(widget_h)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<Image>()
        .unwrap();

    widget.set_opacity(Some(0.0));
}

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
