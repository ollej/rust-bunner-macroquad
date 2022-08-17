pub mod actor;
pub mod bunner;
pub mod car;
pub mod child;
pub mod dirt;
pub mod drawing;
pub mod eagle;
pub mod game;
pub mod global_state;
pub mod grass;
pub mod hedge;
pub mod hedge_mask;
pub mod hedge_row;
pub mod hedge_tile;
pub mod mover;
pub mod pavement;
pub mod player_direction;
pub mod player_state;
pub mod position;
pub mod rail;
pub mod resources;
pub mod road;
pub mod row;
pub mod splat;
pub mod state;
pub mod train;
pub mod water;

pub const WIDTH: i32 = 480;
pub const HEIGHT: i32 = 800;
pub const TITLE: &str = "Bunner Macroquad";
pub const ROW_HEIGHT: i32 = 40;
pub const TILE_WIDTH: i32 = 40;
