#![allow(clippy::type_complexity)]

use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    // ecs::{
    //     schedule::ReportExecutionOrderAmbiguities,
    // },
    //  diagnostic::{
    //     FrameTimeDiagnosticsPlugin,
    //     LogDiagnosticsPlugin,
    //     EntityCountDiagnosticsPlugin,
    // },
    window::{close_on_esc, WindowResolution},
};

mod animation;
mod car;
mod collisions;
mod consts;
mod coordinates;
mod loader;
mod map;
mod particles;
mod player;
mod rng_bag;
mod win_screen;
use crate::consts::{AppState, SCALE, TILE_HEIGHT, TILE_SIZE, TILE_WIDTH};

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Shoe Crosses the Road".to_string(),
                    resolution: WindowResolution::new(
                        TILE_WIDTH * SCALE * TILE_SIZE as f32,
                        TILE_HEIGHT * SCALE * TILE_SIZE as f32,
                    ),
                    ..Default::default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    // .insert_resource(ReportExecutionOrderAmbiguities)
    // .add_plugin(FrameTimeDiagnosticsPlugin::default())
    // .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
    // Adds a system that prints diagnostics to the console
    // .add_plugin(LogDiagnosticsPlugin::default())
    .add_system(close_on_esc)
    .add_state::<AppState>()
    .add_system(animation::sprite_animation_system)
    .add_system(setup.in_schedule(OnEnter(AppState::Setup)))
    .add_plugin(loader::AssetsLoadingPlugin)
    .add_plugin(coordinates::MovementPlugin)
    .add_plugin(collisions::CollisionPlugin)
    .add_plugin(map::MapPlugin)
    .add_plugin(car::CarPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(particles::ParticleSystem)
    .add_plugin(win_screen::WinScreenPlugin)
    // .add_plugin(ConsoleDebugPlugin)
    .run();

    // println!("{}", schedule_graph_dot(&app.app.schedule));
}

fn setup(mut commands: Commands, mut state: ResMut<NextState<AppState>>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.viewport_origin = Vec2::new(0.0, 0.0);
    camera.projection.scaling_mode = ScalingMode::WindowSize(1.0);
    camera.projection.scale = 1.0 / SCALE;

    commands.spawn(camera);

    state.set(AppState::AssetLoading);
}
