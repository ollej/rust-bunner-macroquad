#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::comparison_chain,
    clippy::derive_partial_eq_without_eq,
    clippy::len_zero,
    clippy::manual_range_contains,
    clippy::type_complexity
)]

mod bunner;
mod drawing;
mod game;
mod global_state;
mod grass;
mod hedge;
mod hedge_mask;
mod hedge_row;
mod hedge_tile;
mod player_direction;
mod player_state;
mod position;
mod resources;
mod state;

use macroquad::{
    audio::{self},
    prelude::{collections::storage, coroutines::start_coroutine, *},
};

use global_state::GlobalState;
use resources::Resources;

use std::error;

pub const WIDTH: i32 = 480;
pub const HEIGHT: i32 = 800;
pub const TITLE: &str = "Bunner Macroquad";

pub const ROW_HEIGHT: i32 = 40;

fn window_conf() -> Conf {
    Conf {
        window_title: TITLE.into(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

async fn load_resources() -> Result<(), Box<dyn error::Error>> {
    let resources_loading = start_coroutine(async move {
        let resources = Resources::new().await.unwrap();
        storage::store(resources);
    });

    while !resources_loading.is_done() {
        clear_background(BLACK);
        let text = format!(
            "Loading resources {}",
            ".".repeat(((get_time() * 2.) as usize) % 4)
        );
        draw_text(
            &text,
            screen_width() / 2. - 160.,
            screen_height() / 2.,
            40.,
            WHITE,
        );

        next_frame().await;
    }

    Ok(())
}

#[macroquad::main(window_conf())]
async fn main() -> Result<(), Box<dyn error::Error>> {
    load_resources().await?;

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
