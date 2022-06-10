use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, EnumIter, Eq, Hash, PartialEq)]
pub enum State {
    Menu,
    Play,
    GameOver,
}
