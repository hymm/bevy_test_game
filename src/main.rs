use bevy::{input::system::exit_on_esc_system, prelude::*};
mod consts;
mod loader;
mod map;
use crate::consts::{AppState, APP_STATE_STAGE, SCALE, TILE_HEIGHT, TILE_SIZE, TILE_WIDTH};
mod car;
use car::Position;
use map::TilePosition;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Shoe Crosses the Road".to_string(),
            width: TILE_WIDTH * SCALE * TILE_SIZE as f32,
            height: TILE_HEIGHT * SCALE * TILE_SIZE as f32,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system.system())
        .add_resource(State::new(AppState::Setup))
        .add_stage_after(
            stage::UPDATE,
            APP_STATE_STAGE,
            StateStage::<AppState>::default(),
        )
        .on_state_enter(APP_STATE_STAGE, AppState::Setup, setup.system())
        .add_plugin(loader::AssetsLoadingPlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(car::CarPlugin)
        .add_system(convert_tile_coord.system())
        .run();
}

struct Player;

fn setup(commands: &mut Commands, mut state: ResMut<State<AppState>>) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(64.0 * SCALE, 64.0 * SCALE, 1000.0 - 0.1),
            ..Default::default()
        },
        ..Camera2dBundle::default()
    });
    state.set_next(AppState::AssetLoading).unwrap();
}

fn convert_tile_coord(mut q: Query<(&mut Position, &TilePosition)>) {
    for (mut pos, tile_pos) in q.iter_mut() {
        pos.x = tile_pos.x * TILE_SIZE as f32;
        pos.y = tile_pos.y * TILE_SIZE as f32;
    }
}
