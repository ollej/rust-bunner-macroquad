use crate::player_state::PlayerState;

#[derive(Default)]
pub struct Bunner {
    pub state: PlayerState,
    pub timer: i32,
}

impl Bunner {
    pub fn new() -> Self {
        Self {
            state: PlayerState::Alive,
            timer: 0,
        }
    }

    pub fn update(&mut self) {}

    pub fn draw(&self) {}
}
