use crate::{
    child_type::ChildType, dirt::Dirt, grass::Grass, pavement::Pavement, player_state::PlayerState,
    rail::Rail, road::Road, row::Row, water::Water,
};

#[derive(Clone)]
pub enum RowType {
    Grass(Grass),
    Road(Road),
    Pavement(Pavement),
    Rail(Rail),
    Water(Water),
    Dirt(Dirt),
}

impl RowType {
    pub fn check_collision(&self, x: i32) -> PlayerState {
        match self {
            RowType::Grass(grass) => grass.check_collision(x),
            RowType::Road(road) => road.check_collision(x),
            RowType::Pavement(pavement) => pavement.check_collision(x),
            RowType::Rail(rail) => rail.check_collision(x),
            RowType::Water(water) => water.check_collision(x),
            RowType::Dirt(dirt) => dirt.check_collision(x),
        }
    }

    pub fn y(&self) -> i32 {
        match self {
            RowType::Grass(grass) => grass.y(),
            RowType::Road(road) => road.y(),
            RowType::Pavement(pavement) => pavement.y(),
            RowType::Rail(rail) => rail.y(),
            RowType::Water(water) => water.y(),
            RowType::Dirt(dirt) => dirt.y(),
        }
    }

    pub fn allow_movement(&self, x: i32) -> bool {
        match self {
            RowType::Grass(grass) => grass.allow_movement(x),
            RowType::Road(road) => road.allow_movement(x),
            RowType::Pavement(pavement) => pavement.allow_movement(x),
            RowType::Rail(rail) => rail.allow_movement(x),
            RowType::Water(water) => water.allow_movement(x),
            RowType::Dirt(dirt) => dirt.allow_movement(x),
        }
    }

    pub fn collide(&self, x: i32, margin: i32) -> bool {
        match self {
            RowType::Grass(grass) => grass.collide(x, margin),
            RowType::Road(road) => road.collide(x, margin),
            RowType::Pavement(pavement) => pavement.collide(x, margin),
            RowType::Rail(rail) => rail.collide(x, margin),
            RowType::Water(water) => water.collide(x, margin),
            RowType::Dirt(dirt) => dirt.collide(x, margin),
        }
    }

    pub fn play_sound(&self) {
        match self {
            RowType::Grass(grass) => grass.play_sound(),
            RowType::Road(road) => road.play_sound(),
            RowType::Pavement(pavement) => pavement.play_sound(),
            RowType::Rail(rail) => rail.play_sound(),
            RowType::Water(water) => water.play_sound(),
            RowType::Dirt(dirt) => dirt.play_sound(),
        }
    }

    pub fn next(&self) -> RowType {
        match self {
            RowType::Grass(grass) => grass.next(),
            RowType::Road(road) => road.next(),
            RowType::Pavement(pavement) => pavement.next(),
            RowType::Rail(rail) => rail.next(),
            RowType::Water(water) => water.next(),
            RowType::Dirt(dirt) => dirt.next(),
        }
    }

    pub fn update(&mut self, scroll_pos: i32) {
        match self {
            RowType::Grass(grass) => grass.update(scroll_pos),
            RowType::Road(road) => road.update(scroll_pos),
            RowType::Pavement(pavement) => pavement.update(scroll_pos),
            RowType::Rail(rail) => rail.update(scroll_pos),
            RowType::Water(water) => water.update(scroll_pos),
            RowType::Dirt(dirt) => dirt.update(scroll_pos),
        }
    }

    pub fn draw(&self, x: i32, y: i32) {
        match self {
            RowType::Grass(grass) => grass.draw(x, y),
            RowType::Road(road) => road.draw(x, y),
            RowType::Pavement(pavement) => pavement.draw(x, y),
            RowType::Rail(rail) => rail.draw(x, y),
            RowType::Water(water) => water.draw(x, y),
            RowType::Dirt(dirt) => dirt.draw(x, y),
        }
    }

    pub fn children_mut(&mut self) -> &mut Vec<ChildType> {
        match self {
            RowType::Grass(grass) => grass.children_mut(),
            RowType::Road(road) => road.children_mut(),
            RowType::Pavement(pavement) => pavement.children_mut(),
            RowType::Rail(rail) => rail.children_mut(),
            RowType::Water(water) => water.children_mut(),
            RowType::Dirt(dirt) => dirt.children_mut(),
        }
    }
}
