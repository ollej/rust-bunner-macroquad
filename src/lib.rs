pub mod bunner;
pub mod drawing;
pub mod game;
pub mod global_state;
pub mod grass;
pub mod hedge;
pub mod hedge_mask;
pub mod hedge_row;
pub mod hedge_tile;
pub mod player_direction;
pub mod player_state;
pub mod position;
pub mod resources;
pub mod splat;
pub mod state;

pub const WIDTH: i32 = 480;
pub const HEIGHT: i32 = 800;
pub const TITLE: &str = "Bunner Macroquad";
pub const ROW_HEIGHT: i32 = 40;
pub const TILE_WIDTH: i32 = 40;