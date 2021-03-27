use crate::car::Car;
use crate::collisions::{CollisionEvent, Hurtbox};
use crate::consts::{AppState, APP_STATE_STAGE, TILE_HEIGHT, TILE_WIDTH};
use crate::coordinates::{PixelPosition, SpriteSize, TilePosition, Velocity};
use crate::map::Map;
use crate::animation::{Animations, Animation, AnimationFrame, Animator};
use bevy::prelude::*;

#[derive(Clone, Default)]
struct Materials {
    player_material: Handle<ColorMaterial>,
}

const PLAYER_SPEED: f32 = 60.0;
pub struct Player;
struct CurrentPosition(TilePosition);
struct NextPosition(TilePosition);

enum PlayerStates {
    Idle,
    Walking,
    Rolling,
}

fn setup_player(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    map: Res<Map>,
) {
    let texture_handle = asset_server.load("shoe_animation.png");
    let sprite_size = SpriteSize(Vec2::new(8.0, 8.0));
    let texture_atlas = TextureAtlas::from_grid(texture_handle, sprite_size.0, 4, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_pos = TilePosition(Vec2::new(map.house.tile_x + 1.0, map.house.tile_y - 1.0));
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: player_pos.get_translation(Vec2::new(8.0, 8.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Player)
        .with(CurrentPosition(player_pos))
        .with(PixelPosition(player_pos.get_pixel_position().0))
        .with(sprite_size)
        .with(Hurtbox::new(Vec2::new(-0.5, 0.0), Vec2::new(7.0, 8.0)))
        .with(Animator {
            current_animation: 0,
            current_frame: 0,
            timer: Timer::new(Default::default(), false),
        })
        .with(Animations {
            animations: vec![
                // idle animation 
                Animation {
                    frames: vec![
                        AnimationFrame { atlas_handle: texture_atlas_handle.clone(), atlas_index: 0, duration: 3.0 - 1.0 / 6.0 },
                        AnimationFrame { atlas_handle: texture_atlas_handle.clone(), atlas_index: 3, duration: 1.0 / 6.0 }
                    ]
                },
                // walk animation
                Animation { 
                    frames: vec![
                        AnimationFrame { atlas_handle: texture_atlas_handle.clone(), atlas_index: 1, duration: 1.0 / 15.0 },
                        AnimationFrame { atlas_handle: texture_atlas_handle, atlas_index: 2, duration: 1.0 / 15.0 }
                    ]
                },
            ]
        });
}

fn player_input(
    commands: &mut Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(Entity, &CurrentPosition), (With<Player>, Without<NextPosition>)>,
) {
    for (player, current_position) in player_query.iter_mut() {
        let next_position = if keyboard_input.pressed(KeyCode::Left) {
            TilePosition(Vec2::new(
                current_position.0 .0.x - 1.0,
                current_position.0 .0.y,
            ))
        } else if keyboard_input.pressed(KeyCode::Right) {
            TilePosition(Vec2::new(
                current_position.0 .0.x + 1.0,
                current_position.0 .0.y,
            ))
        } else if keyboard_input.pressed(KeyCode::Up) {
            TilePosition(Vec2::new(
                current_position.0 .0.x,
                current_position.0 .0.y + 1.0,
            ))
        } else if keyboard_input.pressed(KeyCode::Down) {
            TilePosition(Vec2::new(
                current_position.0 .0.x,
                current_position.0 .0.y - 1.0,
            ))
        } else {
            return;
        };

        // limit player to screen bounds
        if next_position.0.x < 0.0
            || next_position.0.x > TILE_WIDTH - 1.0
            || next_position.0.y < 0.0
            || next_position.0.y > TILE_HEIGHT - 1.0
        {
            return;
        }

        commands.insert_one(player, NextPosition(next_position));
        let current_translation = current_position.0.get_translation(Vec2::new(8.0, 8.0));
        let next_translation = next_position.get_translation(Vec2::new(8.0, 8.0));
        let direction = (next_translation - current_translation).normalize();
        commands.insert_one(player, Velocity(direction.truncate() * PLAYER_SPEED));
    }
}

fn player_movement_done(
    commands: &mut Commands,
    mut player_query: Query<(Entity, &NextPosition, &Transform, &Velocity), With<Player>>,
) {
    for (player, next_position, transform, v) in player_query.iter_mut() {
        let diff = next_position.0.get_translation(Vec2::new(8.0, 8.0)) - transform.translation;
        if diff.truncate().dot(v.0) <= 0.0 {
            let new_current_position = CurrentPosition(next_position.0);
            let new_pixel_position = new_current_position.0.get_pixel_position();
            commands.insert_one(player, new_current_position);
            commands.insert_one(player, new_pixel_position);
            commands.remove_one::<Velocity>(player);
            commands.remove_one::<NextPosition>(player);
        }
    }
}

fn player_collides_car(
    commands: &mut Commands,
    events: Res<Events<CollisionEvent<Player, Car>>>,
    mut event_reader: Local<EventReader<CollisionEvent<Player, Car>>>,
    mut player_query: Query<(Entity, &mut PixelPosition, &mut CurrentPosition), With<Player>>,
    map: Res<Map>,
) {
    if event_reader.iter(&events).next().is_some() {
        for (player, mut pixel_position, mut current_position) in player_query.iter_mut() {
            let spawn_pos = TilePosition(Vec2::new(map.house.tile_x + 1.0, map.house.tile_y - 1.0));
            current_position.0 = spawn_pos;
            *pixel_position = spawn_pos.get_pixel_position();
            commands.remove_one::<Velocity>(player);
            commands.remove_one::<NextPosition>(player);
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(APP_STATE_STAGE, AppState::Loading, setup_player.system())
            .on_state_update(APP_STATE_STAGE, AppState::InGame, player_input.system())
            .on_state_update(
                APP_STATE_STAGE,
                AppState::InGame,
                player_movement_done.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::InGame,
                player_collides_car.system(),
            );
    }
}
