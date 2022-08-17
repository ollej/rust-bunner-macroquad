use crate::{
    child_type::ChildType, hedge::Hedge, hedge_mask::HedgeMask, hedge_row::HedgeRow,
    hedge_tile::HedgeTile, player_state::PlayerState, position::Position, resources::Resources,
    road::Road, row::Row, row_type::RowType, water::Water, ROW_HEIGHT, TILE_WIDTH, WIDTH,
};

use macroquad::audio::play_sound_once;
use macroquad::prelude::{collections::storage, debug, draw_texture, WHITE};
use macroquad::rand;

#[derive(Clone)]
pub struct Grass {
    index: i32,
    y: i32,
    hedge_row: HedgeRow,
    hedge_mask: Vec<HedgeMask>,
    children: Vec<ChildType>,
}

impl Row for Grass {
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
            .grass_textures
            .get(self.index as usize)
            .unwrap();
        let x = offset_x;
        let y = self.y + offset_y as i32;
        draw_texture(image, x as f32, y as f32 - image.height(), WHITE);
        self.draw_children(x, y);
    }

    fn play_sound(&self) {
        play_sound_once(storage::get::<Resources>().grass_sound);
    }

    fn next(&self) -> RowType {
        let y = self.y - ROW_HEIGHT;
        if self.index <= 5 {
            self.grass_row(self.index + 8, y)
        } else if self.index == 6 {
            self.grass_row(7, y)
        } else if self.index == 7 {
            self.grass_row(15, y)
        } else if self.index >= 8 && self.index <= 14 {
            self.grass_row(self.index + 1, y)
        } else {
            if rand::gen_range::<u8>(0, 2) == 0 {
                RowType::Road(Road::empty(y))
            } else {
                RowType::Water(Water::empty(y))
            }
        }
    }

    fn allow_movement(&self, x: i32) -> bool {
        x >= 16 && x <= WIDTH - 16 && !self.collide(x, 8)
    }

    fn push(&self) -> i32 {
        0
    }
}

impl Grass {
    pub fn new(
        previous_hedge_mask: Option<Vec<HedgeMask>>,
        previous_hedge_row: HedgeRow,
        index: i32,
        y: i32,
    ) -> Self {
        let (hedge_mask, hedge_row) =
            if previous_hedge_mask.is_none() || previous_hedge_row == HedgeRow::None {
                Self::first_hedge_row(index)
            } else if previous_hedge_row == HedgeRow::First {
                (previous_hedge_mask.unwrap(), HedgeRow::Second)
            } else {
                (Vec::new(), HedgeRow::None)
            };

        let mut children: Vec<ChildType> = Vec::new();
        if hedge_row != HedgeRow::None {
            // See comments in classify_hedge_segment for explanation of previous_mid_segment
            let mut hedge_tile = HedgeTile::Grass;
            let mut previous_mid_segment = None;
            for i in 1..13 {
                (hedge_tile, previous_mid_segment) =
                    Self::classify_hedge_segment(&hedge_mask[i - 1..i + 2], previous_mid_segment);
                if hedge_tile != HedgeTile::Grass {
                    children.push(ChildType::Hedge(Hedge::new(
                        hedge_tile,
                        hedge_row,
                        Position::new(i as i32 * 40 - 20, 0),
                    )));
                }
            }
        }

        Self {
            y,
            index,
            hedge_row,
            hedge_mask,
            children,
        }
    }

    pub fn without_hedge(index: i32, y: i32) -> Self {
        Self::new(None, HedgeRow::None, index, y)
    }

    pub fn classify_hedge_segment(
        mask_window: &[HedgeMask],
        previous_mid_segment: Option<HedgeTile>,
    ) -> (HedgeTile, Option<HedgeTile>) {
        if mask_window[1] == HedgeMask::Empty {
            (HedgeTile::Grass, None)
        } else if mask_window[0] == HedgeMask::Empty && mask_window[2] == HedgeMask::Empty {
            (HedgeTile::SingleWidth, None)
        } else if mask_window[0] == HedgeMask::Empty {
            (HedgeTile::LeftMost, None)
        } else if mask_window[2] == HedgeMask::Empty {
            (HedgeTile::RightMost, None)
        } else {
            match previous_mid_segment {
                Some(HedgeTile::Middle4) if mask_window[2] == HedgeMask::Empty => {
                    (HedgeTile::Middle5, None)
                }
                Some(HedgeTile::Middle4) if mask_window[2] == HedgeMask::Hedge => {
                    (HedgeTile::Middle3, Some(HedgeTile::Middle3))
                }
                Some(HedgeTile::Middle3) => (HedgeTile::Middle3, Some(HedgeTile::Middle3)),
                _ => (HedgeTile::Middle3, Some(HedgeTile::Middle3)),
            }
        }
    }

    pub fn first_hedge_row(index: i32) -> (Vec<HedgeMask>, HedgeRow) {
        if rand::gen_range::<u8>(0, 1) == 0 && index > 7 && index < 14 {
            (Self::generate_hedge_mask(), HedgeRow::First)
        } else {
            (Vec::new(), HedgeRow::None)
        }
    }

    pub fn generate_hedge_mask() -> Vec<HedgeMask> {
        let mut mask = Vec::new();
        mask.resize_with(12, || {
            if rand::gen_range::<u8>(1, 100) > 1 {
                HedgeMask::Hedge
            } else {
                HedgeMask::Empty
            }
        });
        // Ensure there is at least one gap
        mask.insert(rand::gen_range(0, 11), HedgeMask::Empty);

        let mut new_mask = Vec::with_capacity(12);
        //debug!("mask: {:?}", mask);
        for i in 0..12 {
            let low_index = 0.max(i as i32 - 1) as usize;
            let high_index = 12.min(i + 1);
            //debug!(
            //    "i: {} low_index: {} high_index: {}\nnew_mask: {:?}",
            //    i, low_index, high_index, new_mask
            //);
            new_mask.push(
                if &mask[low_index..=high_index]
                    .iter()
                    .filter(|&&item| item == HedgeMask::Empty)
                    .collect::<Vec<&HedgeMask>>()
                    .len()
                    > &0
                {
                    HedgeMask::Empty
                } else {
                    HedgeMask::Hedge
                },
            );
        }

        // Duplicate first and last elements
        let mut mask = Vec::new();
        mask.push(new_mask.get(0).unwrap().clone());
        mask.extend(new_mask.clone());
        mask.push(new_mask.pop().unwrap());

        mask
    }

    fn grass_row(&self, index: i32, y: i32) -> RowType {
        RowType::Grass(Grass::new(
            Some(self.hedge_mask.clone()),
            self.hedge_row,
            index,
            y,
        ))
    }
}
