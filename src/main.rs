#![allow(clippy::type_complexity)]

use bevy::{
    // ecs::{
    //     archetype::Archetypes,
    //     component::Components,
    //     entity::Entities,
    //     schedule::ReportExecutionOrderAmbiguities,
    // },
    //  diagnostic::{
    //     FrameTimeDiagnosticsPlugin,
    //     LogDiagnosticsPlugin,
    //     EntityCountDiagnosticsPlugin,
    // },
    input::system::exit_on_esc_system,
    prelude::*,
    // reflect::TypeRegistration,
    render::camera::{ScalingMode, WindowOrigin},
};
use bevy_mod_debugdump::schedule_graph_dot;

mod animation;
mod car;
mod collisions;
mod console_debug;
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
    let mut app = App::build();
    app.insert_resource(WindowDescriptor {
        title: "Shoe Crosses the Road".to_string(),
        width: TILE_WIDTH * SCALE * TILE_SIZE as f32,
        height: TILE_HEIGHT * SCALE * TILE_SIZE as f32,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    // .insert_resource(ReportExecutionOrderAmbiguities)
    // .add_system(print_ecs_counts.system())
    // .add_plugin(FrameTimeDiagnosticsPlugin::default())
    // .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
    // Adds a system that prints diagnostics to the console
    // .add_plugin(LogDiagnosticsPlugin::default())
    // .add_startup_system(print_resources.system())
    .add_system(exit_on_esc_system.system())
    .add_state(AppState::Setup)
    .add_system(animation::sprite_animation_system.system())
    .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(setup.system()))
    .add_plugin(loader::AssetsLoadingPlugin)
    .add_plugin(coordinates::MovementPlugin)
    .add_plugin(collisions::CollisionPlugin)
    .add_plugin(map::MapPlugin)
    .add_plugin(car::CarPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(particles::DustSystem)
    .add_plugin(win_screen::WinScreenPlugin)
    .add_plugin(console_debug::ConsoleDebugPlugin)
    .run();

    println!("{}", schedule_graph_dot(&app.app.schedule));
}

fn setup(mut commands: Commands, mut state: ResMut<State<AppState>>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.window_origin = WindowOrigin::BottomLeft;
    camera.orthographic_projection.scaling_mode = ScalingMode::WindowSize;
    camera.orthographic_projection.scale = 1.0 / SCALE;

    commands.spawn().insert_bundle(camera);
    commands.spawn().insert_bundle(UiCameraBundle::default());

    state.set(AppState::AssetLoading).unwrap();
}

// fn print_ecs_counts(entities: &Entities, c: &Components, a: &Archetypes) {
//     info!("entities {}, components: {}, archetypes {}", entities.len(), c.len(), a.len());
// }

// fn print_resources(archetypes: &Archetypes, components: &Components) {
//     let mut r: Vec<String> = archetypes
//         .resource()
//         .components()
//         .map(|id| components.get_info(id).unwrap())
//         // get_short_name removes the path information
//         // i.e. `bevy_audio::audio::Audio` -> `Audio`
//         // if you want to see the path info replace
//         // `TypeRegistration::get_short_name` with `String::from`
//         .map(|info| TypeRegistration::get_short_name(info.name()))
//         .collect();

//     // sort list alphebetically
//     r.sort();
//     r.iter().for_each(|name| println!("{}", name));
// }
