use crate::consts::{AppState, APP_STATE_STAGE};
use crate::coordinates::{PixelPosition, Velocity};
use crate::player::Player;
use crate::animation::Animator;
use bevy::prelude::*;
use rand::Rng;

struct DustConfig {
    x_velocity_range: std::ops::Range<f32>, // 1 to 2
    y_velocity_range: std::ops::Range<f32>, // -3 to -1
    lifetime: f32,
    radius: f32,
    gravity: f32,
}
impl Default for DustConfig {
    fn default() -> Self {
        DustConfig {
            x_velocity_range: 1.0..2.0,
            y_velocity_range: -3.0..1.0,
            lifetime: 8.0 / 60.0, // secs
            radius: 1.0,
            gravity: 0.3,
        }
    }
}

struct Dust;
struct DustColor {
    material: Handle<ColorMaterial>,
}
struct Lifetime {
    current_lifetime: f32,
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(DustColor {
        material: materials.add(Color::rgb(0.7, 0.0, 0.0).into()),
    });
}

fn spawn_new_dust(
    commands: &mut Commands,
    player_query: Query<(&PixelPosition, &Animator), With<Player>>,
    dust_color: Res<DustColor>,
    dust_config: Res<DustConfig>,
) {
    let mut rng = rand::thread_rng();
    for (pixel_pos, animator) in player_query.iter() {
        if animator.current_animation == 2 {
            commands
            .spawn(SpriteBundle {
                material: dust_color.material.clone(),
                transform: Transform {
                    scale: Vec3::one() * 10.0,
                    translation: pixel_pos.get_translation(Vec2::new(2.0, 2.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Dust)
            .with(PixelPosition(pixel_pos.0))
            .with(Lifetime {
                current_lifetime: dust_config.lifetime,
            })
            .with(Velocity(Vec2::new(
                rng.gen_range(dust_config.x_velocity_range.clone()),
                rng.gen_range(dust_config.y_velocity_range.clone()),
            )));
        }
    }
}

fn update_dust_lifetime(
    commands: &mut Commands,
    mut query: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    // time += timestep;
    // position += timestep * (velocity + timestep * acceleration / 2);
    // velocity += timestep * acceleration;

    // x += dx;
    // y += dy;
    // dy += gravity;
    // radius *= 0.9;
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.current_lifetime -= time.delta_seconds();
        if lifetime.current_lifetime < 0.0 {
            commands.despawn(entity);
        }
    }
}

pub struct DustSystem;
impl Plugin for DustSystem {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<DustConfig>()
            .on_state_enter(APP_STATE_STAGE, AppState::InGame, setup.system())
            .on_state_update(APP_STATE_STAGE, AppState::InGame, spawn_new_dust.system())
            .on_state_update(
                APP_STATE_STAGE,
                AppState::InGame,
                update_dust_lifetime.system(),
            );
    }
}
