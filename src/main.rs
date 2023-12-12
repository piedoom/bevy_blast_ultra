use bevy::{prelude::*, window::WindowMode};
use bevy_blast_ultra::prelude::*;
use bevy_easings::EasingsPlugin;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_wasm_window_resize::WindowResizePlugin;
use bevy_xpbd_3d::prelude::*;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin::default())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                }),
            PhysicsPlugins::default(),
            EasingsPlugin,
            CorePlugins,
            EguiPlugin,
            PhysicsDebugPlugin::default(),
            WorldInspectorPlugin::new().run_if(|debug: Res<DebugMode>| *debug == DebugMode::On),
            WindowResizePlugin,
        ))
        .insert_resource(DebugMode::Off)
        .insert_resource(bevy_xpbd_3d::resources::Gravity(Vec3::NEG_Z * 50f32))
        .insert_resource(PhysicsDebugConfig {
            enabled: false,
            ..default()
        })
        .run();
}
