#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::comparison_chain,
    clippy::derive_partial_eq_without_eq,
    clippy::len_zero,
    clippy::manual_range_contains,
    clippy::type_complexity
)]

use macroquad::{
    audio::{self},
    prelude::{next_frame, Conf},
};

use bunner_macroquad::{global_state::GlobalState, resources::Resources, HEIGHT, TITLE, WIDTH};

use std::error;

fn window_conf() -> Conf {
    Conf {
        window_title: TITLE.into(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() -> Result<(), Box<dyn error::Error>> {
    Resources::load().await?;

    // Start music
    let music = audio::load_sound("resources/music/theme.ogg").await?;
    let mut global_state = GlobalState::new(music);
    global_state.init();

    loop {
        global_state.update();
        global_state.draw();

        next_frame().await
    }
}
