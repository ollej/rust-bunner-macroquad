use crate::{grass::Grass, player_state::PlayerState, row::Row};

#[derive(Clone)]
pub enum RowType {
    Grass(Grass),
}

impl RowType {
    pub fn check_collision(&self, x: i32) -> PlayerState {
        match self {
            RowType::Grass(grass) => grass.check_collision(x),
        }
    }

    pub fn y(&self) -> i32 {
        match self {
            RowType::Grass(grass) => grass.y(),
        }
    }

    pub fn allow_movement(&self, x: i32) -> bool {
        match self {
            RowType::Grass(grass) => grass.allow_movement(x),
        }
    }

    pub fn play_sound(&self) {
        match self {
            RowType::Grass(grass) => grass.play_sound(),
        }
    }

    pub fn next(&self) -> RowType {
        match self {
            RowType::Grass(grass) => grass.next(),
        }
    }

    pub fn update(&mut self) {
        match self {
            RowType::Grass(grass) => grass.update(),
        }
    }

    pub fn draw(&self, x: i32, y: i32) {
        match self {
            RowType::Grass(grass) => grass.draw(x, y),
        }
    }
}
