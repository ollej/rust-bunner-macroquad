use crate::{
    child_type::ChildType, grass::Grass, hedge::Hedge, hedge_mask::HedgeMask, hedge_row::HedgeRow,
    hedge_tile::HedgeTile, pavement::Pavement, player_state::PlayerState, position::Position,
    rail::Rail, resources::Resources, row::Row, row_type::RowType, ROW_HEIGHT, TILE_WIDTH, WIDTH,
};

use macroquad::audio::play_sound_once;
use macroquad::prelude::{collections::storage, debug, draw_texture, WHITE};
use macroquad::rand;

#[derive(Clone)]
pub struct Road {
    predecessor: Option<Box<RowType>>,
    index: i32,
    y: i32,
    children: Vec<ChildType>,
}

impl Row for Road {
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
        // TODO: Trigger sound effects
    }

    fn draw(&self, offset_x: i32, offset_y: i32) {
        let x = offset_x;
        let y = self.y + offset_y;
        let image = *storage::get::<Resources>()
            .road_textures
            .get(self.index as usize)
            .unwrap();
        draw_texture(image, x as f32, (y - ROW_HEIGHT) as f32, WHITE);

        for child in self.children() {
            child.draw(x, y);
        }
    }

    fn play_sound(&self) {
        play_sound_once(storage::get::<Resources>().road_sound);
    }

    fn next(&self) -> RowType {
        let predecessor = Some(Box::new(RowType::Road(self.clone())));
        let y = self.y - ROW_HEIGHT;
        if self.index == 0 {
            RowType::Road(Road::new(predecessor, 1, y))
        } else if self.index < 5 {
            let random = rand::gen_range::<u8>(0, 100);
            if random < 80 {
                RowType::Road(Road::new(predecessor, self.index + 1, y))
            } else if random < 88 {
                RowType::Grass(Grass::new(predecessor, rand::gen_range(0, 7), y))
            } else if random < 94 {
                RowType::Rail(Rail::new(predecessor, 0, y))
            } else {
                RowType::Pavement(Pavement::new(predecessor, 0, y))
            }
        } else {
            let random = rand::gen_range::<u8>(0, 100);
            if random < 60 {
                RowType::Grass(Grass::new(predecessor, rand::gen_range(0, 7), y))
            } else if random < 90 {
                RowType::Rail(Rail::new(predecessor, 0, y))
            } else {
                RowType::Pavement(Pavement::new(predecessor, 0, y))
            }
        }
    }

    fn allow_movement(&self, x: i32) -> bool {
        x >= 16 && x <= WIDTH - 16 && !self.collide(x, 8)
    }

    fn collide(&self, x: i32, margin: i32) -> bool {
        // TODO: Check against movers
        false
    }

    fn check_collision(&self, x: i32) -> PlayerState {
        if self.collide(x, 0) {
            play_sound_once(storage::get::<Resources>().splat_sound);
            PlayerState::Splat(0)
        } else {
            PlayerState::Alive
        }
    }

    fn push(&self) -> i32 {
        0
    }
}

impl Road {
    pub fn new(predecessor: Option<Box<RowType>>, index: i32, y: i32) -> Self {
        Self {
            predecessor,
            index,
            y,
            children: Vec::new(),
        }
    }
}
