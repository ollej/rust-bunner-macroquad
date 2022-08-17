use crate::{
    child::Child, dirt::Dirt, position::Position, resources::Resources, row::Row, ROW_HEIGHT,
};

use macroquad::audio::play_sound_once;
use macroquad::prelude::{collections::storage, debug, draw_texture, WHITE};
use macroquad::rand;

#[derive(Clone)]
pub struct Water {
    dx: i32,
    previous_dx: i32,
    index: i32,
    y: i32,
    children: Vec<Child>,
}

impl Row for Water {
    fn y(&self) -> i32 {
        self.y
    }

    fn children(&self) -> &[Child] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Child> {
        self.children.as_mut()
    }

    fn update(&mut self, scroll_pos: i32, bunner_pos: Option<Position>) {
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

    fn next(&self) -> Box<dyn Row> {
        let y = self.y - ROW_HEIGHT;
        if self.index == 7 || (self.index >= 1 && rand::gen_range(0, 2) == 0) {
            Box::new(Dirt::new(rand::gen_range(4, 7), y))
        } else {
            Box::new(Water::new(self.dx, self.index + 1, y))
        }
    }

    // TODO: implement push()
}

impl Water {
    pub fn new(previous_dx: i32, index: i32, y: i32) -> Self {
        Self {
            dx: 0,
            previous_dx,
            index,
            y,
            children: Vec::new(),
        }
    }

    pub fn empty(y: i32) -> Self {
        Self::new(0, 0, y)
    }
}
