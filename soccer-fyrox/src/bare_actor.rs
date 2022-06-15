use crate::prelude::*;

#[my_actor_based]
pub struct BareActor {}

impl BareActor {
    pub fn new(
        img_base: &'static str,
        index: Option<u8>,
        anchor: Anchor,
        graph: &mut Graph,
    ) -> Self {
        let vpos = Vector2::new(0., 0.);

        let img_indexes = [index].iter().filter_map(|i| *i).collect();

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
