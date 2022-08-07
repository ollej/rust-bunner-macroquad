use crate::{bunner::Bunner, player_state::PlayerState, resources::Resources, HEIGHT};

use macroquad::prelude::{clear_background, draw_texture, BLACK, WHITE};
use macroquad::rand::gen_range;
use macroquad::{prelude::collections::storage, rand::ChooseRandom};

#[derive(Default)]
pub struct Game {
    pub bunner: Option<Bunner>,
    pub scroll_pos: i32,
    timer: i32,
}

impl Game {
    pub fn new(bunner: Option<Bunner>) -> Self {
        Self {
            bunner,
            scroll_pos: -HEIGHT,
            timer: 0,
        }
    }

    pub fn update(&mut self) {
        if let Some(bunner) = &self.bunner {
            // Scroll faster if the player is close to the top of the screen. Limit scroll speed to
            // between 1 and 3 pixels per frame.
            self.scroll_pos -= 1.max(3.min(self.scroll_pos + HEIGHT - bunner.y) / (HEIGHT / 4));
        } else {
            self.scroll_pos -= 1;
        }

        if let Some(bunner) = self.bunner.as_mut() {
            bunner.update();
        }
    }

    pub fn draw(&mut self) {
        let resources = storage::get::<Resources>();
        clear_background(BLACK);

        if let Some(bunner) = self.bunner.as_mut() {
            bunner.draw(0, -self.scroll_pos);
        }
    }

    pub fn game_over(&self) -> bool {
        if let Some(bunner) = &self.bunner {
            bunner.state != PlayerState::Alive && bunner.timer < 0
        } else {
            false
        }
    }

    pub fn score(&self) -> u32 {
        if let Some(bunner) = &self.bunner {
            0.max((-320 - bunner.min_y as i32) / 40) as u32
        } else {
            0
        }
    }
}
