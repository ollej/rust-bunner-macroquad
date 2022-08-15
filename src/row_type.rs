use crate::{grass::Grass, pavement::Pavement, player_state::PlayerState, road::Road, row::Row};

#[derive(Clone)]
pub enum RowType {
    Grass(Grass),
    Road(Road),
    Pavement(Pavement),
}

impl RowType {
    pub fn check_collision(&self, x: i32) -> PlayerState {
        match self {
            RowType::Grass(grass) => grass.check_collision(x),
            RowType::Road(road) => road.check_collision(x),
            RowType::Pavement(pavement) => pavement.check_collision(x),
        }
    }

    pub fn y(&self) -> i32 {
        match self {
            RowType::Grass(grass) => grass.y(),
            RowType::Road(road) => road.y(),
            RowType::Pavement(pavement) => pavement.y(),
        }
    }

    pub fn allow_movement(&self, x: i32) -> bool {
        match self {
            RowType::Grass(grass) => grass.allow_movement(x),
            RowType::Road(road) => road.allow_movement(x),
            RowType::Pavement(pavement) => pavement.allow_movement(x),
        }
    }

    pub fn play_sound(&self) {
        match self {
            RowType::Grass(grass) => grass.play_sound(),
            RowType::Road(road) => road.play_sound(),
            RowType::Pavement(pavement) => pavement.play_sound(),
        }
    }

    pub fn next(&self) -> RowType {
        match self {
            RowType::Grass(grass) => grass.next(),
            RowType::Road(road) => road.next(),
            RowType::Pavement(pavement) => pavement.next(),
        }
    }

    pub fn update(&mut self) {
        match self {
            RowType::Grass(grass) => grass.update(),
            RowType::Road(road) => road.update(),
            RowType::Pavement(pavement) => pavement.update(),
        }
    }

    pub fn draw(&self, x: i32, y: i32) {
        match self {
            RowType::Grass(grass) => grass.draw(x, y),
            RowType::Road(road) => road.draw(x, y),
            RowType::Pavement(pavement) => pavement.draw(x, y),
        }
    }
}
