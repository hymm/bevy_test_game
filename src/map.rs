use crate::consts::{AppState, APP_STATE_STAGE, TILE_SIZE};
use crate::coordinates::TilePosition;
use bevy::{prelude::*, reflect::TypeUuid};
use bevy_asset_ron::RonAssetPlugin;
use ron::{
    de::from_reader,
    ser::{to_writer_pretty, PrettyConfig},
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};

#[derive(Serialize, Deserialize)]
pub struct MapRow {
    sprite: u32,
}

#[derive(Serialize, Deserialize)]
pub struct House {
    pub tile_x: f32,
    pub tile_y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct BusStop {
    pub tile_x: f32,
    pub tile_y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct CarData {
    pub tile_position: TilePosition,
    pub speed: f32,
}

#[derive(Serialize, Deserialize, TypeUuid)]
#[uuid = "c57b6443-0ba6-4beb-82a9-a4d0948f99f5"]
pub struct Map {
    pub rows: [MapRow; 16],
    pub house: House,
    pub bus_stop: BusStop,
    pub cars: Vec<CarData>,
}
impl FromResources for Map {
    fn from_resources(_: &Resources) -> Self {
        Map {
            rows: [
                MapRow { sprite: 0 },
                MapRow { sprite: 1 },
                MapRow { sprite: 0 },
                MapRow { sprite: 1 },
                MapRow { sprite: 0 },
                MapRow { sprite: 1 },
                MapRow { sprite: 0 },
                MapRow { sprite: 3 },
                MapRow { sprite: 11 },
                MapRow { sprite: 1 },
                MapRow { sprite: 0 },
                MapRow { sprite: 1 },
                MapRow { sprite: 0 },
                MapRow { sprite: 1 },
                MapRow { sprite: 0 },
                MapRow { sprite: 1 },
            ],
            house: House {
                tile_x: 7.0,
                tile_y: 10.0,
            },
            bus_stop: BusStop {
                tile_x: 7.0,
                tile_y: 5.0,
            },
            cars: vec![
                CarData {
                    tile_position: TilePosition(Vec2::new(-2.0, 8.0)),
                    speed: 30.0,
                },
                CarData {
                    tile_position: TilePosition(Vec2::new(16.0, 7.0)),
                    speed: -30.0,
                },
            ],
        }
    }
}

pub struct CurrentMap(Handle<Map>);

fn load_map_atlas(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut state: ResMut<State<AppState>>,
    maps: Res<Assets<Map>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let map_handle: Handle<Map> = asset_server.load("levels/level_1.map");
    let map = maps.get(&map_handle).unwrap();

    let texture_handle = asset_server.get_handle("sprites/map_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        4,
        3,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for r in 0..map.rows.len() {
        for c in 0..16 {
            let spr = TextureAtlasSprite::new(map.rows[r].sprite);
            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform {
                    // order rows from top down
                    translation: TilePosition(Vec2::new(c as f32, (15 - r) as f32))
                        .get_translation(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                    ..Default::default()
                },
                sprite: spr,
                ..Default::default()
            });
        }
    }

    let house_handle = asset_server.get_handle("sprites/house.png");
    commands.spawn(SpriteBundle {
        material: materials.add(house_handle.into()),
        transform: Transform {
            translation: TilePosition(Vec2::new(map.house.tile_x, map.house.tile_y))
                .get_translation(Vec2::new(16., 16.)),
            ..Default::default()
        },
        ..Default::default()
    });

    let bus_stop_handle = asset_server.get_handle("sprites/bus_stop.png");
    commands.spawn(SpriteBundle {
        material: materials.add(bus_stop_handle.into()),
        transform: Transform {
            translation: TilePosition(Vec2::new(map.bus_stop.tile_x, map.bus_stop.tile_y))
                .get_translation(Vec2::new(16., 16.)),
            ..Default::default()
        },
        ..Default::default()
    });
    state.set_next(AppState::InGame).unwrap();
}

// fn save_map_to_file(map: &Map, path: &str) {
//     let file = File::create(format!("assets/levels/{}", path)).expect("Couldn't open file");
//     let pretty = PrettyConfig::new();
//     to_writer_pretty(file, &map, pretty).expect("Serialization failed");
// }

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Map>()
            .add_plugin(RonAssetPlugin::<Map>::new(&["map"]))
            .on_state_enter(APP_STATE_STAGE, AppState::Loading, load_map_atlas.system());
    }
}
