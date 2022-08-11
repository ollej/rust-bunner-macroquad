use macroquad::audio::play_sound_once;
use macroquad::prelude::{
    collections::storage, draw_texture, get_last_key_pressed, KeyCode, Texture2D, WHITE,
};
use std::collections::VecDeque;

use crate::{
    player_direction::PlayerDirection, player_state::PlayerState, position::Position,
    resources::Resources, ROW_HEIGHT,
};

pub struct Bunner {
    pub state: PlayerState,
    pub timer: i32,
    pub x: i32,
    pub y: i32,
    pub min_y: i32,
    direction: PlayerDirection,
    input_queue: VecDeque<PlayerDirection>,
    image: Texture2D,
}

impl Bunner {
    const MOVE_DISTANCE: i32 = 10;

    pub fn new(position: Position) -> Self {
        Self {
            state: PlayerState::Alive,
            timer: 0,
            x: position.x,
            y: position.y,
            min_y: position.y,
            direction: PlayerDirection::Down,
            input_queue: VecDeque::new(),
            image: storage::get::<Resources>().blank_texture,
        }
    }

    pub fn update(&mut self) {
        if let Some(direction) = get_last_key_pressed()
            .map(|d| match d {
                KeyCode::Up => Some(PlayerDirection::Up),
                KeyCode::Right => Some(PlayerDirection::Right),
                KeyCode::Down => Some(PlayerDirection::Down),
                KeyCode::Left => Some(PlayerDirection::Left),
                _ => None,
            })
            .flatten()
        {
            self.input_queue.push_back(direction);
        }

        match self.state {
            PlayerState::Alive => {
                // While the player is alive, the timer variable is used for movement.
                // If it's zero, the player is on the ground. If it's above zero,
                // they're currently jumping to a new location.

                // Are we on the ground, and are there inputs to process?
                if self.timer == 0 {
                    // Take the next input off the queue and process it
                    let direction = self.input_queue.pop_front();
                    self.handle_input(direction);
                }

                let mut land = false;
                if self.timer > 0 {
                    // Apply movement
                    self.x += match &self.direction {
                        PlayerDirection::Up => 0,
                        PlayerDirection::Right => 4,
                        PlayerDirection::Down => 0,
                        PlayerDirection::Left => -4,
                    };
                    self.y += match &self.direction {
                        PlayerDirection::Up => -4,
                        PlayerDirection::Right => 0,
                        PlayerDirection::Down => 4,
                        PlayerDirection::Left => 0,
                    };
                    self.timer -= 1;
                    // If timer reaches zero, we've just landed
                    land = self.timer == 0;
                }
            }
            _ => {
                // Not alive - timer now counts down prior to game over screen
                self.timer -= 1
            }
        }

        // Keep track of the furthest we've got in the level
        self.min_y = self.min_y.min(self.y);

        // Choose sprite image
        self.image = match self.state {
            PlayerState::Alive => {
                let sprite_index = match self.direction {
                    PlayerDirection::Up => 0,
                    PlayerDirection::Right => 1,
                    PlayerDirection::Down => 2,
                    PlayerDirection::Left => 3,
                };
                if self.timer > 0 {
                    *storage::get::<Resources>()
                        .jump_textures
                        .get(sprite_index)
                        .unwrap()
                } else {
                    *storage::get::<Resources>()
                        .sit_textures
                        .get(sprite_index)
                        .unwrap()
                }
            }
            PlayerState::Splash if self.timer > 84 => {
                // Display appropriate 'splash' animation frame. Note that we use a different technique to display the
                // 'splat' image - see: comments earlier in this method. The reason two different techniques are used is
                // that the splash image should be drawn on top of other objects, whereas the splat image must be drawn
                // underneath other objects. Since the player is always drawn on top of other objects, changing the player
                // sprite is a suitable method of displaying the splash image.
                let splash_index = ((100 - self.timer) / 2) as usize;
                *storage::get::<Resources>()
                    .splash_textures
                    .get(splash_index)
                    .unwrap()
            }
            _ => storage::get::<Resources>().blank_texture,
        };
    }

    pub fn draw(&mut self, offset_x: i32, offset_y: i32) {
        draw_texture(
            self.image,
            (self.x + offset_x) as f32,
            (self.y + offset_y - ROW_HEIGHT) as f32,
            WHITE,
        );
    }

    pub fn handle_input(&mut self, direction: Option<PlayerDirection>) {
        if let Some(direction) = direction {
            self.direction = direction;
            self.timer = Bunner::MOVE_DISTANCE;
            play_sound_once(storage::get::<Resources>().jump_sound);
        }
    }
}
