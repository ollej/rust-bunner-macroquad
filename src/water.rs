use crate::{
    child_type::ChildType, dirt::Dirt, grass::Grass, hedge::Hedge, hedge_mask::HedgeMask,
    hedge_row::HedgeRow, hedge_tile::HedgeTile, player_state::PlayerState, position::Position,
    resources::Resources, road::Road, row::Row, row_type::RowType, ROW_HEIGHT, TILE_WIDTH, WIDTH,
};

use macroquad::audio::play_sound_once;
use macroquad::prelude::{collections::storage, debug, draw_texture, WHITE};
use macroquad::rand;

#[derive(Clone)]
pub struct Water {
    predecessor: Option<Box<RowType>>,
    index: i32,
    y: i32,
    children: Vec<ChildType>,
}

impl Row for Water {
    fn y(&self) -> i32 {
        self.y
    }

    fn children(&self) -> &[ChildType] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<ChildType> {
        self.children.as_mut()
    }

    fn update(&mut self, scroll_pos: i32) {
        // TODO: super update
        // TODO: update logs
    }

    fn draw(&self, offset_x: i32, offset_y: i32) {
        let image = *storage::get::<Resources>()
            .water_textures
            .get(self.index as usize)
            .unwrap();
        let x = offset_x;
        let y = self.y + offset_y - image.height() as i32;
        draw_texture(image, x as f32, y as f32, WHITE);
    }

    fn play_sound(&self) {
        play_sound_once(storage::get::<Resources>().log_sound);
    }

    fn next(&self) -> RowType {
        let predecessor = Some(Box::new(RowType::Water(self.clone())));
        let y = self.y - ROW_HEIGHT;
        if self.index == 7 || (self.index >= 1 && rand::gen_range(0, 2) == 0) {
            RowType::Dirt(Dirt::new(predecessor, rand::gen_range(4, 7), y))
        } else {
            RowType::Water(Water::new(predecessor, self.index + 1, y))
        }
    }

    // TODO: implement push()
}

impl Water {
    pub fn new(predecessor: Option<Box<RowType>>, index: i32, y: i32) -> Self {
        Self {
            predecessor,
            index,
            y,
            children: Vec::new(),
        }
    }
}
