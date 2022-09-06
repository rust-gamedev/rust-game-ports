#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PlayerState {
    Alive,
    Splat(i32),
    Splash,
    Eagle(i32),
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Alive
    }
}
