use crate::{bunner::Bunner, player_state::PlayerState, resources::Resources};

use macroquad::prelude::{clear_background, draw_texture, BLACK, WHITE};
use macroquad::rand::gen_range;
use macroquad::{prelude::collections::storage, rand::ChooseRandom};

#[derive(Default)]
pub struct Game {
    pub bunner: Option<Bunner>,
    pub scroll_pos: usize,
    pub score: i32,
    timer: i32,
}

impl Game {
    pub fn new(bunner: Option<Bunner>) -> Self {
        Self {
            bunner,
            scroll_pos: 0,
            score: 0,
            timer: 0,
        }
    }

    pub fn update(&mut self) {
        self.timer += 1;
    }

    pub fn draw(&self) {
        let resources = storage::get::<Resources>();
        clear_background(BLACK);
    }

    pub fn game_over(&self) -> bool {
        if let Some(bunner) = &self.bunner {
            bunner.state != PlayerState::Alive && bunner.timer < 0
        } else {
            false
        }
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}
