use crate::{
    actor::Actor, bunner::Bunner, eagle::Eagle, grass::Grass, player_state::PlayerState,
    position::Position, row_type::RowType, HEIGHT, ROW_HEIGHT,
};

use macroquad::prelude::{clear_background, BLACK};

#[derive(Default)]
pub struct Game {
    pub bunner: Option<Bunner>,
    pub scroll_pos: i32,
    eagle: Option<Eagle>,
    rows: Vec<RowType>,
}

impl Game {
    pub fn new(bunner: Option<Bunner>) -> Self {
        Self {
            bunner,
            scroll_pos: -HEIGHT,
            eagle: None,
            rows: vec![RowType::Grass(Grass::new(None, 0, 0))],
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

        // Remove rows that have scrolled past the bottom of the screen.
        let scroll_pos = self.scroll_pos;
        self.rows
            .retain(|row| row.y() < (scroll_pos + HEIGHT + ROW_HEIGHT * 2));

        // Add rows
        while let Some(last_row) = self.rows.last() {
            if last_row.y() > self.scroll_pos + ROW_HEIGHT {
                let new_row = last_row.next();
                self.rows.push(new_row)
            } else {
                break;
            }
        }

        for row in self.rows.iter_mut() {
            row.update();
        }
        if let Some(bunner) = self.bunner.as_mut() {
            bunner.update(self.scroll_pos, &mut self.rows);
            match bunner.state {
                PlayerState::Eagle(x) => {
                    if self.eagle.is_none() {
                        self.eagle = Some(Eagle::new(Position::new(x, self.scroll_pos)));
                    }
                }
                _ => (),
            };
        }
        if let Some(eagle) = self.eagle.as_mut() {
            eagle.update();
        }

        // TODO: Play river/traffic sounds
    }

    pub fn draw(&mut self) {
        clear_background(BLACK);

        for row in self.rows.iter().rev() {
            row.draw(0, -self.scroll_pos);
        }
        if let Some(bunner) = &self.bunner {
            bunner.draw(0, -self.scroll_pos);
        }
        if let Some(eagle) = &self.eagle {
            eagle.draw(0, -self.scroll_pos);
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
