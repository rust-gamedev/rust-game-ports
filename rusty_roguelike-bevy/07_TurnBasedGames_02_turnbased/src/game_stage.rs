use crate::prelude::*;

// From our game design perspective, GameStage is a group of systems that require the commands in the
// previous group to be flushed.
// See `mod.rs`.
//
#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    // The first stage is the standard Update
    MovePlayer,
    PlayerCollisions,
    MoveMonsters,
    MonsterCollisions,
}
