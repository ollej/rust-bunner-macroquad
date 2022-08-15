use crate::{
    actor::Actor, hedge_row::HedgeRow, hedge_tile::HedgeTile, position::Position,
    resources::Resources,
};
use macroquad::prelude::{collections::storage, draw_texture, WHITE};

#[derive(Clone)]
pub struct Hedge {
    hedge_tile: HedgeTile,
    hedge_row: HedgeRow,
    position: Position,
}

impl Actor for Hedge {
    fn update(&mut self) {}

    fn draw(&self, offset_x: i32, offset_y: i32) {
        let x = self.position.x + offset_x - 20;
        let y = self.position.y + offset_y;
        let image = *storage::get::<Resources>()
            .bush_textures
            .get(self.hedge_tile as usize * 2 + self.hedge_row as usize)
            .unwrap();
        draw_texture(image, x as f32, y as f32, WHITE);
    }

    fn x(&self) -> i32 {
        self.position.x
    }

    fn y(&self) -> i32 {
        self.position.y
    }
}

impl Hedge {
    pub fn new(hedge_tile: HedgeTile, hedge_row: HedgeRow, position: Position) -> Self {
        Self {
            hedge_tile,
            hedge_row,
            position,
        }
    }
}
