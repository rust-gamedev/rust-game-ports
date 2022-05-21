use fyrox::{
    core::{algebra::Vector3, pool::Handle},
    scene::{
        base::BaseBuilder, dim2::rectangle::RectangleBuilder, graph::Graph, node::Node,
        transform::TransformBuilder,
    },
};

use crate::resources::Resources;

// Convenience trait to compactly load a texture (from the resources), and build a node.
// Could be merged into Resources for simplicity, but for conceptual clarity (their domain is different),
// we keep them separated.
// It's hard to find an exact name, since the concept bridges the Fyrox (Texture/Node) and the PyGame
// PyGame semantics (image).
//
pub trait ImageNodesBuilder {
    fn resources(&self) -> &Resources;

    fn build_image_node(&self, graph: &mut Graph, base: &str, i1: u8, i2: u8) -> Handle<Node> {
        let (texture, width, height) = self.resources().image(base, i1, i2);

        RectangleBuilder::new(
            BaseBuilder::new().with_local_transform(
                TransformBuilder::new()
                    .with_local_scale(Vector3::new(width, height, f32::EPSILON))
                    .build(),
            ),
        )
        .with_texture(texture)
        .build(graph)
    }
}
