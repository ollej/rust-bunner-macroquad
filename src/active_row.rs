use crate::{child::Child, position::Position, WIDTH};
use macroquad::rand;

pub trait ActiveRow {
    fn build_children(dx: i32) -> Vec<Child>
    where
        Self: Sized,
    {
        let mut children = Vec::new();
        let mut x = -WIDTH / 2 - 70;
        while x < WIDTH / 2 + 70 {
            x += rand::gen_range::<i32>(240, 481);
            let position = if dx > 0 {
                Position::new(WIDTH / 2 + x, 0)
            } else {
                Position::new(WIDTH / 2 - x, 0)
            };
            children.push(Self::build_child(dx, position));
        }
        children
    }

    fn build_child(dx: i32, position: Position) -> Child
    where
        Self: Sized;
}
