use crate::collisions::Hitbox;
use crate::consts::{AppState, APP_STATE_STAGE, SCREEN_X_MAX, SCREEN_Y_MAX, TILE_SIZE};
use crate::coordinates::{PixelPosition, TilePosition, Velocity};
use bevy::prelude::*;

pub struct Car;
struct GoingOffscreenEvent(Entity, f32);

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
            material: m.suv_material,
            ..Default::default()
        })
        .with(Car)
        .with(PixelPosition(Vec2::new(
            tile_pos.0.x * TILE_SIZE as f32,
            tile_pos.0.y * TILE_SIZE as f32,
        )))
        .with(Velocity(Vec2::new(30.0, 0.0))) // pixels per second
        .with(Hitbox::new(Vec2::new(-7.0, -4.0), Vec2::new(14.0, 8.0)));
}

fn spawn_initial_cars(commands: &mut Commands, m: Res<Materials>) {
    spawn_car(commands, m.clone(), TilePosition(Vec2::new(-2.0, 8.0)));
    spawn_car(commands, m.clone(), TilePosition(Vec2::new(7.0, 7.0)));
}

fn spawn_another_car(
    commands: &mut Commands,
    events: Res<Events<GoingOffscreenEvent>>,
    mut event_reader: Local<EventReader<GoingOffscreenEvent>>,
    m: Res<Materials>,
) {
    for ev in event_reader.iter(&events) {
        spawn_car(commands, m.clone(), TilePosition(Vec2::new(-2.0, ev.1)));
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
        let left = pos.0.x + hitbox.offset.x - hitbox.size.x / 2.;
        let right = pos.0.x + hitbox.offset.x + hitbox.size.x / 2.;
        let top = pos.0.y + hitbox.offset.y - hitbox.size.y / 2.;
        let bottom = pos.0.y + hitbox.offset.y + hitbox.size.y / 2.;
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
        let left_offscreen =
            (pos.0.x + hitbox.offset.x - hitbox.size.x / 2. < 0.) && velocity.0.x < 0.0;
        let right_offscreen = (pos.0.x + hitbox.offset.x + hitbox.size.x / 2.
            > SCREEN_X_MAX as f32)
            && velocity.0.x > 0.0;
        let top_offscreen = (pos.0.y + hitbox.offset.y - hitbox.size.y / 2. > SCREEN_Y_MAX as f32)
            && velocity.0.y > 0.0;
        let bottom_offscreen =
            (pos.0.y + hitbox.offset.y + hitbox.offset.y / 2. < 0.0) && velocity.0.y < 0.0;
        if left_offscreen || right_offscreen || top_offscreen || bottom_offscreen {
            ev_going_offscreen.send(GoingOffscreenEvent(entity, pos.0.y / TILE_SIZE as f32));
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
