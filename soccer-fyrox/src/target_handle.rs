use crate::prelude::*;

// Stupid simple workaround for the source project duck typing.

#[derive(Clone, Copy)]
pub enum TargetHandle {
    None,
    Player(Handle<Player>),
    Goal(Handle<Goal>),
}

impl TargetHandle {
    // The is_*() methods could be replaced by making Any a supertrait of Targetable, but there are
    // tradeoff; in some cases, the object is not loaded, so the chain `load(&pool).as_any().is::<T>`,
    // while (in a way) more elegant, it's actually clunkier.
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

    pub fn load<'a>(&self, pools: &'a Pools) -> &'a dyn Target {
        match self {
            Self::Player(handle) => pools.players.borrow(*handle),
            Self::Goal(handle) => pools.goals.borrow(*handle),
            Self::None => panic!(),
        }
    }
}
