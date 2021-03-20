use crate::consts::{AppState, APP_STATE_STAGE, TILE_SIZE};
use crate::coordinates::TilePosition;
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

fn load_map_atlas(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut state: ResMut<State<AppState>>,
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
                    translation: TilePosition(Vec2::new(c as f32, r as f32))
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
        app.on_state_enter(APP_STATE_STAGE, AppState::Loading, load_map_atlas.system());
    }
}
