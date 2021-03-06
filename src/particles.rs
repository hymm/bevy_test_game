use crate::animation::Animator;
use crate::consts::AppState;
use crate::coordinates::{Acceleration, Layer, PixelPosition, Velocity};
use crate::player::Player;
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

struct ParticleConfig {
    x_velocity_range: std::ops::Range<f32>, // 1 to 2
    y_velocity_range: std::ops::Range<f32>, // -3 to -1
    lifetime: f32,
    gravity: f32,
}
impl Default for ParticleConfig {
    fn default() -> Self {
        ParticleConfig {
            x_velocity_range: -50.0..50.0,
            y_velocity_range: -20.0..80.0,
            lifetime: 16.0 / 60.0, // secs
            gravity: -300.0,
        }
    }
}

struct Particle;
struct ParticleSpawnTimer(Timer);
struct ParticleColor {
    material: Handle<ColorMaterial>,
}
impl FromWorld for ParticleColor {
    fn from_world(world: &mut World) -> Self {
        if let Some(mut materials) = world.get_resource_mut::<Assets<ColorMaterial>>() {
            ParticleColor {
                material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            }
        } else {
            panic!("Could not get materials to initialize dust color");
        }
    }
}

struct Lifetime {
    current_lifetime: f32,
}

#[derive(Bundle)]
struct ParticleBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    particle: Particle,
    dust_pos: PixelPosition,
    layer: Layer,
    lifetime: Lifetime,
    velocity: Velocity,
    acceleration: Acceleration,
}

fn spawn_new_dust(
    mut commands: Commands,
    player_query: Query<(&PixelPosition, &Animator), With<Player>>,
    dust_color: Res<ParticleColor>,
    config: Res<ParticleConfig>,
    mut timer: ResMut<ParticleSpawnTimer>,
    time: Res<Time>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }
    let mut rng = rand::thread_rng();
    let dust_layer = 3.0;
    for (player_pos, animator) in player_query.iter() {
        if animator.current_animation == 2 {
            let dust_pos = PixelPosition(player_pos.0 + Vec2::new(8.0, 4.0));
            commands.spawn().insert_bundle(ParticleBundle {
                sprite_bundle: SpriteBundle {
                    sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                    material: dust_color.material.clone(),
                    transform: Transform {
                        translation: dust_pos.get_translation(Vec2::new(1.0, 1.0), dust_layer),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                particle: Particle,
                dust_pos,
                layer: Layer(dust_layer),
                lifetime: Lifetime {
                    current_lifetime: config.lifetime,
                },
                velocity: Velocity(Vec2::new(
                    rng.gen_range(config.x_velocity_range.clone()),
                    rng.gen_range(config.y_velocity_range.clone()),
                )),
                acceleration: Acceleration(Vec2::new(0.0, config.gravity)),
            });
        }
    }
}

fn update_dust_lifetime(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.current_lifetime -= time.delta_seconds();
        if lifetime.current_lifetime < 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub struct ParticleSystem;
impl Plugin for ParticleSystem {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ParticleConfig>()
            .init_resource::<ParticleColor>()
            .insert_resource(ParticleSpawnTimer(Timer::new(
                Duration::from_millis((0.75 / 60.0 * 1000.0) as u64),
                true,
            )))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(spawn_new_dust.system())
                    .with_system(update_dust_lifetime.system()),
            );
    }
}
