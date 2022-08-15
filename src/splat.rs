use crate::{
    player_direction::PlayerDirection, position::Position, resources::Resources, ROW_HEIGHT,
};
use macroquad::prelude::{collections::storage, draw_texture, WHITE};

pub struct Splat {
    direction: PlayerDirection,
    position: Position,
}

impl Splat {
    pub fn new(direction: PlayerDirection, position: Position) -> Self {
        Self {
            direction,
            position,
        }
    }

    pub fn update(&mut self) {}

    pub fn draw(&self, offset_x: i32, offset_y: i32) {
        let image = *storage::get::<Resources>()
            .splat_textures
            .get(self.direction as usize)
            .unwrap();
        draw_texture(
            image,
            (self.position.x + offset_x) as f32,
            (self.position.y + offset_y - ROW_HEIGHT) as f32,
            WHITE,
        );
    }
}
