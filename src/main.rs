#![windows_subsystem = "windows"]
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
    input::{is_key_pressed, utils::*, KeyCode},
    time::get_frame_time,
    window::{next_frame, Conf},
};

use bunner_macroquad::{
    global_state::GlobalState, resources::Resources, FPS_120, FPS_30, FPS_60, HEIGHT, TITLE, WIDTH,
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
    let mut accumulator: f32 = 0.;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            std::process::exit(0);
        }
        repeat_all_miniquad_input(&mut global_state, input_subscriber);
        // Snap delta to FPS if close enough
        let delta = if (get_frame_time() - FPS_120).abs() < 0.0002 {
            FPS_120
        } else if (get_frame_time() - FPS_60).abs() < 0.0002 {
            FPS_60
        } else if (get_frame_time() - FPS_30).abs() < 0.0002 {
            FPS_30
        } else {
            get_frame_time()
        };
        accumulator += delta;
        while accumulator >= FPS_60 {
            global_state.update();
            accumulator -= FPS_60;
        }
        global_state.draw();

        next_frame().await
    }
}
