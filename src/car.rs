use crate::consts::{AppState, APP_STATE_STAGE, SCALE, SCREEN_X_MAX, SCREEN_Y_MAX, TILE_SIZE};
use crate::map::TilePosition;
use bevy::prelude::*;

struct Car;
struct GoingOffscreenEvent(Entity, f32);

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
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

#[derive(Clone, Default)]
struct Materials {
    suv_material: Handle<ColorMaterial>,
}

fn store_car_material(
    mut m: ResMut<Materials>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let handle = asset_server.get_handle("suv.png");
    m.suv_material = materials.add(handle.into());
}

fn spawn_car(commands: &mut Commands, m: Materials, tile_pos: TilePosition) {
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
            x: (tile_pos.x * TILE_SIZE as f32 * SCALE),
            y: (tile_pos.y * TILE_SIZE as f32 * SCALE),
        })
        .with(Velocity { x: 10.0, y: 0.0 }) // pixels per second
        .with(Hitbox {
            width: 14.0,
            height: 8.0,
            x: -7.0,
            y: -4.0,
            ..Default::default()
        });
}

fn spawn_initial_cars(commands: &mut Commands, m: Res<Materials>) {
    let tile_pos = TilePosition { x: 0.1, y: 6.0 };
    spawn_car(commands, m.clone(), tile_pos)
}

fn spawn_another_car(
    commands: &mut Commands,
    events: Res<Events<GoingOffscreenEvent>>,
    mut event_reader: Local<EventReader<GoingOffscreenEvent>>,
    m: Res<Materials>,
) {
    for ev in event_reader.iter(&events) {
        spawn_car(commands, m.clone(), TilePosition { x: 0.0, y: ev.1 });
    }
}

fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(pos.x as f32, pos.y as f32, 0.0);
    }
}

struct FullyOffscreen;
struct GoingOffscreen;
fn fully_offscreen(mut q: Query<(Entity, &Position, &Hitbox), Without<FullyOffscreen>>, commands: &mut Commands) {
    for (entity, pos, hitbox) in q.iter_mut() {
        let left = pos.x + hitbox.x;
        let right = pos.x + hitbox.x + hitbox.width;
        let top = pos.y + hitbox.y;
        let bottom = pos.y + hitbox.y + hitbox.height;
        if (right as i32) < 0
            || (left as i32) > SCREEN_X_MAX
            || (top as i32) < 0
            || (bottom as i32) > SCREEN_Y_MAX
        {
            commands.insert_one(entity, FullyOffscreen);
        }
    }
}

fn going_offscreen(
    mut q: Query<
        (Entity, &Position, &Hitbox, &Velocity),
        (Without<FullyOffscreen>, Without<GoingOffscreen>),
    >,
    commands: &mut Commands,
    mut ev_going_offscreen: ResMut<Events<GoingOffscreenEvent>>,
) {
    for (entity, pos, hitbox, velocity) in q.iter_mut() {
        let left_offscreen = (pos.x + hitbox.x < 0.0) && velocity.x < 0.0;
        let right_offscreen =
            (pos.x + hitbox.x + hitbox.width > SCREEN_X_MAX as f32) && velocity.x > 0.0;
        let top_offscreen = (pos.y + hitbox.y > SCREEN_Y_MAX as f32) && velocity.y > 0.0;
        let bottom_offscreen = (pos.y + hitbox.y + hitbox.height < 0.0) && velocity.y < 0.0;
        if left_offscreen || right_offscreen || top_offscreen || bottom_offscreen {
            ev_going_offscreen.send(GoingOffscreenEvent(entity, pos.y / SCALE / TILE_SIZE as f32));
            commands.insert_one(entity, GoingOffscreen);
        }
    }
}

fn despawn_out_of_bounds(
    commands: &mut Commands,
    mut q: Query<Entity, (With<GoingOffscreen>, With<FullyOffscreen>)>,
) {
    for entity in q.iter_mut() {
        commands.despawn(entity);
    }
}

fn update_position(mut q: Query<(&Velocity, &mut Position)>, time: Res<Time>) {
    for (v, mut p) in q.iter_mut() {
        p.x += v.x * SCALE * time.delta_seconds();
        p.y += v.y * SCALE * time.delta_seconds();
    }
}

pub struct CarPlugin;
impl Plugin for CarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Materials>()
            .add_event::<GoingOffscreenEvent>()
            .on_state_enter(
                APP_STATE_STAGE,
                AppState::Loading,
                store_car_material.system(),
            )
            .on_state_enter(
                APP_STATE_STAGE,
                AppState::Loading,
                spawn_initial_cars.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::InGame,
                position_translation.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::InGame,
                spawn_another_car.system(),
            )
            .on_state_update(APP_STATE_STAGE, AppState::InGame, update_position.system())
            .on_state_update(APP_STATE_STAGE, AppState::InGame, fully_offscreen.system())
            .on_state_update(
                APP_STATE_STAGE,
                AppState::InGame,
                despawn_out_of_bounds.system(),
            )
            .on_state_update(APP_STATE_STAGE, AppState::InGame, going_offscreen.system());
    }
}
