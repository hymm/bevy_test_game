use bevy::{
    input::system::exit_on_esc_system, 
    prelude::*
};
mod consts;
mod loader;
mod map;
use crate::consts::{
    AppState, APP_STATE_STAGE, SCALE, SCREEN_X_MAX, SCREEN_Y_MAX, TILE_HEIGHT, TILE_SIZE,
    TILE_WIDTH,
};
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
        .init_resource::<Materials>()
        .add_stage_after(
            stage::UPDATE,
            APP_STATE_STAGE,
            StateStage::<AppState>::default(),
        )
        .on_state_enter(APP_STATE_STAGE,
            AppState::Setup,setup.system())
        .add_plugin(loader::AssetsLoadingPlugin)
        .add_plugin(map::MapPlugin)
        .on_state_enter(
            APP_STATE_STAGE,
            AppState::Loading,
            spawn_initial_cars.system(),
        )
        .on_state_update(
            APP_STATE_STAGE,
            AppState::InGame,
            spawn_another_car.system(),
        )
        .add_system(update_position.system())
        .add_system(convert_tile_coord.system())
        .add_system(position_translation.system())
        .add_event::<FullyOffscreenEvent>()
        .add_event::<PartiallyOffscreenEvent>()
        .add_system(offscreen.system())
        .add_system(despawn_out_of_bounds.system())
        .run();
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Default, Copy, Clone, PartialEq)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone, PartialEq)]
struct Hitbox {
    x: f32, // relative to parent
    y: f32, // relative to parent
    width: f32,
    height: f32,
}

impl Default for Hitbox {
    fn default() -> Self {
        Hitbox {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
        }
    }
}

struct Player;
struct Car;
struct PartiallyOffscreenEvent(Entity, f32);
struct FullyOffscreenEvent(Entity);

#[derive(Clone, Default)]
struct Materials {
    suv_material: Handle<ColorMaterial>,
}

fn setup(
    commands: &mut Commands, 
    mut state: ResMut<State<AppState>>,
) {
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
        pos.x = (tile_pos.x * TILE_SIZE as f32) as i32;
        pos.y = (tile_pos.y * TILE_SIZE as f32) as i32;
    }
}

fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(pos.x as f32, pos.y as f32, 0.0);
    }
}

fn spawn_car(commands: &mut Commands, m: Materials, tile_pos: TilePosition) {
    println!("spawn_car");
    commands
        .spawn(SpriteBundle {
            material: m.suv_material.clone(),
            transform: Transform {
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Car)
        .with(Position {
            x: (tile_pos.x * TILE_SIZE as f32 * SCALE) as i32,
            y: (tile_pos.y * TILE_SIZE as f32 * SCALE) as i32,
        })
        .with(Velocity { x: 1.0, y: 0.0 })
        .with(Hitbox {
            width: 14.0,
            height: 8.0,
            ..Default::default()
        });
}

fn spawn_initial_cars(commands: &mut Commands, m: Res<Materials>) {
    println!("spawn_initial_cars");
    let tile_pos = TilePosition { x: 0.1, y: 6.0 };
    spawn_car(commands, m.clone(), tile_pos)
}

fn offscreen(
    mut q: Query<(Entity, &Position, &Hitbox)>,
    mut ev_partial: ResMut<Events<PartiallyOffscreenEvent>>,
    mut ev_full: ResMut<Events<FullyOffscreenEvent>>,
) {
    for (entity, pos, hitbox) in q.iter_mut() {
        let left = pos.x as f32 + hitbox.x;
        let right = pos.x as f32 + hitbox.x + hitbox.width;
        let top = pos.y as f32 + hitbox.y;
        let bottom = pos.y as f32 + hitbox.y + hitbox.height;
        if (right as i32) < 0
            || (left as i32) > SCREEN_X_MAX
            || (top as i32) < 0
            || (bottom as i32) > SCREEN_Y_MAX
        {
            ev_full.send(FullyOffscreenEvent(entity));
            continue;
        }

        if (left as i32) < 0
            || (right as i32) > SCREEN_X_MAX
            || (bottom as i32) < 0
            || (top as i32) > SCREEN_Y_MAX
        {
            ev_partial.send(PartiallyOffscreenEvent(entity, pos.y as f32));
            continue;
        }
    }
}

fn despawn_out_of_bounds(
    commands: &mut Commands,
    events: Res<Events<FullyOffscreenEvent>>,
    mut event_reader: Local<EventReader<FullyOffscreenEvent>>,
) {
    for ev in event_reader.iter(&events) {
        commands.despawn(ev.0);
    }
}

fn spawn_another_car(
    commands: &mut Commands,
    events: Res<Events<PartiallyOffscreenEvent>>,
    mut event_reader: Local<EventReader<PartiallyOffscreenEvent>>,
    m: Res<Materials>,
) {
    for ev in event_reader.iter(&events) {
        spawn_car(commands, m.clone(), TilePosition { x: 0.1, y: ev.1 });
    }
}

fn update_position(mut q: Query<(&Velocity, &mut Position)>) {
    for (v, mut p) in q.iter_mut() {
        p.x += (v.x * SCALE) as i32;
        p.y += (v.y * SCALE) as i32;
    }
}
