use crate::prelude::*;

// Only Fov is currently required; a String works as well, but this is the clean approach.
//
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum StateLabel {
    Fov,
}
