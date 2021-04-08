use crate::collisions::Hitbox;
use crate::consts::{AppState, SCREEN_X_MAX, SCREEN_Y_MAX, TILE_SIZE};
use crate::coordinates::{PixelPosition, TilePosition, Velocity, Layer};
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
        .spawn()
        .insert_bundle(SpriteBundle {
            material: m.suv_material,
            transform: Transform {
                scale: Vec3::new(if speed < 0.0 { -1.0 } else { 1.0 }, 1.0, 1.0),
                translation: tile_pos.get_translation(Vec2::new(14.0, 8.0), 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Car)
        .insert(Layer(1.0))
        .insert(PixelPosition(Vec2::new(
            tile_pos.0.x * TILE_SIZE as f32,
            tile_pos.0.y * TILE_SIZE as f32,
        )))
        .insert(Velocity(Vec2::new(speed, 0.0))) // pixels per second
        .insert(Hitbox::new(Vec2::new(0.0, 0.0), Vec2::new(14.0, 8.0)));
}

fn spawn_initial_cars(mut commands: Commands, m: Res<Materials>, map: Res<Map>) {
    for car_data in map.cars.iter() {
        spawn_car(&mut commands, m.clone(), car_data.tile_position, car_data.speed);
    }
}

fn spawn_another_car(
    mut commands: Commands,
    mut event_reader: EventReader<GoingOffscreenEvent>,
    m: Res<Materials>,
) {
    for ev in event_reader.iter() {
        let spawn_x = if ev.2 < 0.0 { 16.0 } else { -2.0 };
        spawn_car(
            &mut commands,
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
    mut commands: Commands,
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
            commands.entity(entity).insert(FullyOffscreen);
        }
    }
}

fn going_offscreen(
    mut q: Query<
        (Entity, &PixelPosition, &Hitbox, &Velocity),
        (Without<FullyOffscreen>, Without<GoingOffscreen>, With<Car>),
    >,
    mut commands: Commands,
    mut ev_going_offscreen: EventWriter<GoingOffscreenEvent>,
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
            commands.entity(entity).insert(GoingOffscreen);
        }
    }
}

fn despawn_out_of_bounds(
    mut commands: Commands,
    mut q: Query<Entity, (With<GoingOffscreen>, With<FullyOffscreen>)>,
) {
    for entity in q.iter_mut() {
        commands.entity(entity).despawn();
    }
}

pub struct CarPlugin;
impl Plugin for CarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Materials>()
            .add_event::<GoingOffscreenEvent>()
            .add_system_set(
                SystemSet::on_enter(AppState::Loading)
                    .with_system(store_car_material.system())
                    .with_system(spawn_initial_cars.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(spawn_another_car.system())
                    .with_system(fully_offscreen.system())
                    .with_system(despawn_out_of_bounds.system())
                    .with_system(going_offscreen.system()),
            );
    }
}
