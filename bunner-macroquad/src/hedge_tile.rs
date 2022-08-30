#[derive(Copy, Clone, Eq, PartialEq)]
pub enum HedgeTile {
    SingleWidth = 0,
    LeftMost = 1,
    RightMost = 2,
    Middle3 = 3,
    Middle4 = 4,
    Middle5 = 5,
    Grass = 6,
}
