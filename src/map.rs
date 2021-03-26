use crate::consts::{AppState, APP_STATE_STAGE, TILE_SIZE};
use crate::coordinates::TilePosition;
use bevy::prelude::*;

pub struct MapRow {
    sprite: u32,
}

pub struct House {
    pub tile_x: f32,
    pub tile_y: f32,
}

pub struct BusStop {
    pub tile_x: f32,
    pub tile_y: f32,
}

pub struct CarData {
    pub tile_position: TilePosition,
    pub speed: f32,
}

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

fn load_map_atlas(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut state: ResMut<State<AppState>>,
    map: Res<Map>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.get_handle("map_tiles.png");
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
                    translation: TilePosition(Vec2::new(c as f32, (15 - r) as f32))
                        .get_translation(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                    ..Default::default()
                },
                sprite: spr,
                ..Default::default()
            });
        }
    }

    let house_handle = asset_server.get_handle("house.png");
    commands.spawn(SpriteBundle {
        material: materials.add(house_handle.into()),
        transform: Transform {
            translation: TilePosition(Vec2::new(map.house.tile_x, map.house.tile_y))
                .get_translation(Vec2::new(16., 16.)),
            ..Default::default()
        },
        ..Default::default()
    });

    let bus_stop_handle = asset_server.get_handle("bus_stop.png");
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

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Map>().on_state_enter(
            APP_STATE_STAGE,
            AppState::Loading,
            load_map_atlas.system(),
        );
    }
}
