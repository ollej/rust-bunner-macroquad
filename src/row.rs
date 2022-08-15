use crate::{
    actor::Actor, child_type::ChildType, player_state::PlayerState, position::Position,
    resources::Resources, row_type::RowType, TILE_WIDTH, WIDTH,
};

pub trait Row {
    fn y(&self) -> i32;
    fn children(&self) -> &[ChildType];
    fn children_mut(&mut self) -> &mut Vec<ChildType>;

    fn update(&mut self) {
        for child in self.children_mut().iter_mut() {
            child.update();
        }
    }

    fn draw(&self, offset_x: i32, offset_y: i32);

    fn play_sound(&self);

    fn next(&self) -> RowType;

    fn check_collision(&self, x: i32) -> PlayerState {
        PlayerState::Alive
    }

    fn allow_movement(&self, x: i32) -> bool {
        x >= 16 && x <= WIDTH - 16
    }

    fn collide(&self, x: i32, margin: i32) -> bool {
        for child in self.children().iter() {
            if x >= (child.x() - TILE_WIDTH / 2 - margin)
                && x < (child.x() + TILE_WIDTH / 2 + margin)
            {
                return true;
            }
        }
        false
    }

    fn push(&self) -> i32 {
        0
    }
}
