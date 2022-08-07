#[derive(Clone, Copy, PartialEq)]
pub enum PlayerState {
    Alive,
    Splat,
    Splash,
    Eagle,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Alive
    }
}
