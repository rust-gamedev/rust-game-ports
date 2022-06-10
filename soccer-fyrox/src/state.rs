use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, EnumIter, PartialEq)]
pub enum State {
    Menu,
    Play,
    GameOver,
}
