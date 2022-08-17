use crate::{
    child::Child, dirt::Dirt, log::Log, player_state::PlayerState, position::Position,
    resources::Resources, row::Row, ROW_HEIGHT, WIDTH,
};

use macroquad::audio::play_sound_once;
use macroquad::prelude::{collections::storage, debug, draw_texture, WHITE};
use macroquad::rand;

#[derive(Clone)]
pub struct Water {
    dx: i32,
    previous_dx: i32,
    timer: f32,
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
        self.update_children();
        self.children.retain(|c| c.x() > -70 && c.x() < WIDTH + 70);
        self.timer -= 1.;

        // Create new child objects on a random interval
        if self.timer < 0. {
            let pos = Position::new(if self.dx < 0 { WIDTH + 70 } else { -70 }, 0);
            self.children.push(Child::Log(Log::new(self.dx, pos)));
            // 240 is minimum distance between the start of one child object and the start of the next, assuming its
            // speed is 1. If the speed is 2, they can occur twice as frequently without risk of overlapping with
            // each other. The maximum distance is double the minimum distance (1 + random value of 1)
            self.timer = (1. + rand::gen_range::<f32>(0.0, 1.0)) * (240 / self.dx.abs()) as f32;
        }
    }

    fn draw(&self, offset_x: i32, offset_y: i32) {
        let image = *storage::get::<Resources>()
            .water_textures
            .get(self.index as usize)
            .unwrap();
        let x = offset_x;
        let y = self.y + offset_y;
        draw_texture(image, x as f32, y as f32 - image.height(), WHITE);

        for child in self.children() {
            child.draw(x, y);
        }
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

    fn check_collision(&self, x: i32) -> PlayerState {
        if self.collide(x, -4) {
            return PlayerState::Alive;
        }
        PlayerState::Splash
    }

    fn push(&self) -> i32 {
        self.dx
    }
}

impl Water {
    pub fn new(previous_dx: i32, index: i32, y: i32) -> Self {
        let dx = if previous_dx >= 0 {
            rand::gen_range(1, 3) * -1
        } else {
            rand::gen_range(1, 3)
        };
        let mut children = Vec::new();
        let mut x = -WIDTH / 2 - 70;
        while x < WIDTH / 2 + 70 {
            x += rand::gen_range::<i32>(240, 481);
            let pos = if dx > 0 {
                Position::new(WIDTH / 2 + x, 0)
            } else {
                Position::new(WIDTH / 2 - x, 0)
            };
            children.push(Child::Log(Log::new(dx, pos)));
        }
        Self {
            dx,
            previous_dx,
            timer: 0.,
            index,
            y,
            children,
        }
    }

    pub fn empty(y: i32) -> Self {
        Self::new(0, 0, y)
    }
}
