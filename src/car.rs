use crate::collisions::Hitbox;
use crate::consts::{AppState, APP_STATE_STAGE, SCREEN_X_MAX, SCREEN_Y_MAX, TILE_SIZE};
use crate::coordinates::{PixelPosition, TilePosition, Velocity};
use crate::map::Map;
use bevy::prelude::*;

pub struct Car;
struct GoingOffscreenEvent(Entity, f32, f32);

#[derive(Clone, Default)]
struct Materials {
    suv_material: Handle<ColorMaterial>,
}

fn store_car_material(
    mut m: ResMut<Materials>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let handle = asset_server.get_handle("sprites/suv.png");
    m.suv_material = materials.add(handle.into());
}

fn spawn_car(commands: &mut Commands, m: Materials, tile_pos: TilePosition, speed: f32) {
    commands
        .spawn(SpriteBundle {
            material: m.suv_material,
            transform: Transform {
                scale: Vec3::new(if speed < 0.0 { -1.0 } else { 1.0 }, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Car)
        .with(PixelPosition(Vec2::new(
            tile_pos.0.x * TILE_SIZE as f32,
            tile_pos.0.y * TILE_SIZE as f32,
        )))
        .with(Velocity(Vec2::new(speed, 0.0))) // pixels per second
        .with(Hitbox::new(Vec2::new(0.0, 0.0), Vec2::new(14.0, 8.0)));
}

fn spawn_initial_cars(commands: &mut Commands, m: Res<Materials>, map: Res<Map>) {
    for car_data in map.cars.iter() {
        spawn_car(commands, m.clone(), car_data.tile_position, car_data.speed);
    }
}

fn spawn_another_car(
    commands: &mut Commands,
    events: Res<Events<GoingOffscreenEvent>>,
    mut event_reader: Local<EventReader<GoingOffscreenEvent>>,
    m: Res<Materials>,
) {
    for ev in event_reader.iter(&events) {
        let spawn_x = if ev.2 < 0.0 { 16.0 } else { -2.0 };
        spawn_car(
            commands,
            m.clone(),
            TilePosition(Vec2::new(spawn_x, ev.1)),
            ev.2,
        );
    }
}

struct FullyOffscreen;
struct GoingOffscreen;
fn fully_offscreen(
    mut q: Query<
        (Entity, &PixelPosition, &Hitbox),
        (With<GoingOffscreen>, Without<FullyOffscreen>, With<Car>),
    >,
    commands: &mut Commands,
) {
    for (entity, pos, hitbox) in q.iter_mut() {
        let left = pos.0.x;
        let right = pos.0.x + hitbox.size.x;
        let top = pos.0.y;
        let bottom = pos.0.y + hitbox.size.y;
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
        (Entity, &PixelPosition, &Hitbox, &Velocity),
        (Without<FullyOffscreen>, Without<GoingOffscreen>, With<Car>),
    >,
    commands: &mut Commands,
    mut ev_going_offscreen: ResMut<Events<GoingOffscreenEvent>>,
) {
    for (entity, pos, hitbox, velocity) in q.iter_mut() {
        let left_offscreen = (pos.0.x < 0.) && velocity.0.x < 0.0;
        let right_offscreen = (pos.0.x + hitbox.size.x > SCREEN_X_MAX as f32) && velocity.0.x > 0.0;
        let top_offscreen = (pos.0.y > SCREEN_Y_MAX as f32) && velocity.0.y > 0.0;
        let bottom_offscreen = (pos.0.y + hitbox.size.y < 0.0) && velocity.0.y < 0.0;
        if left_offscreen || right_offscreen || top_offscreen || bottom_offscreen {
            ev_going_offscreen.send(GoingOffscreenEvent(
                entity,
                pos.0.y / TILE_SIZE as f32,
                velocity.0.x,
            ));
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
                spawn_another_car.system(),
            )
            .on_state_update(APP_STATE_STAGE, AppState::InGame, fully_offscreen.system())
            .on_state_update(
                APP_STATE_STAGE,
                AppState::InGame,
                despawn_out_of_bounds.system(),
            )
            .on_state_update(APP_STATE_STAGE, AppState::InGame, going_offscreen.system());
    }
}
