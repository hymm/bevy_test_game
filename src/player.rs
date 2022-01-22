use crate::animation::{Animation, AnimationFrame, Animations, Animator};
use crate::car::Car;
use crate::collisions::{CollisionEvent, Hurtbox};
use crate::consts::{AppState, SystemLabels, TILE_HEIGHT, TILE_WIDTH};
use crate::coordinates::{Layer, PixelPosition, SpriteSize, TilePosition, Velocity};
use crate::map::{CurrentLevel, Wall};
use bevy::prelude::*;
use std::time::Duration;

const PLAYER_SPEED: f32 = 60.0;
#[derive(Component)]
pub struct Player;
#[derive(Component)]
struct CurrentPosition(TilePosition);
#[derive(Component)]
struct NextPosition(Option<TilePosition>);

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    player: Player,
    current_position: CurrentPosition,
    next_position: NextPosition,
    pixel_position: PixelPosition,
    layer: Layer,
    sprite_size: SpriteSize,
    hurtbox: Hurtbox,
    animator: Animator,
    animations: Animations,
    velocity: Velocity,
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    current_level: Res<CurrentLevel>,
) {
    let texture_handle = asset_server.load("sprites/shoe_animation.png");
    let sprite_size = SpriteSize(Vec2::new(8.0, 8.0));
    let texture_atlas = TextureAtlas::from_grid(texture_handle, sprite_size.0, 4, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_pos = TilePosition(Vec2::new(
        current_level.0.house.tile_x + 1.0,
        current_level.0.house.tile_y - 1.0,
    ));
    let player_layer = 2.0;
    commands.spawn_bundle(PlayerBundle {
        sprite_bundle: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: player_pos.get_translation(Vec2::new(8.0, 8.0), player_layer),
                ..Default::default()
            },
            ..Default::default()
        },
        player: Player,
        current_position: CurrentPosition(player_pos),
        next_position: NextPosition(None),
        pixel_position: PixelPosition(player_pos.get_pixel_position().0),
        velocity: Velocity(Vec2::new(0.0, 0.0)),
        layer: Layer(player_layer),
        sprite_size,
        hurtbox: Hurtbox::new(Vec2::new(-0.5, 0.0), Vec2::new(7.0, 8.0)),
        animator: Animator::default(),
        animations: Animations {
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
        },
    });
}

fn player_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            Entity,
            &CurrentPosition,
            &mut Animator,
            &Layer,
            &NextPosition,
        ),
        With<Player>,
    >,
) {
    for (player, current_position, mut animator, layer, next_position) in player_query.iter_mut() {
        if next_position.0 != None {
            continue;
        }

        let mut next_position = TilePosition(current_position.0 .0);
        if keyboard_input.pressed(KeyCode::Left) {
            next_position.0.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            next_position.0.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            next_position.0.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            next_position.0.y -= 1.0;
        }

        if next_position == current_position.0 {
            return;
        }

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
        commands
            .entity(player)
            .insert(NextPosition(Some(next_position)));
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
        if let Some(next_position) = next_position.0 {
            let diff =
                next_position.get_translation(Vec2::new(8.0, 8.0), layer.0) - transform.translation;
            if diff.truncate().dot(v.0) <= 0.0 {
                let new_current_position = CurrentPosition(next_position);
                let new_pixel_position = new_current_position.0.get_pixel_position();
                commands
                    .entity(player)
                    .insert(new_current_position)
                    .insert(new_pixel_position)
                    .insert(Velocity(Vec2::new(0.0, 0.0)))
                    .insert(NextPosition(None));
                animator.current_animation = 0;
                animator.current_frame = 0;
            }
        }
    }
}

fn player_step_sfx(
    player_query: Query<&Animator, With<Player>>,
    mut last_frame: Local<usize>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let animator = player_query.single();
    if animator.current_animation == 1 && *last_frame != animator.current_frame {
        let sfx = asset_server.load("sfx/step.mp3");
        audio.play(sfx);
    }
    *last_frame = animator.current_frame;
}

const PLAYER_ROLLING_SPEED: f32 = 60.0;
fn player_collides_car(
    mut commands: Commands,
    mut event_reader: EventReader<CollisionEvent<Player, Car>>,
    mut player_query: Query<(Entity, &mut Animator, &PixelPosition, &Layer), With<Player>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    level: Res<CurrentLevel>,
) {
    if event_reader.iter().next().is_some() {
        let (player, mut animator, current_position, layer) = player_query.single_mut();

        if animator.current_animation == 2 {
            return;
        }
        let spawn_pos = TilePosition(Vec2::new(
            level.0.house.tile_x + 1.0,
            level.0.house.tile_y - 1.0,
        ));
        commands
            .entity(player)
            .insert(NextPosition(Some(spawn_pos)));

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

fn player_collides_wall(
    mut commands: Commands,
    mut event_reader: EventReader<CollisionEvent<Player, Wall>>,
    mut player_query: Query<(Entity, &mut Animator, &CurrentPosition), With<Player>>,
) {
    if event_reader.iter().next().is_some() {
        let (player, mut animator, current_position) = player_query.single_mut();
        
        if animator.current_animation == 2 {
            return;
        }
        commands
            .entity(player)
            .insert(current_position.0.get_pixel_position())
            .insert(Velocity(Vec2::new(0.0, 0.0)))
            .insert(NextPosition(None));
        animator.current_animation = 0;
        animator.current_frame = 0;
    }
}

fn level_complete(
    mut state: ResMut<State<AppState>>,
    player_query: Query<&CurrentPosition, With<Player>>,
    level: Res<CurrentLevel>,
) {
    let current_position = player_query.single();
    
    if (current_position.0 .0.y - level.0.bus_stop.tile_y - 1.0).abs() < 0.1 {
        state.set(AppState::LevelDone).unwrap();
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Loading)
                .with_system(setup_player.system().after("load_current_map")),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(player_input.system().before(SystemLabels::PlayerMovement))
                .with_system(player_step_sfx.system().after(SystemLabels::PlayerMovement))
                .with_system(
                    player_movement_done
                        .system()
                        .label(SystemLabels::PlayerMovement),
                )
                .with_system(level_complete.system().after(SystemLabels::PlayerMovement))
                .with_system(
                    player_collides_car
                        .system()
                        .after(SystemLabels::PlayerMovement),
                )
                .with_system(
                    player_collides_wall
                        .system()
                        .after(SystemLabels::PlayerMovement),
                ),
        );
    }
}
