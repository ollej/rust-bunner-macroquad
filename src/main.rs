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
    input::utils::*,
    prelude::{next_frame, Conf},
    time::get_frame_time,
};

use bunner_macroquad::{
    global_state::GlobalState, resources::Resources, HEIGHT, TIME_PER_FRAME, TITLE, WIDTH,
};

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

    let input_subscriber = register_input_subscriber();
    let mut frame_time: f32 = 0.;
    loop {
        repeat_all_miniquad_input(&mut global_state, input_subscriber);
        frame_time += get_frame_time().min(0.25);
        while frame_time >= TIME_PER_FRAME {
            global_state.update();
            frame_time -= TIME_PER_FRAME;
        }
        global_state.draw();

        next_frame().await
    }
}
