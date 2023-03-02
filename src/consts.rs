use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, States, Default)]
pub enum AppState {
    #[default]
    Setup,
    AssetLoading,
    Loading,
    InGame,
    LevelDone,
    Finished,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, SystemSet)]
pub enum SystemLabels {
    PlayerMovement,
}

pub const TILE_WIDTH: f32 = 16.0;
pub const TILE_HEIGHT: f32 = 16.0;
pub const SCALE: f32 = 4.0;
pub const TILE_SIZE: i32 = 8;
pub const SCREEN_X_MAX: i32 = TILE_WIDTH as i32 * TILE_SIZE as i32;
pub const SCREEN_Y_MAX: i32 = TILE_HEIGHT as i32 * TILE_SIZE as i32;
