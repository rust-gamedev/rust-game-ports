// This has a long explanation; see systems/mods.rs.
//
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameStep {
    AwaitingInput,
    MovePlayer,
    Collisions,
    GenerateMonsterMoves,
    MoveMonsters,
}
