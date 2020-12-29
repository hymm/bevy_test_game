use bevy::prelude::*;
use bevy::{asset::LoadState, sprite::TextureAtlasBuilder};

fn main() {
    App::build()
        .init_resource::<SpriteHandles>()
        .add_plugins(DefaultPlugins)
        .add_resource(State::new(AppState::Setup))
        .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
        .on_state_enter(STAGE, AppState::Setup, load_textures.system())
        .on_state_update(STAGE, AppState::Setup, check_textures.system())
        .on_state_enter(STAGE, AppState::Finished, draw_map.system())
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
                scale: Vec3::splat(4.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(vendor_index as u32),
            texture_atlas: atlas_handle,
            ..Default::default()
        });
}

struct Car {
    x: i16,
    y: i16,
    speed: f32,
    hitbox_width: i16,
}

struct MapRow {
    sprite: u32,
}

struct Map {
    rows: [MapRow; 16],
    // cars: Vec<Car>,
}

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
    /*cars: vec![
        Car {
            x: 0,
            y: 0,
            speed: 0.5,
            hitbox_width: 13,
        }
    ]*/
};

fn draw_map(
    commands: &mut Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // load sprite sheet as texture atlas
    
    let texture_handle = asset_server.get_handle("map_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 4, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // let vendor_index = texture_atlas.get_texture_index(&texture_handle).unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(64.0 * 4.0, 64.0 * 4.0, 1000.0 - 0.1),
            rotation: Quat::default(),
            scale: Vec3::new(1.0, -1.0, 1.0)
        },
        ..Camera2dBundle::default() }
    );
    for r in 0..map.rows.len() {
        for c in 0..16   {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new((c as f32) * 32.0, (r as f32) * 32.0, 0.0),
                    scale: Vec3::splat(4.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(map.rows[r].sprite),
                ..Default::default()
            });
        }
    }
}