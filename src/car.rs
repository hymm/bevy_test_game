use bevy::prelude::*;
use crate::consts::{TILE_SIZE, SCALE, APP_STATE_STAGE, AppState, SCREEN_X_MAX, SCREEN_Y_MAX};
use crate::map::TilePosition;

struct Car;
struct PartiallyOffscreenEvent(Entity, f32);
struct FullyOffscreenEvent(Entity);

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
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
) 
{   
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
          x: (tile_pos.x * TILE_SIZE as f32 * SCALE) as i32,
          y: (tile_pos.y * TILE_SIZE as f32 * SCALE) as i32,
      })
      .with(Velocity { x: 0.5, y: 0.0 })
      .with(Hitbox {
          width: 14.0,
          height: 8.0,
          ..Default::default()
      });
}

fn spawn_initial_cars(commands: &mut Commands, m: Res<Materials>) {
  let tile_pos = TilePosition { x: 0.1, y: 6.0 };
  spawn_car(commands, m.clone(), tile_pos)
}

fn spawn_another_car(
  commands: &mut Commands,
  events: Res<Events<PartiallyOffscreenEvent>>,
  mut event_reader: Local<EventReader<PartiallyOffscreenEvent>>,
  m: Res<Materials>,
) {
  for ev in event_reader.iter(&events) {
      println!("spawn another car");
      spawn_car(commands, m.clone(), TilePosition { x: 0.1, y: ev.1 });
  }
}


fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
  for (pos, mut transform) in q.iter_mut() {
      transform.translation = Vec3::new(pos.x as f32, pos.y as f32, 0.0);
  }
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
      println!("despawn car");
      commands.despawn(ev.0);
  }
}

fn update_position(mut q: Query<(&Velocity, &mut Position)>) {
  for (v, mut p) in q.iter_mut() {
      p.x += (v.x * SCALE) as i32;
      p.y += (v.y * SCALE) as i32;
  }
}

pub struct CarPlugin;
impl Plugin for CarPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
    .init_resource::<Materials>()
    .add_event::<FullyOffscreenEvent>()
    .add_event::<PartiallyOffscreenEvent>()
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
    .on_state_update(
      APP_STATE_STAGE,
      AppState::InGame,
      update_position.system(),
    )
    .on_state_update(
      APP_STATE_STAGE,
      AppState::InGame,
      offscreen.system(),
    )
    .on_state_update(
      APP_STATE_STAGE,
      AppState::InGame,
      despawn_out_of_bounds.system(),
    );
  }
}