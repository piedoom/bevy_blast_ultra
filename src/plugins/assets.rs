use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

/// Loads all assets
pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<SettingsAsset>::new(&["settings.ron"]))
            // Continue to the main game state once everything is loaded in, so
            // we can be sure all assets are loaded first
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
            )
            // This file specifies our collection of assets as strings, so that
            // assets can be modified or moved withou the need of recompilation
            .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
                GameState::Loading,
                "assets_game.assets.ron",
            )
            // Load all GameAssets exported from Blender
            .add_collection_to_loading_state::<_, GameAssets>(GameState::Loading);
    }
}
