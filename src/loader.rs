use crate::consts::{AppState, APP_STATE_STAGE};
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

fn setup(
    asset_server: Res<AssetServer>,
    mut sprite_handles: ResMut<SpriteHandles>,
    mut map_handles: ResMut<MapHandles>,
) {
    sprite_handles.handles = asset_server.load_folder("./sprites").unwrap();
    map_handles.handles = asset_server.load_folder("./levels").unwrap();
}

fn track_assets_ready(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    map_handles: ResMut<MapHandles>,
    asset_server: Res<AssetServer>,
) {
    if LoadState::Loaded
        != asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        return;
    }

    if LoadState::Loaded
        != asset_server.get_group_load_state(map_handles.handles.iter().map(|handle| handle.id))
    {
        return;
    }

    state.set_next(AppState::Loading).unwrap();
}

pub struct AssetsLoadingPlugin;
impl Plugin for AssetsLoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SpriteHandles>()
            .init_resource::<MapHandles>()
            .on_state_enter(APP_STATE_STAGE, AppState::AssetLoading, setup.system())
            .on_state_update(
                APP_STATE_STAGE,
                AppState::AssetLoading,
                track_assets_ready.system(),
            );
    }
}
