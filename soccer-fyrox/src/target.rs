use crate::prelude::*;

// Something that can be targeted/marked (either a Player or a Goal).
// This, along with TargetRef, is the static-type translation of duck typing in the source project.
//
pub trait Target: MyActor {}
