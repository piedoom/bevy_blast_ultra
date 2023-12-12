use crate::prelude::*;
use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        // Add before any systems that use settings have a chance to run.
        // In the future, we might want to separate this out into another stage
        // of loading.
        app.add_systems(OnExit(GameState::Loading), init);
    }
}

fn init(mut cmd: Commands, game_assets: Res<GameAssets>, settings: Res<Assets<SettingsAsset>>) {
    // Add various settings as resources so we don't need to constantly grab settings
    let settings = settings.get(game_assets.settings.clone()).unwrap();

    cmd.insert_resource(settings.game.camera.clone());
    cmd.insert_resource(settings.ui.font.clone());
    cmd.insert_resource(settings.game.player.clone());
    cmd.insert_resource(settings.ui.scene.clone());
    cmd.insert_resource(settings.audio.clone());
}
