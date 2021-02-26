use crate::consts::{AppState, APP_STATE_STAGE, SCALE, TILE_SIZE};
use bevy::prelude::*;

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

const map: Map = Map {
    rows: [
        MapRow { sprite: 0 },
        MapRow { sprite: 1 },
        MapRow { sprite: 2 },
        MapRow { sprite: 3 },
        MapRow { sprite: 4 },
        MapRow { sprite: 5 },
        MapRow { sprite: 6 },
        MapRow { sprite: 7 },
        MapRow { sprite: 8 },
        MapRow { sprite: 9 },
        MapRow { sprite: 10 },
        MapRow { sprite: 11 },
        MapRow { sprite: 0 },
        MapRow { sprite: 1 },
        MapRow { sprite: 2 },
        MapRow { sprite: 3 },
    ],
    house: House {
        tile_x: 0.5,
        tile_y: 0.5,
    },
    bus_stop: BusStop {
        tile_x: 14.5,
        tile_y: 14.5,
    }, 
    
    /*cars: vec![
        Car {
            x: 0,
            y: 0,
            speed: 0.5,
            hitbox_width: 13,
        }
    ]*/
};

#[derive(Default, Copy, Clone, PartialEq)]
pub struct TilePosition {
    pub x: f32,
    pub y: f32,
}

fn get_transform_vector_from_tile_coordinate(t: TilePosition, offset: f32) -> Vec3 {
    Vec3::new(
        (t.x as f32 * TILE_SIZE as f32 + offset) * SCALE,
        (t.y as f32 * TILE_SIZE as f32 + offset) * SCALE,
        0.0,
    )
}

fn load_map_atlas(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut state: ResMut<State<AppState>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.get_handle("map_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 4, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for r in 0..map.rows.len() {
        for c in 0..16 {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform {
                    translation: get_transform_vector_from_tile_coordinate(
                        TilePosition {
                            x: c as f32,
                            y: r as f32,
                        },
                        4.0,
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
                4.0,
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
                4.0,
            ),
            scale: Vec3::splat(SCALE),
            ..Default::default()
        },
        ..Default::default()
    });
    state.set_next(AppState::InGame).unwrap();
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(APP_STATE_STAGE, AppState::Loading, load_map_atlas.system());
    }
}
