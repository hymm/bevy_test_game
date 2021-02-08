use bevy::prelude::*;
use bevy::{asset::LoadState, sprite::TextureAtlasBuilder};


const TILE_WIDTH: f32 = 16.0;
const TILE_HEIGHT: f32 = 16.0;
const SCALE: f32 = 4.0;
const TILE_SIZE: i32 = 8;
const SCREEN_X_MAX: i32 = TILE_WIDTH as i32 * TILE_SIZE * SCALE as i32;
const SCREEN_Y_MAX: i32 = TILE_HEIGHT as i32 * TILE_SIZE * SCALE as i32;

fn main() {
    App::build()
        .init_resource::<SpriteHandles>()
        .add_resource(WindowDescriptor {
            title: "Shoe Crosses the Road".to_string(),
            width: TILE_WIDTH * SCALE * TILE_SIZE as f32,
            height: TILE_HEIGHT * SCALE * TILE_SIZE as f32,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_resource(State::new(AppState::Setup))
        .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
        .on_state_enter(STAGE, AppState::Setup, load_textures.system())
        .on_state_update(STAGE, AppState::Setup, check_textures.system())
        .on_state_enter(STAGE, AppState::Finished, draw_map.system())
        .on_state_enter(STAGE, AppState::Finished, spawn_initial_cars.system())
        .add_system(update_position.system())
        .add_system(convert_tile_coord.system())
        .add_system(position_translation.system())
        .add_event::<FullyOffscreenEvent>()
        .add_event::<PartiallyOffscreenEvent>()
        .add_system(offscreen.system())
        .add_system(despawn_out_of_bounds.system())
        .run();
}

pub struct HelloPlugin;

struct MapTile {
    texture_handle: Handle<Texture>,
}

const STAGE: &str = "app_state";

#[derive(Clone)]
enum AppState {
    Setup,
    Finished,
}

#[derive(Default)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = asset_server.load_folder(".").unwrap();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set_next(AppState::Finished).unwrap();
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    sprite_handles: Res<SpriteHandles>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let vendor_handle = asset_server.get_handle("shoeroad_sprites.png");
    let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(vendor_index as u32),
            texture_atlas: atlas_handle,
            ..Default::default()
        });
}

struct MapRow {
    sprite: u32,
}

struct House {
    tile_x: f32,
    tile_y: f32,
}

struct BusStop {
    tile_x: f32,
    tile_y: f32,
}

struct Map {
    rows: [MapRow; 16],
    house: House,
    bus_stop: BusStop,
    // cars: Vec<Car>,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Default, Copy, Clone, PartialEq)]
struct TilePosition {
    x: f32,
    y: f32,
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

struct Player;
struct Car;
struct PartiallyOffscreenEvent(Entity, f32);
struct FullyOffscreenEvent(Entity);

const map: Map = Map {
    rows: [
        MapRow { sprite: 0, },
        MapRow { sprite: 1, },
        MapRow { sprite: 2, },
        MapRow { sprite: 3, },
        MapRow { sprite: 4, },
        MapRow { sprite: 5, },        
        MapRow { sprite: 6, },        
        MapRow { sprite: 7, },
        MapRow { sprite: 8, },
        MapRow { sprite: 9, },
        MapRow { sprite: 10, },
        MapRow { sprite: 11, },
        MapRow { sprite: 0, },
        MapRow { sprite: 1, },
        MapRow { sprite: 2, },
        MapRow { sprite: 3, },
    ],
    house: House {
        tile_x: 0.5,
        tile_y: 0.5,
    },
    bus_stop: BusStop {
        tile_x: 14.5,
        tile_y: 14.5,
    } 
    /*cars: vec![
        Car {
            x: 0,
            y: 0,
            speed: 0.5,
            hitbox_width: 13,
        }
    ]*/
};

fn get_transform_vector_from_tile_coordinate(t: TilePosition, offset: f32) -> Vec3 {
Vec3::new((t.x as f32 * TILE_SIZE as f32 + offset) * SCALE, (t.y as f32 * TILE_SIZE as f32 + offset) * SCALE, 0.0)
}

fn draw_map(
    commands: &mut Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // load sprite sheet as texture atlas
    
    let texture_handle = asset_server.get_handle("map_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 4, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // let vendor_index = texture_atlas.get_texture_index(&texture_handle).unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(64.0 * SCALE, 64.0 * SCALE, 1000.0 - 0.1),
            ..Default::default()
        },
        ..Camera2dBundle::default() 
    });
    
    for r in 0..map.rows.len() {
        for c in 0..16   {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform {
                    translation: get_transform_vector_from_tile_coordinate(
                        TilePosition { x: c as f32, y: r as f32 }, 4.0
                    ),
                    scale: Vec3::splat(SCALE),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(map.rows[r].sprite),
                ..Default::default()
            });
        }
    }

    let house_handle = asset_server.get_handle("house.png");
    commands.spawn(SpriteBundle {
        material: materials.add(house_handle.into()),
        transform: Transform {
            translation: get_transform_vector_from_tile_coordinate(
                TilePosition { 
                    x: map.house.tile_x, 
                    y: map.house.tile_y,
                },
                4.0
            ),
            scale: Vec3::splat(SCALE),
            ..Default::default()
        },
        ..Default::default()
    });

    let bus_stop_handle = asset_server.get_handle("bus_stop.png");
    commands.spawn(SpriteBundle {
        material: materials.add(bus_stop_handle.into()),
        transform: Transform {
            translation: get_transform_vector_from_tile_coordinate(
                TilePosition { 
                    x: map.bus_stop.tile_x, 
                    y: map.bus_stop.tile_y,
                },
                4.0
            ),
            scale: Vec3::splat(SCALE),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn convert_tile_coord(mut q: Query<(&mut Position, &TilePosition)>) {
    for (mut pos, tile_pos) in q.iter_mut() {
        pos.x = (tile_pos.x * TILE_SIZE as f32) as i32;
        pos.y = (tile_pos.y * TILE_SIZE as f32) as i32;
    }
}

fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(pos.x as f32, pos.y as f32, 0.0);
    }
}

fn spawn_car(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tile_pos: TilePosition,
) {
    let m = asset_server.get_handle("suv.png");
    commands
        .spawn(SpriteBundle {
            material: materials.add(m.into()),
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
        .with(Velocity {
            x: 1.0,
            y: 0.0,
        })
        .with(Hitbox {
            width: 14.0,
            height: 8.0,
            ..Default::default()
        });
}

fn spawn_initial_cars(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let tile_pos = TilePosition { x: 0.1, y: 6.0 };
    spawn_car(commands, asset_server, materials, tile_pos)
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
        if (right as i32) < 0 || (left as i32) > SCREEN_X_MAX
            || (top as i32) < 0 || (bottom as i32) > SCREEN_Y_MAX {
            ev_full.send(FullyOffscreenEvent(entity));
            continue;       
        }

        if (left as i32) < 0 || (right as i32) > SCREEN_X_MAX
            || (bottom as i32) < 0 || (top as i32) > SCREEN_Y_MAX {
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
        commands.despawn(ev.0);
    }
}

// fn spawn_another_car(
//     commands: &mut Commands,
//     events: Res<Events<PartiallyOffscreenEvent>>,
//     mut event_reader: Local<EventReader<PartiallyOffscreenEvent>>,
//     asset_server: Res<AssetServer>,
//     mut materials: ResMut<Assets<ColorMaterial>>
// ) {
//     for ev in event_reader.iter(&events) {
//         spawn_car(commands, asset_server, materials, TilePosition { x: 0.1, y: ev.1 });
//     }
// }


fn update_position(mut q: Query<(&Velocity, &mut Position)>) {
    for (v, mut p) in q.iter_mut() {
        p.x += (v.x * SCALE) as i32;
        p.y += (v.y * SCALE) as i32;
    }
}