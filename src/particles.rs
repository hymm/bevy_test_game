use crate::animation::Animator;
use crate::consts::AppState;
use crate::coordinates::{Acceleration, Layer, PixelPosition, Velocity};
use crate::player::Player;
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

struct DustConfig {
    x_velocity_range: std::ops::Range<f32>, // 1 to 2
    y_velocity_range: std::ops::Range<f32>, // -3 to -1
    lifetime: f32,
    gravity: f32,
}
impl Default for DustConfig {
    fn default() -> Self {
        DustConfig {
            x_velocity_range: -50.0..50.0,
            y_velocity_range: -20.0..80.0,
            lifetime: 16.0 / 60.0, // secs
            gravity: -300.0,
        }
    }
}

struct Dust;
struct DustSpawnTimer(Timer);
struct DustColor {
    material: Handle<ColorMaterial>,
}
impl FromWorld for DustColor {
    fn from_world(world: &mut World) -> Self {
        if let Some(mut materials) = world.get_resource_mut::<Assets<ColorMaterial>>() {
            DustColor {
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

fn spawn_new_dust(
    mut commands: Commands,
    player_query: Query<(&PixelPosition, &Animator), With<Player>>,
    dust_color: Res<DustColor>,
    config: Res<DustConfig>,
    mut timer: ResMut<DustSpawnTimer>,
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
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                    material: dust_color.material.clone(),
                    transform: Transform {
                        translation: dust_pos.get_translation(Vec2::new(1.0, 1.0), dust_layer),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Dust)
                .insert(dust_pos)
                .insert(Layer(dust_layer))
                .insert(Lifetime {
                    current_lifetime: config.lifetime,
                })
                .insert(Velocity(Vec2::new(
                    rng.gen_range(config.x_velocity_range.clone()),
                    rng.gen_range(config.y_velocity_range.clone()),
                )))
                .insert(Acceleration(Vec2::new(0.0, config.gravity)));
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

pub struct DustSystem;
impl Plugin for DustSystem {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<DustConfig>()
            .init_resource::<DustColor>()
            .insert_resource(DustSpawnTimer(Timer::new(
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
