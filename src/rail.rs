use crate::{
    child_type::ChildType, grass::Grass, hedge::Hedge, hedge_mask::HedgeMask, hedge_row::HedgeRow,
    hedge_tile::HedgeTile, player_state::PlayerState, position::Position, resources::Resources,
    road::Road, row::Row, row_type::RowType, water::Water, ROW_HEIGHT, TILE_WIDTH, WIDTH,
};

use macroquad::audio::play_sound_once;
use macroquad::prelude::{collections::storage, debug, draw_texture, WHITE};
use macroquad::rand;

#[derive(Clone)]
pub struct Rail {
    predecessor: Option<Box<RowType>>,
    index: i32,
    y: i32,
    children: Vec<ChildType>,
}

impl Row for Rail {
    fn y(&self) -> i32 {
        self.y
    }

    fn children(&self) -> &[ChildType] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<ChildType> {
        self.children.as_mut()
    }

    fn update(&mut self) {
        // TODO: super update
        // TODO: Show train
    }

    fn draw(&self, offset_x: i32, offset_y: i32) {
        let image = *storage::get::<Resources>()
            .rail_textures
            .get(self.index as usize)
            .unwrap();
        let x = offset_x;
        let y = self.y + offset_y - image.height() as i32;
        draw_texture(image, x as f32, y as f32, WHITE);
    }

    fn play_sound(&self) {
        play_sound_once(storage::get::<Resources>().grass_sound);
    }

    fn next(&self) -> RowType {
        let predecessor = Some(Box::new(RowType::Rail(self.clone())));
        let y = self.y - ROW_HEIGHT;
        if self.index < 3 {
            RowType::Rail(Rail::new(predecessor, self.index + 1, y))
        } else {
            if rand::gen_range::<u8>(0, 2) == 0 {
                RowType::Road(Road::new(predecessor, 0, y))
            } else {
                RowType::Water(Water::new(predecessor, 0, y))
            }
        }
    }
}

impl Rail {
    pub fn new(predecessor: Option<Box<RowType>>, index: i32, y: i32) -> Self {
        Self {
            predecessor,
            index,
            y,
            children: Vec::new(),
        }
    }
}
