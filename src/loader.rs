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

    state.set(AppState::Loading).unwrap();
}

pub struct AssetsLoadingPlugin;
impl Plugin for AssetsLoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SpriteHandles>()
            .init_resource::<MapHandles>()
            .add_system_set(
                SystemSet::on_enter(AppState::AssetLoading)
                    .with_system(setup.system())
                    .before("check_assets"),
            )
            .add_system_set(
                SystemSet::on_update(AppState::AssetLoading)
                    .label("check_assets")
                    .with_system(track_assets_ready.system()),
            );
    }
}
