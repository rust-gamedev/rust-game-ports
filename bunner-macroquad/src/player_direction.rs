#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayerDirection {
    Up,
    Right,
    Down,
    Left,
}

impl Default for PlayerDirection {
    fn default() -> Self {
        PlayerDirection::Down
    }
}
