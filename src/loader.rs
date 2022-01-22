use crate::consts::AppState;
use bevy::asset::LoadState;
use bevy::prelude::*;

#[derive(Default, Clone)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Default)]
struct MapHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Default)]
struct SfxHandles {
    handles: Vec<HandleUntyped>,
}

fn setup_loader(
    asset_server: Res<AssetServer>,
    mut sprite_handles: ResMut<SpriteHandles>,
    mut sfx_handles: ResMut<SfxHandles>,
) {
    sprite_handles.handles = vec![
        "sprites/bus_stop.png",
        "sprites/house.png",
        "sprites/map_tiles.png",
        "sprites/shoe_animation",
        "sprites/suv.png",
        "sprites/victory_screen.png",
    ]
    .iter()
    .map(|filename| asset_server.load_untyped(*filename))
    .collect();
    
    sfx_handles.handles = vec!["sfx/honk.mp3", "sfx/step.mp3"]
        .iter()
        .map(|filename| asset_server.load_untyped(*filename))
        .collect();
}

fn track_assets_ready(
    mut state: ResMut<State<AppState>>,
    sprite_handles: Res<SpriteHandles>,
    map_handles: Res<MapHandles>,
    sfx_handles: Res<SfxHandles>,
    asset_server: Res<AssetServer>,
) {
    let handles: Vec<HandleUntyped> = sprite_handles
        .handles
        .iter()
        .cloned()
        .chain(map_handles.handles.iter().cloned())
        .chain(sfx_handles.handles.iter().cloned())
        .collect();

    if LoadState::Loaded
        != asset_server.get_group_load_state(handles.iter().map(|handle| handle.id))
    {
        return;
    }

    state.set(AppState::Loading).unwrap();
}

pub struct AssetsLoadingPlugin;
impl Plugin for AssetsLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteHandles>()
            .init_resource::<MapHandles>()
            .init_resource::<SfxHandles>()
            .add_system_set(
                SystemSet::on_enter(AppState::AssetLoading)
                    .with_system(setup_loader.system())
                    .before("check_assets"),
            )
            .add_system_set(
                SystemSet::on_update(AppState::AssetLoading)
                    .label("check_assets")
                    .with_system(track_assets_ready.system()),
            );
    }
}
