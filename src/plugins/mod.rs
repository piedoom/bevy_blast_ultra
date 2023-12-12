use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_gltf_blueprints::*;
use bevy_xpbd_3d::prelude::*;

use crate::prelude::*;

mod assets;
mod audio;
mod camera;
mod gameplay;
mod input;
mod levels;
mod player;
mod powerups;
mod settings;
mod ui;
mod util;

pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InitPlugin)
            .add(BlueprintsPlugin {
                library: BlueprintsLibrary::Files(vec![
                    "levels/library/goal.gltf".into(),
                    "levels/library/player.gltf".into(),
                    "levels/library/powerup_spawner.gltf".into(),
                ]),
                format: GltfFormat::GLTF,
                ..Default::default()
            })
            .add(assets::AssetsPlugin)
            .add(levels::LevelsPlugin)
            .add(input::InputPlugin)
            .add(player::PlayerPlugin)
            .add(camera::CameraPlugin)
            .add(gameplay::GameplayPlugin)
            .add(util::UtilPlugin)
            .add(powerups::PowerupsPlugin)
            .add(ui::UiPlugin)
            .add(settings::SettingsPlugin)
            .add(audio::AudioPlugin)
    }
}

/// Do initial stuff to the game and world
struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>() // Register all types intended to be used either in Blender or to be saved
            .register_type::<RigidBody>()
            .register_type::<Collider>()
            .register_type::<AutoCollider>()
            .register_type::<Player>()
            .register_type::<PlayerInfluenceRadius>()
            .register_type::<OrbitCamera>()
            .register_type::<Sensor>()
            .register_type::<Powerup>()
            .register_type::<Active>()
            .register_type::<GoalArea>()
            .register_type::<LinearVelocity>()
            .register_type::<AngularVelocity>()
            .register_type::<Spawn>()
            .register_type::<Camera>()
            .register_type::<FloorBounds>()
            .register_type::<GameManager>()
            .register_type::<Indicator>()
            .register_type::<Cleanup>();
    }
}

/// Marker component for Blender compatibility. Replaced via [`physics_replace_proxies`].
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub enum Collider {
    Ball(f32),
    Cuboid(Vec3),
    Capsule(Vec3, Vec3, f32),
    #[default]
    Mesh,
}

/// Marker component for Blender compatibility. Replaced via [`physics_replace_proxies`].
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub enum AutoCollider {
    #[default]
    Cuboid,
    Ball,
    Capsule,
}
