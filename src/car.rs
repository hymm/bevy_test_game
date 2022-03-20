use crate::collisions::Hitbox;
use crate::consts::{AppState, SCREEN_X_MAX, SCREEN_Y_MAX, TILE_SIZE};
use crate::coordinates::{Layer, PixelPosition, SpriteSize, TilePosition, Velocity};
use crate::map::CurrentLevel;
use crate::rng_bag::RngBag;
use bevy::prelude::*;
use bevy_prototype_schedule_states::AppStateHelpers;

#[derive(Component)]
pub struct Car;
struct GoingOffscreenEvent(Entity, f32, f32);

#[derive(Clone, Default)]
struct Materials {
    suv_material: Handle<TextureAtlas>,
}

struct ColorBag(pub RngBag<usize>);
impl Default for ColorBag {
    fn default() -> ColorBag {
        ColorBag(RngBag::<usize>::new(vec![0, 0, 0, 1, 2, 3, 4, 5]))
    }
}

fn store_car_material(
    mut m: ResMut<Materials>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/suv.png");
    let sprite_size = SpriteSize(Vec2::new(14.0, 8.0));
    let texture_atlas = TextureAtlas::from_grid(texture_handle, sprite_size.0, 6, 1);
    m.suv_material = texture_atlases.add(texture_atlas);
}

#[derive(Bundle)]
struct CarBundle {
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    car: Car,
    layer: Layer,
    pixel_position: PixelPosition,
    velocity: Velocity,
    hitbox: Hitbox,
    sprite_size: SpriteSize,
}

fn spawn_car(
    commands: &mut Commands,
    m: Materials,
    tile_pos: TilePosition,
    speed: f32,
    colors: &mut ColorBag,
) {
    let travelling_left = speed < 0.0;
    commands.spawn_bundle(CarBundle {
        sprite_bundle: SpriteSheetBundle {
            texture_atlas: m.suv_material.clone(),
            transform: Transform {
                scale: Vec3::new(if travelling_left { -1.0 } else { 1.0 }, 1.0, 1.0),
                translation: tile_pos.get_translation(Vec2::new(14.0, 8.0), 1.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                index: colors.0.get(),
                ..Default::default()
            },
            ..Default::default()
        },
        car: Car,
        sprite_size: SpriteSize(Vec2::new(14.0, 8.0)),
        layer: Layer(1.0),
        pixel_position: PixelPosition(Vec2::new(
            tile_pos.0.x * TILE_SIZE as f32 + if travelling_left { 0.0 } else { 2.0 },
            tile_pos.0.y * TILE_SIZE as f32,
        )),
        velocity: Velocity(Vec2::new(speed, 0.0)),
        hitbox: Hitbox::new(Vec2::new(0.0, 0.0), Vec2::new(14.0, 8.0)),
    });
}

fn spawn_initial_cars(
    mut commands: Commands,
    m: Res<Materials>,
    current_level: Res<CurrentLevel>,
    mut color_bag: ResMut<ColorBag>,
) {
    for car_data in current_level.0.cars.iter() {
        spawn_car(
            &mut commands,
            m.clone(),
            car_data.tile_position,
            car_data.speed,
            &mut color_bag,
        );
    }
}

fn spawn_another_car(
    mut commands: Commands,
    mut event_reader: EventReader<GoingOffscreenEvent>,
    m: Res<Materials>,
    mut color_bag: ResMut<ColorBag>,
) {
    for ev in event_reader.iter() {
        let spawn_x = if ev.2 < 0.0 { 16.0 } else { -2.0 };
        spawn_car(
            &mut commands,
            m.clone(),
            TilePosition(Vec2::new(spawn_x, ev.1)),
            ev.2,
            &mut color_bag,
        );
    }
}

#[derive(Component)]
struct FullyOffscreen;
fn fully_offscreen(
    mut q: Query<
        (Entity, &PixelPosition, &Hitbox, &Velocity),
        (Without<FullyOffscreen>, With<Car>),
    >,
    mut commands: Commands,
    mut ev_going_offscreen: EventWriter<GoingOffscreenEvent>,
) {
    for (entity, pos, hitbox, velocity) in q.iter_mut() {
        let left = pos.0.x;
        let right = pos.0.x + hitbox.size.x;
        let top = pos.0.y;
        let bottom = pos.0.y + hitbox.size.y;
        if (right < 0.0 && velocity.0.x < 0.0)
            || (left > SCREEN_X_MAX as f32 && velocity.0.x > 0.0)
            || (top < 0.0 && velocity.0.y < 0.0)
            || (bottom > SCREEN_Y_MAX as f32 && velocity.0.y > 0.0)
        {
            commands.entity(entity).insert(FullyOffscreen);
            ev_going_offscreen.send(GoingOffscreenEvent(
                entity,
                pos.0.y / TILE_SIZE as f32,
                velocity.0.x,
            ));
        }
    }
}

// fn going_offscreen(
//     mut q: Query<
//         (Entity, &PixelPosition, &Hitbox, &Velocity),
//         (Without<FullyOffscreen>, Without<GoingOffscreen>, With<Car>),
//     >,
//     mut commands: Commands,
//     mut ev_going_offscreen: EventWriter<GoingOffscreenEvent>,
// ) {
//     for (entity, pos, hitbox, velocity) in q.iter_mut() {
//         let left_offscreen = (pos.0.x < 0.) && velocity.0.x < 0.0;
//         let right_offscreen = (pos.0.x + hitbox.size.x > SCREEN_X_MAX as f32) && velocity.0.x > 0.0;
//         let top_offscreen = (pos.0.y > SCREEN_Y_MAX as f32) && velocity.0.y > 0.0;
//         let bottom_offscreen = (pos.0.y + hitbox.size.y < 0.0) && velocity.0.y < 0.0;
//         if left_offscreen || right_offscreen || top_offscreen || bottom_offscreen {
//             ev_going_offscreen.send(GoingOffscreenEvent(
//                 entity,
//                 pos.0.y / TILE_SIZE as f32,
//                 velocity.0.x,
//             ));
//             commands.entity(entity).insert(GoingOffscreen);
//         }
//     }
// }

fn despawn_out_of_bounds(mut commands: Commands, mut q: Query<Entity, With<FullyOffscreen>>) {
    for entity in q.iter_mut() {
        commands.entity(entity).despawn();
    }
}

pub struct CarPlugin;
impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Materials>()
            .init_resource::<ColorBag>()
            .add_event::<GoingOffscreenEvent>()
            .add_system_set_to_state_enter(
                AppState::Loading,
                SystemSet::new()
                    .with_system(store_car_material.before("spawn_initial_cars"))
                    .with_system(
                        spawn_initial_cars
                            .label("spawn_initial_cars")
                            .after("load_current_map"),
                    ),
            )
            .add_system_set_to_state_update(
                AppState::InGame,
                SystemSet::new()
                    .with_system(spawn_another_car.system().label("spawn_another_car"))
                    .with_system(
                        fully_offscreen
                            .label("fully_offscreen")
                            .before("spawn_another_car"),
                    )
                    .with_system(despawn_out_of_bounds.after("fully_offscreen")),
            );
    }
}
