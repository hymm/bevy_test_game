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
    mut map_handles: ResMut<MapHandles>,
    mut sfx_handles: ResMut<SfxHandles>,
) {
    sprite_handles.handles = asset_server.load_folder("./sprites").unwrap();
    map_handles.handles = asset_server.load_folder("./levels").unwrap();
    sfx_handles.handles = asset_server.load_folder("./sfx").unwrap();
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
    fn build(&self, app: &mut AppBuilder) {
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
