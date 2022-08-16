use crate::{actor::Actor, hedge::Hedge, train::Train};

#[derive(Clone)]
pub enum ChildType {
    Hedge(Hedge),
    Train(Train),
}

impl ChildType {
    pub fn update(&mut self) {
        match self {
            ChildType::Hedge(hedge) => hedge.update(),
            ChildType::Train(train) => train.update(),
        }
    }

    pub fn draw(&self, x: i32, y: i32) {
        match self {
            ChildType::Hedge(hedge) => hedge.draw(x, y),
            ChildType::Train(train) => train.draw(x, y),
        }
    }

    pub fn x(&self) -> i32 {
        return match self {
            ChildType::Hedge(hedge) => hedge.x(),
            ChildType::Train(train) => train.x(),
        };
    }

    pub fn width(&self) -> i32 {
        return match self {
            ChildType::Hedge(hedge) => hedge.width(),
            ChildType::Train(train) => train.width(),
        };
    }
}
