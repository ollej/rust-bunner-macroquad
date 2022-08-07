#[derive(Clone, Copy, PartialEq)]
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
