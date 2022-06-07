use crate::prelude::*;

// Stupid simple workaround for the source project duck typing.

#[derive(Clone, Copy)]
pub enum Target {
    None,
    Player(Handle<Player>),
    Goal(Handle<Goal>),
}

impl Target {
    pub fn is_goal(&self) -> bool {
        match self {
            Self::Player(_) => false,
            Self::Goal(_) => true,
            Self::None => panic!(),
        }
    }

    pub fn is_player(&self) -> bool {
        match self {
            Self::Player(_) => true,
            Self::Goal(_) => false,
            Self::None => panic!(),
        }
    }

    pub fn load<'a>(&self, pools: &'a Pools) -> &'a dyn Targetable {
        match self {
            Self::Player(handle) => pools.players.borrow(*handle),
            Self::Goal(handle) => pools.goals.borrow(*handle),
            Self::None => panic!(),
        }
    }
}
