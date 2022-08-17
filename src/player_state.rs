use crate::player_direction::PlayerDirection;

#[derive(Clone, Copy, PartialEq)]
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
