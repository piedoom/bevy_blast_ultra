//! Level lifecycle management
//! Loads assets for the game. Much of this code is taken from the examples
//! from the Blender_bevy_components_workflow project, which is MIT licensed,
//! as shown below.
//!
//! MIT License
//!
//! Copyright (c) 2023 Mark "kaosat-dev" Moissette
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

use crate::prelude::*;
use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::{DebandDither, Tonemapping},
    },
    pbr::*,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};
use bevy_gltf_blueprints::*;
use bevy_xpbd_3d::{
    components::{CollisionLayers, Friction},
    prelude::Collider as XpbdCollider,
};

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::level_transition()),
            (cleanup, spawn.after(cleanup)),
        )
        // Hack until we can just handle that with the level transition
        .add_systems(OnEnter(GameState::Menu), cleanup)
        .add_systems(
            Update,
            (
                process_map.run_if(in_state(GameState::level_transition())),
                process_colliders,
                process_lights,
            ),
        );
    }
}

fn cleanup(
    mut cmd: Commands,
    time: Res<Time>,
    existing_game_world: Query<Entity, With<GameWorldTag>>,
    cleanup_entities: Query<Entity, With<Cleanup>>,
) {
    // Clean up previous map
    if let Ok(existing_game_world) = existing_game_world.get_single() {
        cmd.entity(existing_game_world).despawn_recursive();
    }

    cleanup_entities.for_each(|e| cmd.entity(e).despawn_recursive());

    // Insert a timer
    cmd.insert_resource(GameTime::from(time.elapsed()));
}

fn spawn(
    mut cmd: Commands,
    gltf: Res<Assets<bevy::gltf::Gltf>>,
    state: Res<State<GameState>>,
    assets: Res<GameAssets>,
    camera_settings: Res<settings::CameraSettings>,
    mut images: ResMut<Assets<Image>>,
) {
    // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
    // so they appear as one texture. The following code reconfigures the texture as necessary.
    let convert = |image: &mut Image| {
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }
    };

    {
        convert(images.get_mut(assets.environment_diffuse.clone()).unwrap());
    }
    {
        convert(images.get_mut(assets.environment_specular.clone()).unwrap());
    }

    // Add a new camera
    cmd.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: camera_settings.fov.to_radians(),
                ..default()
            }),
            tonemapping: Tonemapping::BlenderFilmic,
            dither: DebandDither::Enabled,
            ..default()
        },
        OrbitCamera::default(),
        SpatialListener::new(0.1),
        Cleanup,
        EnvironmentMapLight {
            diffuse_map: assets.environment_diffuse.clone(),
            specular_map: assets.environment_specular.clone(),
        },
        bevy::core_pipeline::Skybox(assets.environment_specular.clone()),
        BloomSettings {
            intensity: 0.01,
            composite_mode: BloomCompositeMode::Additive,
            ..default()
        },
    ));

    // Get the map to spawn
    match state.get() {
        GameState::LevelTransition { level } => {
            // Spawn the new world. This seems to not like entities being added before
            // it fully spawns, so we do all that in `process_map`
            cmd.spawn((
                SceneBundle {
                    scene: gltf.get(level).expect("level to load").scenes[0].clone(),
                    ..default()
                },
                GameWorldTag,
                // // LoadedMarker,
                Name::new("level_1"),
            ));
        }
        _ => unreachable!(),
    }
}

/// All stuff that spawn the actual map. Can spawn additional entities that do
/// not rely on any references to the level or environment.
fn process_map(
    mut cmd: Commands,
    player_settings: Res<settings::PlayerSettings>,
    // Determines whether this system adds any stuff to the map. If not, we transition
    spawn_query: Query<&Transform, Added<Spawn>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(transform) = spawn_query.get_single() {
        //  Add necessary game entities at the [`Spawn`] marker component
        //  position
        cmd.spawn((
            PlayerInfluenceRadius(player_settings.influence_radius),
            *transform,
            Cleanup,
        ));

        // Spawn in the player entity
        // Spawn new entity
        cmd.spawn(BluePrintBundle {
            blueprint: BlueprintName("player".to_string()),
            transform: TransformBundle::from_transform(*transform),
            ..Default::default()
        })
        .insert((Active, Name::new("player"), Cleanup));

        cmd.spawn((FocalPoint, *transform, Cleanup));

        // Transition the state
        next_state.set(GameState::Main);
    }
}

fn process_lights(
    mut cmd: Commands,
    mut added_lights: Query<(Entity, &mut DirectionalLight), Added<DirectionalLight>>,
    mut added_spotlights: Query<&mut SpotLight, Added<SpotLight>>,
) {
    // for light in added_lights.iter() {
    //     println!("light");
    // }
    // Replace lights with proper settings
    for (entity, mut light) in added_lights.iter_mut() {
        light.illuminance *= 15.0;
        light.shadows_enabled = true;
        let shadow_config: CascadeShadowConfig = CascadeShadowConfigBuilder {
            first_cascade_far_bound: 15.0,
            maximum_distance: 256.0,
            ..default()
        }
        .into();
        cmd.entity(entity).insert(shadow_config);
        // TEST/
        // commands.entity(entity).despawn();
    }
    for mut light in added_spotlights.iter_mut() {
        light.shadows_enabled = true;
    }
}

fn process_colliders(
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
    // needed for tri meshes
    children: Query<&Children>,
    mut cmd: Commands,
    mut proxy_colliders: Query<
        (Entity, &Collider, &Name, &mut Visibility),
        (Without<XpbdCollider>, Added<Collider>, Without<Player>),
    >,
) {
    // Replace physics entities
    for proxy_colider in proxy_colliders.iter_mut() {
        let (entity, collider_proxy, name, mut visibility) = proxy_colider;
        // we hide the collider meshes: perhaps they should be removed altogether once processed ?
        if name.ends_with("_collider") || name.ends_with("_sensor") {
            *visibility = Visibility::Hidden;
        }

        let friction = Friction::new(0.5f32);
        let collision_layers = CollisionLayers::all::<Layer>(); // CollisionLayers::new([Layer::Environment], [Layer::Player]);

        let mut xpbd_collider: XpbdCollider;
        match collider_proxy {
            Collider::Ball(radius) => {
                xpbd_collider = XpbdCollider::ball(*radius);
                cmd.entity(entity)
                .insert((xpbd_collider,  collision_layers, friction))
                //.insert(ActiveEvents::COLLISION_EVENTS)  // FIXME: this is just for demo purposes (also is there something like that in xpbd ?) !!!
                ;
            }
            Collider::Cuboid(size) => {
                xpbd_collider = XpbdCollider::cuboid(size.x, size.y, size.z);
                cmd.entity(entity)
            .insert((xpbd_collider, collision_layers, friction))
                //.insert(ActiveEvents::COLLISION_EVENTS)  // FIXME: this is just for demo purposes (also is there something like that in xpbd ?) !!!
                ;
            }
            Collider::Capsule(a, b, radius) => {
                // FIXME: temp
                let height = Vec3::distance(*a, *b);
                xpbd_collider = XpbdCollider::capsule(height, *radius);
                info!("CAPSULE {} {}", height, radius);
                cmd.entity(entity)
                .insert((xpbd_collider, collision_layers, friction))
                //.insert(ActiveEvents::COLLISION_EVENTS)  // FIXME: this is just for demo purposes (also is there something like that in xpbd ?) !!!
                ;
            }
            Collider::Mesh => {
                for (_, collider_mesh) in
                    Mesh::search_in_children(entity, &children, &meshes, &mesh_handles)
                {
                    xpbd_collider = XpbdCollider::trimesh_from_mesh(collider_mesh).unwrap(); // convex_hull_from_mesh?
                    cmd.entity(entity)
                        .insert((xpbd_collider, collision_layers, friction));
                }
            }
        }
    }
}
