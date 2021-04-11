use crate::animation::{Animation, AnimationFrame, Animations, Animator};
use crate::car::Car;
use crate::collisions::{CollisionEvent, Hurtbox};
use crate::consts::{AppState, SystemLabels, TILE_HEIGHT, TILE_WIDTH};
use crate::coordinates::{Layer, PixelPosition, SpriteSize, TilePosition, Velocity};
use crate::map::Map;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Clone, Default)]
struct Materials {
    player_material: Handle<ColorMaterial>,
}

const PLAYER_SPEED: f32 = 60.0;
pub struct Player;
struct CurrentPosition(TilePosition);
struct NextPosition(TilePosition);

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    map: Res<Map>,
) {
    let texture_handle = asset_server.load("sprites/shoe_animation.png");
    let sprite_size = SpriteSize(Vec2::new(8.0, 8.0));
    let texture_atlas = TextureAtlas::from_grid(texture_handle, sprite_size.0, 4, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_pos = TilePosition(Vec2::new(map.house.tile_x + 1.0, map.house.tile_y - 1.0));
    let player_layer = 2.0;
    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: player_pos.get_translation(Vec2::new(8.0, 8.0), player_layer),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(CurrentPosition(player_pos))
        .insert(PixelPosition(player_pos.get_pixel_position().0))
        .insert(Layer(player_layer))
        .insert(sprite_size)
        .insert(Hurtbox::new(Vec2::new(-0.5, 0.0), Vec2::new(7.0, 8.0)))
        .insert(Animator {
            current_animation: 0,
            last_animation: 0,
            current_frame: 0,
            timer: Timer::new(Default::default(), false),
        })
        .insert(Animations {
            animations: vec![
                // idle animation
                Animation {
                    frames: vec![
                        AnimationFrame {
                            atlas_handle: texture_atlas_handle.clone(),
                            atlas_index: 0,
                            duration: Duration::from_secs_f32(3.0 - 1.0 / 6.0),
                        },
                        AnimationFrame {
                            atlas_handle: texture_atlas_handle.clone(),
                            atlas_index: 3,
                            duration: Duration::from_secs_f32(1.0 / 6.0),
                        },
                    ],
                },
                // walk animation
                Animation {
                    frames: vec![
                        AnimationFrame {
                            atlas_handle: texture_atlas_handle.clone(),
                            atlas_index: 1,
                            duration: Duration::from_secs_f32(1.0 / 15.0),
                        },
                        AnimationFrame {
                            atlas_handle: texture_atlas_handle.clone(),
                            atlas_index: 2,
                            duration: Duration::from_secs_f32(1.0 / 15.0),
                        },
                    ],
                },
                // rolling animation
                Animation {
                    frames: vec![
                        AnimationFrame {
                            atlas_handle: texture_atlas_handle.clone(),
                            atlas_index: 4,
                            duration: Duration::from_secs_f32(1.0 / 15.0),
                        },
                        AnimationFrame {
                            atlas_handle: texture_atlas_handle.clone(),
                            atlas_index: 5,
                            duration: Duration::from_secs_f32(1.0 / 15.0),
                        },
                        AnimationFrame {
                            atlas_handle: texture_atlas_handle.clone(),
                            atlas_index: 6,
                            duration: Duration::from_secs_f32(1.0 / 15.0),
                        },
                        AnimationFrame {
                            atlas_handle: texture_atlas_handle,
                            atlas_index: 7,
                            duration: Duration::from_secs_f32(1.0 / 15.0),
                        },
                    ],
                },
            ],
        });
}

fn player_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (Entity, &CurrentPosition, &mut Animator, &Layer),
        (With<Player>, Without<NextPosition>),
    >,
) {
    for (player, current_position, mut animator, layer) in player_query.iter_mut() {
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

        animator.current_animation = 1;
        animator.current_frame = 0;
        commands.entity(player).insert(NextPosition(next_position));
        let current_translation = current_position
            .0
            .get_translation(Vec2::new(8.0, 8.0), layer.0);
        let next_translation = next_position.get_translation(Vec2::new(8.0, 8.0), layer.0);
        let direction = (next_translation - current_translation).normalize();
        commands
            .entity(player)
            .insert(Velocity(direction.truncate() * PLAYER_SPEED));
    }
}

fn player_movement_done(
    mut commands: Commands,
    mut player_query: Query<
        (
            Entity,
            &NextPosition,
            &Transform,
            &Velocity,
            &mut Animator,
            &Layer,
        ),
        With<Player>,
    >,
) {
    for (player, next_position, transform, v, mut animator, layer) in player_query.iter_mut() {
        let diff = next_position
            .0
            .get_translation(Vec2::new(8.0, 8.0), layer.0)
            - transform.translation;
        if diff.truncate().dot(v.0) <= 0.0 {
            let new_current_position = CurrentPosition(next_position.0);
            let new_pixel_position = new_current_position.0.get_pixel_position();
            commands
                .entity(player)
                .insert(new_current_position)
                .insert(new_pixel_position)
                .remove::<Velocity>()
                .remove::<NextPosition>();
            animator.current_animation = 0;
            animator.current_frame = 0;
        }
    }
}

const PLAYER_ROLLING_SPEED: f32 = 90.0;
fn player_collides_car(
    mut commands: Commands,
    mut event_reader: EventReader<CollisionEvent<Player, Car>>,
    mut player_query: Query<(Entity, &mut Animator, &PixelPosition, &Layer), With<Player>>,
    map: Res<Map>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if event_reader.iter().next().is_some() {
        for (player, mut animator, current_position, layer) in player_query.iter_mut() {
            if animator.current_animation == 2 {
                continue;
            }
            let spawn_pos = TilePosition(Vec2::new(map.house.tile_x + 1.0, map.house.tile_y - 1.0));
            commands.entity(player).insert(NextPosition(spawn_pos));

            let current_translation =
                current_position.get_translation(Vec2::new(8.0, 8.0), layer.0);
            let next_translation = spawn_pos.get_translation(Vec2::new(8.0, 8.0), layer.0);
            let direction = (next_translation - current_translation).normalize();
            commands
                .entity(player)
                .insert(Velocity(direction.truncate() * PLAYER_ROLLING_SPEED));
            animator.current_animation = 2;
            animator.current_frame = 0;
            let sfx = asset_server.load("sfx/honk.mp3");
            audio.play(sfx);
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Loading).with_system(setup_player.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(player_input.system().before(SystemLabels::PlayerMovement))
                .with_system(
                    player_movement_done
                        .system()
                        .label(SystemLabels::PlayerMovement),
                )
                .with_system(
                    player_collides_car
                        .system()
                        .after(SystemLabels::PlayerMovement)
                        .before("update_position"),
                ),
        );
    }
}
