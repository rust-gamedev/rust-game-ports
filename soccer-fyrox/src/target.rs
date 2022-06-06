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

    // There's no trivial solution to this - instantiating each variant with the respective pool is
    // a nice idea, but requires either Rc's, that pollute the program with borrow()'s, or references,
    // which pollute the program with lifetimes.
    // Alternatively, players and goals could be stored in a single pool under a single trait, although
    // a mixed Pool type should be implemented (it's farly easy), otherwise, all the borrows require
    // downcasting (from Any), which is, again, very polluting.
    //
    pub fn vpos(&self, game: &Game) -> Vector2<f32> {
        match self {
            Self::Player(handle) => game.players_pool.borrow(*handle).vpos,
            Self::Goal(handle) => game.goals_pool.borrow(*handle).vpos,
            Self::None => panic!(),
        }
    }

    pub fn active(&self, game: &Game) -> bool {
        match self {
            Self::Player(handle) => game.players_pool.borrow(*handle).active(&game.ball),
            Self::Goal(handle) => game.goals_pool.borrow(*handle).active(&game.ball),
            Self::None => panic!(),
        }
    }
}
