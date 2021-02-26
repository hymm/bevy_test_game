use crate::consts::{AppState, APP_STATE_STAGE};
use bevy::asset::LoadState;
use bevy::prelude::*;

#[derive(Default, Clone)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

fn setup(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = asset_server.load_folder(".").unwrap();
}

fn track_assets_ready(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set_next(AppState::Loading).unwrap();
    }
}

pub struct AssetsLoadingPlugin;
impl Plugin for AssetsLoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SpriteHandles>()
            .on_state_enter(APP_STATE_STAGE, AppState::AssetLoading, setup.system())
            .on_state_update(
                APP_STATE_STAGE,
                AppState::AssetLoading,
                track_assets_ready.system(),
            );
    }
}
