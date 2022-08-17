use crate::{
    child_type::ChildType, grass::Grass, pavement::Pavement, player_state::PlayerState, rail::Rail,
    resources::Resources, row::Row, ROW_HEIGHT, WIDTH,
};

use macroquad::audio::play_sound_once;
use macroquad::prelude::{collections::storage, debug, draw_texture, WHITE};
use macroquad::rand;

#[derive(Clone)]
pub struct Road {
    dx: i32,
    previous_dx: i32,
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

    fn next(&self) -> Box<dyn Row> {
        let y = self.y - ROW_HEIGHT;
        if self.index == 0 {
            Box::new(Road::new(self.dx, 1, y))
        } else if self.index < 5 {
            let random = rand::gen_range::<u8>(0, 100);
            if random < 80 {
                Box::new(Road::new(self.dx, self.index + 1, y))
            } else if random < 88 {
                Box::new(Grass::without_hedge(rand::gen_range(0, 7), y))
            } else if random < 94 {
                Box::new(Rail::empty(y))
            } else {
                Box::new(Pavement::empty(y))
            }
        } else {
            let random = rand::gen_range::<u8>(0, 100);
            if random < 60 {
                Box::new(Grass::without_hedge(rand::gen_range(0, 7), y))
            } else if random < 90 {
                Box::new(Rail::empty(y))
            } else {
                Box::new(Pavement::empty(y))
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
