use crate::collisions::Hitbox;
use crate::consts::{AppState, TILE_SIZE};
use crate::coordinates::{Layer, TilePosition};
use bevy::{prelude::*, reflect::TypeUuid};
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

pub struct Levels {
    pub current_level: usize,
    pub levels: Vec<String>,
}
impl FromWorld for Levels {
    fn from_world(_: &mut World) -> Self {
        Levels {
            current_level: 0,
            levels: vec![
                "4_cars_with_walls.map".to_string(),
                "level_2.map".to_string(),
            ],
        }
    }
}

pub struct Wall;

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

#[derive(Serialize, Deserialize)]
pub struct MapWallRow {
    row: i32,
    columns: [bool; 16],
}

#[derive(Serialize, Deserialize, TypeUuid)]
#[uuid = "c57b6443-0ba6-4beb-82a9-a4d0948f99f5"]
pub struct Map {
    pub rows: [MapRow; 16],
    pub house: House,
    pub bus_stop: BusStop,
    pub cars: Vec<CarData>,
    pub walls: Vec<MapWallRow>,
}

pub struct CurrentLevel(pub Map);
impl Default for CurrentLevel {
    fn default() -> Self {
        CurrentLevel(Map {
            rows: [
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
                MapRow { sprite: 0 },
            ],
            house: House {
                tile_x: 7.0,
                tile_y: 10.0,
            },
            bus_stop: BusStop {
                tile_x: 7.0,
                tile_y: 5.0,
            },
            cars: vec![],
            walls: vec![],
        })
    }
}

fn load_current_map(levels: Res<Levels>, mut current_level: ResMut<CurrentLevel>) {
    current_level.0 = load_map(&levels.levels[levels.current_level]);
}

fn load_map_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut state: ResMut<State<AppState>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    levels: Res<Levels>,
) {
    let map = load_map(&levels.levels[levels.current_level]);

    let texture_handle = asset_server.get_handle("sprites/map_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        4,
        3,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let tile_layer = 0.0;

    for r in 0..map.rows.len() {
        for c in 0..16 {
            let spr = TextureAtlasSprite::new(map.rows[r].sprite);
            commands
                .spawn()
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform {
                        // order rows from top down
                        translation: TilePosition(Vec2::new(c as f32, (15 - r) as f32))
                            .get_translation(
                                Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
                                tile_layer,
                            ),
                        ..Default::default()
                    },
                    sprite: spr,
                    ..Default::default()
                })
                .insert(Layer(tile_layer))
                .id();
        }
    }

    for wall_row in map.walls.iter() {
        let mut c = 0;
        for wall_exists in wall_row.columns.iter() {
            if *wall_exists {
                commands
                    .spawn()
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        transform: Transform {
                            // order rows from top down
                            translation: TilePosition(Vec2::new(
                                c as f32,
                                (15 - wall_row.row) as f32,
                            ))
                            .get_translation(
                                Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
                                tile_layer,
                            ),
                            ..Default::default()
                        },
                        sprite: TextureAtlasSprite::new(2),
                        ..Default::default()
                    })
                    .insert(Layer(tile_layer + 0.1))
                    .insert(Hitbox::new(Vec2::new(0.0, 0.0), Vec2::new(8.0, 8.0)))
                    .insert(Wall);
            }
            c += 1;
        }
    }

    let house_handle = asset_server.get_handle("sprites/house.png");
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(house_handle.into()),
            transform: Transform {
                translation: TilePosition(Vec2::new(map.house.tile_x, map.house.tile_y))
                    .get_translation(Vec2::new(16., 16.), 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Layer(1.0));

    let bus_stop_handle = asset_server.get_handle("sprites/bus_stop.png");
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(bus_stop_handle.into()),
            transform: Transform {
                translation: TilePosition(Vec2::new(map.bus_stop.tile_x, map.bus_stop.tile_y))
                    .get_translation(Vec2::new(16., 16.), 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Layer(1.0));
    state.set(AppState::InGame).unwrap();
}

fn unload_level(
    mut commands: Commands,
    sprite_query: Query<Entity, Or<(With<Sprite>, With<TextureAtlasSprite>)>>,
    mut levels: ResMut<Levels>,
    mut state: ResMut<State<AppState>>,
) {
    for entity in sprite_query.iter() {
        commands.entity(entity).despawn();
    }

    if levels.current_level < levels.levels.len() - 1 {
        levels.current_level += 1;
        state.set(AppState::Loading).unwrap();
    } else {
        state.set(AppState::Finished).unwrap();
    }
}

// fn save_map_to_file(map: &Map, path: &str) {
//     let file = File::create(format!("assets/levels/{}", path)).expect("Couldn't open file");
//     let pretty = PrettyConfig::new();
//     to_writer_pretty(file, &map, pretty).expect("Serialization failed");
// }

pub fn load_map(path: &str) -> Map {
    let file = File::open(format!("assets/levels/{}", path)).expect("Couldn't open map file");
    from_reader(file).expect("Could not parse ron map file")
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Levels>()
            .insert_resource(CurrentLevel::default())
            .add_system_set(
                SystemSet::on_enter(AppState::Loading)
                    .with_system(load_current_map.system().label("load_current_map"))
                    .with_system(load_map_atlas.system().after("load_current_map")),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::LevelDone).with_system(unload_level.system()),
            );
    }
}
