use macroquad::audio::{play_sound, set_sound_volume, PlaySoundParams, Sound};
use macroquad::prelude::{
    collections::storage, draw_texture, is_key_pressed, rand, KeyCode, WHITE,
};
use std::collections::VecDeque;
use std::fs;

use crate::{
    bunner::Bunner,
    drawing::{display_number, NumberAlign, NumberColor},
    game::Game,
    player_direction::PlayerDirection,
    position::Position,
    resources::Resources,
    state::State,
    HEIGHT, WIDTH,
};

pub struct GlobalState {
    state: State,
    game: Game,
    high_score: u32,
    music: Sound,
}

impl GlobalState {
    pub fn new(music: Sound) -> Self {
        Self {
            // Set the initial game state
            state: State::Menu,
            game: Game::new(None),
            high_score: 0,
            music,
        }
    }

    pub fn init(&mut self) {
        rand::srand(macroquad::miniquad::date::now() as u64);
        play_sound(
            self.music,
            PlaySoundParams {
                looped: true,
                volume: 1.,
            },
        );
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.high_score = fs::read_to_string("high.txt")
                .map_or(Ok(0), |i| i.parse::<u32>())
                .unwrap_or(0);
        }
    }

    pub fn update(&mut self, input_queue: VecDeque<KeyCode>) {
        match self.state {
            State::Menu => {
                if input_queue.contains(&KeyCode::Space) {
                    // Switch to play state, and create a new Game object, passing it a new Player object to use
                    self.state = State::Play;
                    self.game = Game::new(Some(Bunner::new(Position::new(240, -320))));
                    set_sound_volume(self.music, 0.3);
                } else {
                    self.game.update(input_queue);
                }
            }
            State::Play => {
                if self.game.game_over() {
                    self.high_score = self.high_score.max(self.game.score());
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        fs::write("high.txt", self.high_score.to_string()).ok();
                    }

                    self.state = State::GameOver;
                } else {
                    self.game.update(input_queue);
                }
            }
            State::GameOver => {
                if input_queue.contains(&KeyCode::Space) {
                    // Switch to menu state, and create a new game object
                    self.state = State::Menu;
                    self.game = Game::new(None);
                    set_sound_volume(self.music, 1.0);
                }
            }
        }
    }

    pub fn draw(&mut self) {
        let resources = storage::get::<Resources>();

        self.game.draw();

        match self.state {
            State::Menu => {
                // Draw title screen
                draw_texture(resources.title_texture, 0., 0., WHITE);
                //let screen = (self.game.scroll_pos / 6 % 4) as usize;
                let screen = 0;
                draw_texture(
                    resources.start_textures[screen],
                    (WIDTH - 270) as f32 / 2.,
                    (HEIGHT - 240) as f32,
                    WHITE,
                );
            }
            State::Play => {
                // Display score and high score
                display_number(self.game.score(), NumberColor::Blue, 0, NumberAlign::Left);
                display_number(
                    self.high_score,
                    NumberColor::Yellow,
                    WIDTH - 10,
                    NumberAlign::Right,
                );
            }
            State::GameOver => {
                // Display "Game Over" image
                draw_texture(resources.gameover_texture, 0., 0., WHITE);
            }
        }
    }
}
