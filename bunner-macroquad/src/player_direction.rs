#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayerDirection {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Default for PlayerDirection {
    fn default() -> Self {
        PlayerDirection::Down
    }
}
