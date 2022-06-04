use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum TargetRef {
    None,
    Player(Handle<Player>),
    Goal(Handle<Goal>),
}
