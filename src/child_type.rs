use crate::{actor::Actor, hedge::Hedge};

#[derive(Clone)]
pub enum ChildType {
    Hedge(Hedge),
}

impl ChildType {
    pub fn update(&mut self) {
        match self {
            ChildType::Hedge(hedge) => hedge.update(),
        }
    }

    pub fn draw(&self, x: i32, y: i32) {
        match self {
            ChildType::Hedge(hedge) => hedge.draw(x, y),
        }
    }

    pub fn x(&self) -> i32 {
        return match self {
            ChildType::Hedge(hedge) => hedge.x(),
        };
    }
}
