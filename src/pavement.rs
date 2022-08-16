use crate::{
    child_type::ChildType, grass::Grass, hedge::Hedge, hedge_mask::HedgeMask, hedge_row::HedgeRow,
    hedge_tile::HedgeTile, player_state::PlayerState, position::Position, resources::Resources,
    road::Road, row::Row, row_type::RowType, ROW_HEIGHT, TILE_WIDTH, WIDTH,
};

use macroquad::audio::play_sound_once;
use macroquad::prelude::{collections::storage, debug, draw_texture, WHITE};
use macroquad::rand;

#[derive(Clone)]
pub struct Pavement {
    predecessor: Option<Box<RowType>>,
    index: i32,
    y: i32,
    children: Vec<ChildType>,
}

impl Row for Pavement {
    fn y(&self) -> i32 {
        self.y
    }

    fn children(&self) -> &[ChildType] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<ChildType> {
        self.children.as_mut()
    }

    fn draw(&self, offset_x: i32, offset_y: i32) {
        let image = *storage::get::<Resources>()
            .side_textures
            .get(self.index as usize)
            .unwrap();
        let x = offset_x;
        let y = self.y + offset_y - image.height() as i32;
        draw_texture(image, x as f32, y as f32, WHITE);
    }

    fn play_sound(&self) {
        play_sound_once(storage::get::<Resources>().sidewalk_sound);
    }

    fn next(&self) -> RowType {
        let predecessor = Some(Box::new(RowType::Pavement(self.clone())));
        let y = self.y - ROW_HEIGHT;
        if self.index < 2 {
            RowType::Pavement(Pavement::new(predecessor, self.index + 1, y))
        } else {
            RowType::Road(Road::new(predecessor, 0, y))
        }
    }
}

impl Pavement {
    pub fn new(predecessor: Option<Box<RowType>>, index: i32, y: i32) -> Self {
        Self {
            predecessor,
            index,
            y,
            children: Vec::new(),
        }
    }
}
