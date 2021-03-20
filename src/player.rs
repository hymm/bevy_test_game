use bevy::prelude::*;
use crate::consts::{AppState, APP_STATE_STAGE};
use crate::coordinates::{PixelPosition, TilePosition};

#[derive(Clone, Default)]
struct Materials {
    player_material: Handle<ColorMaterial>,
}

struct Player;
struct CurrentPosition(TilePosition);
struct NextPosition(TilePosition);
struct CurrentPixelPosition(PixelPosition);

enum Direction {
  UP,
  DOWN,
  LEFT,
  RIGHT,
  NONE,
}

fn setup_player(
  commands: &mut Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let texture_handle = asset_server.load("shoe_walk.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 4, 1);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);
  let player_pos = TilePosition(Vec2::new(8.0, 8.0));
  commands
    .spawn(SpriteSheetBundle {
      texture_atlas: texture_atlas_handle,
      transform: Transform {
        translation: player_pos.get_translation(Vec2::new(8.0, 8.0)),
        ..Default::default()
      },
      ..Default::default()
    })
    .with(Player)
    .with(CurrentPosition(player_pos));
}

fn player_input(
  commands: &mut Commands,
  keyboard_input: Res<Input<KeyCode>>,
  player_query: Query<(Entity, &CurrentPosition), (With<Player>, Without<NextPosition>)>
) {
  for (player, current_pos) in player_query.iter() {
    let next = if keyboard_input.pressed(KeyCode::Left) {
      TilePosition(Vec2::new(current_pos.0.0.x - 1.0, current_pos.0.0.y))
    } else if keyboard_input.pressed(KeyCode::Right) {
      TilePosition(Vec2::new(current_pos.0.0.x + 1.0, current_pos.0.0.y))
    } else if keyboard_input.pressed(KeyCode::Up) {
      TilePosition(Vec2::new(current_pos.0.0.x, current_pos.0.0.y + 1.0))
    } else if keyboard_input.pressed(KeyCode::Down) {
      TilePosition(Vec2::new(current_pos.0.0.x, current_pos.0.0.y - 1.0))
    } else {
      return;
    };

    commands.insert_one(player, NextPosition(next));
  }
}

fn player_movement(
  commands: &mut Commands,
  mut player_query: Query<(Entity, &CurrentPosition, &NextPosition, &mut Transform), With<Player>>,
) {
  // TODO: limit updating to 60 times / sec
  for (player, current_position, next_position, mut transform) in player_query.iter_mut() {
    let current_translation = current_position.0.get_translation(Vec2::new(8.0, 8.0));
    let next_translation = next_position.0.get_translation(Vec2::new(8.0, 8.0));
    let direction = (next_translation - current_translation).normalize();
    transform.translation =  (transform.translation + direction).round();

    if transform.translation == next_translation {
      commands.insert_one(player, CurrentPosition(next_position.0));
      commands.remove_one::<NextPosition>(player);
    }
  }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .on_state_enter(
        APP_STATE_STAGE,
        AppState::Loading,
        setup_player.system(),
      )
      .on_state_update(
        APP_STATE_STAGE,
        AppState::InGame,
        player_input.system(),
      )
      .on_state_update(
        APP_STATE_STAGE,
        AppState::InGame,
        player_movement.system(),
      );
  }
}

