use crate::{
    child_type::ChildType, resources::Resources, road::Road, row::Row, row_type::RowType,
    water::Water, ROW_HEIGHT,
};

use macroquad::audio::play_sound_once;
use macroquad::prelude::{collections::storage, debug, draw_texture, WHITE};
use macroquad::rand;

#[derive(Clone)]
pub struct Dirt {
    index: i32,
    y: i32,
    children: Vec<ChildType>,
}

impl Row for Dirt {
    fn y(&self) -> i32 {
        self.y
    }

    fn children(&self) -> &[ChildType] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<ChildType> {
        self.children.as_mut()
    }

    fn update(&mut self, scroll_pos: i32) {}

    fn draw(&self, offset_x: i32, offset_y: i32) {
        let image = *storage::get::<Resources>()
            .dirt_textures
            .get(self.index as usize)
            .unwrap();
        let x = offset_x;
        let y = self.y + offset_y - image.height() as i32;
        draw_texture(image, x as f32, y as f32, WHITE);
    }

    fn play_sound(&self) {
        play_sound_once(storage::get::<Resources>().dirt_sound);
    }

    fn next(&self) -> RowType {
        let y = self.y - ROW_HEIGHT;
        if self.index <= 5 {
            RowType::Dirt(Dirt::new(self.index + 8, y))
        } else if self.index == 6 {
            RowType::Dirt(Dirt::new(7, y))
        } else if self.index == 7 {
            RowType::Dirt(Dirt::new(15, y))
        } else if self.index >= 8 && self.index <= 14 {
            RowType::Dirt(Dirt::new(self.index + 1, y))
        } else {
            if rand::gen_range::<u8>(0, 2) == 1 {
                RowType::Road(Road::empty(y))
            } else {
                RowType::Water(Water::empty(y))
            }
        }
    }
}

impl Dirt {
    pub fn new(index: i32, y: i32) -> Self {
        Self {
            index,
            y,
            children: Vec::new(),
        }
    }
}
