use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

use super::settings;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(key = "levels", collection, collection(typed))]
    pub levels: Vec<Handle<Gltf>>,

    #[asset(key = "menu")]
    pub menu: Handle<Gltf>,

    #[asset(key = "library", collection, collection(typed, mapped))]
    pub library: HashMap<String, Handle<Gltf>>,

    #[asset(key = "music", collection, collection(typed, mapped))]
    pub music: HashMap<String, Handle<AudioSource>>,

    #[asset(key = "sfx", collection, collection(typed, mapped))]
    pub sfx: HashMap<String, Handle<AudioSource>>,

    #[asset(key = "images", collection, collection(typed, mapped))]
    pub images: HashMap<String, Handle<Image>>,

    #[asset(key = "environment_diffuse")]
    pub environment_diffuse: Handle<Image>,
    #[asset(key = "environment_specular")]
    pub environment_specular: Handle<Image>,
    #[asset(key = "settings")]
    pub settings: Handle<SettingsAsset>,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath, Clone)]
pub struct SettingsAsset {
    pub ui: settings::UiSettings,
    pub game: settings::GameSettings,
    pub audio: settings::AudioSettings,
}
