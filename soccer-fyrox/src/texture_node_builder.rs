use fyrox::{
    core::{algebra::Vector3, pool::Handle},
    resource::texture::Texture,
    scene::{
        base::BaseBuilder, dim2::rectangle::RectangleBuilder, graph::Graph, node::Node,
        transform::TransformBuilder,
    },
};

// Convenience function to build a node, using data typically provided from Resources#image.
// Could be merged into Resources for simplicity, but for conceptual clarity (their domain is different),
// we keep them separated.
// It's hard to find an exact name, since the concept bridges the Fyrox (Texture/Node) and the PyGame
// PyGame semantics (image).
//
pub fn build_image_node(
    graph: &mut Graph,
    image_data: (Texture, f32, f32),
    x: i16,
    y: i16,
    z: i16,
) -> Handle<Node> {
    let (texture, width, height) = image_data;

    RectangleBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_scale(Vector3::new(width, height, f32::EPSILON))
                .with_local_position(Vector3::new(x as f32, y as f32, z as f32))
                .build(),
        ),
    )
    .with_texture(texture)
    .build(graph)
}
