use crate::{
    child_type::ChildType, player_state::PlayerState, position::Position, resources::Resources,
    road::Road, row::Row, row_type::RowType, train::Train, water::Water, HEIGHT, ROW_HEIGHT, WIDTH,
};

use macroquad::audio::play_sound_once;
use macroquad::prelude::{collections::storage, debug, draw_texture, rand::gen_range, WHITE};
use macroquad::rand;
use macroquad::rand::ChooseRandom;

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

    fn update(&mut self, scroll_pos: i32) {
        self.update_children();
        if self.index == 2 {
            self.children
                .retain(|c| c.x() > -1000 && c.x() < WIDTH + 1000);
            if self.y < scroll_pos + HEIGHT
                && self.children.is_empty()
                && rand::gen_range::<u8>(0, 100) < 1
            {
                let dx = *vec![-20, 20].choose().unwrap();
                let position = if dx < 0 {
                    Position::new(WIDTH + 1000, 47)
                } else {
                    Position::new(WIDTH - 1000, 47)
                };
                self.children
                    .push(ChildType::Train(Train::new(dx, position)));
                play_sound_once(storage::get::<Resources>().bell_sound);
                let train_sound = *storage::get::<Resources>()
                    .train_sounds
                    .get(rand::gen_range::<usize>(0, 2))
                    .unwrap();
                play_sound_once(train_sound);
            }
        }
    }

    fn draw(&self, offset_x: i32, offset_y: i32) {
        let image = *storage::get::<Resources>()
            .rail_textures
            .get(self.index as usize)
            .unwrap();
        let x = offset_x;
        let y = self.y + offset_y;
        draw_texture(image, x as f32, y as f32 - image.height(), WHITE);
        self.draw_children(x, y);
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

    fn check_collision(&self, x: i32) -> PlayerState {
        if self.index == 2 && self.collide(x, 0) {
            play_sound_once(storage::get::<Resources>().splat_sound);
            return PlayerState::Splat(8);
        }
        PlayerState::Alive
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
