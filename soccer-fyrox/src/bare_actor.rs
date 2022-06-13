use crate::prelude::*;

#[my_actor_based]
pub struct BareActor {}

impl BareActor {
    pub fn new(img_base: &'static str, anchor: Anchor, graph: &mut Graph) -> Self {
        let vpos = Vector2::new(0., 0.);

        let img_indexes = vec![];

        let rectangle_h = RectangleBuilder::new(BaseBuilder::new()).build(graph);

        Self {
            vpos,
            img_base,
            img_indexes,
            anchor,
            rectangle_h,
        }
    }
}
