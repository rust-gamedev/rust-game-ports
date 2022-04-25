use crate::prelude::*;

// Differently from the source project, which uses resources, we use Bevy's messaging system for move
// messages.
// In the context of this project, it's a bit more ergonomic, but in larger ones, there advantages are
// more significant.
pub struct WantsToMove {
    pub entity: Entity,
    // Event type fields don't need to be components; in this case we don't need to use PointC, but
    // it can be trivially done.
    pub destination: Point,
}
