use bevy::{
    input::system::exit_on_esc_system, 
    prelude::*,
    render::camera::Camera,
};


mod consts;
mod loader;
mod map;
mod car;
mod coordinates;
use crate::consts::{AppState, APP_STATE_STAGE, SCALE, TILE_HEIGHT, TILE_SIZE, TILE_WIDTH};

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Shoe Crosses the Road".to_string(),
            width: TILE_WIDTH * SCALE * TILE_SIZE  as f32,
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
        .run();
}

struct Player;

fn setup(commands: &mut Commands, mut state: ResMut<State<AppState>>) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(64.0, 64.0, 0.0),
            scale: Vec3::splat(1. / SCALE),
            ..Default::default()
        },
        ..Camera2dBundle::default()
    });
    state.set_next(AppState::AssetLoading).unwrap();
}
