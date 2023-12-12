use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_xpbd_3d::{
    components::CollisionLayers,
    plugins::spatial_query::{SpatialQuery, SpatialQueryFilter},
};
use leafwing_input_manager::prelude::*;

use crate::prelude::*;

use super::input::Action;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                create_and_move_focal_point.after(super::player::manage_player_influence),
                orbit_camera_input.after(create_and_move_focal_point),
                orbit_camera_movement.after(orbit_camera_input),
            )
                .run_if(in_state(GameState::Main)),
        );
    }
}

/// Move the camera focal point to the center of the influence sphere
fn create_and_move_focal_point(
    mut cmd: Commands,
    mut transforms: Query<&mut Transform>,
    focal_point_entity: Query<Entity, With<FocalPoint>>,
    influence_entity: Query<Entity, With<PlayerInfluenceRadius>>,
    time: Res<Time>,
) {
    // TODO: In the future, smooth this value
    if let Ok(influence_entity) = influence_entity.get_single() {
        match focal_point_entity.get_single() {
            Ok(focal_point_entity) => {
                let influence_transform = transforms.get(influence_entity).cloned();
                if let (Ok(mut focus_transform), Ok(influence_transform)) =
                    (transforms.get_mut(focal_point_entity), influence_transform)
                {
                    // TODO: this is a bad way to tween
                    focus_transform.translation = focus_transform.translation.lerp(
                        influence_transform.translation,
                        time.delta_seconds() * 10f32,
                    );
                }
            }
            Err(_) => {
                // create at influence location
                cmd.spawn((FocalPoint, *transforms.get(influence_entity).unwrap()));
            }
        }
    }
}

/// Apply input to the orbit camera
fn orbit_camera_input(
    mut camera: Query<(&mut OrbitCamera, &mut Projection)>,
    camera_settings: Res<settings::CameraSettings>,
    actions: Res<ActionState<Action>>,
    time: Res<Time>,
    mut mouse: EventReader<MouseMotion>,
) {
    let mouse_motion = mouse.read().fold(Vec2::ZERO, |acc, b| acc + b.delta) * 0.01;
    if let Ok((mut orbit_camera, mut projection)) = camera.get_single_mut() {
        if let Projection::Perspective(projection) = projection.as_mut() {
            if let Some(look) = actions.clamped_axis_pair(Action::Look) {
                orbit_camera.add_view_angle_normalized(
                    look.y() * camera_settings.sensitivity.y * time.delta_seconds(),
                );
                orbit_camera.add_view_angle_normalized(
                    mouse_motion.y.to_radians() * camera_settings.sensitivity.y,
                );

                orbit_camera.rotation -=
                    look.x() * camera_settings.sensitivity.x * time.delta_seconds();
                orbit_camera.rotation -=
                    mouse_motion.x.to_radians() * camera_settings.sensitivity.x;
            }
        }
    }
}

fn orbit_camera_movement(
    mut cmd: Commands,
    focus: Query<&Transform, With<FocalPoint>>,
    camera: Query<(Entity, &Transform, &OrbitCamera), With<Camera3d>>,
    camera_settings: Res<settings::CameraSettings>,
    spatial: SpatialQuery,
) {
    // move camera to correct position based on orbit camera

    if let (Ok(focus_transform), Ok((camera_entity, camera_transform, orbit_camera))) =
        (focus.get_single(), camera.get_single())
    {
        // We have the focus translation, so we need the desired camera
        // translation. Then, we will find the direction from the focus to the
        // camera, and cast a ray. If the ray intersects with anything before
        // the desired length, the camera will instead be placed there as to
        // avoid clipping into things
        let mut cast = *focus_transform;
        cast.rotation = Quat::from_axis_angle(Vec3::Z, orbit_camera.rotation);
        cast.rotate_local_y(
            orbit_camera
                .view_angle(camera_settings.angle_range.clone())
                .to_radians(),
        );

        let mut new_camera_position = cast.forward();
        // Cast a ray to the camera
        if let Some(hit) = spatial.cast_ray(
            cast.translation,
            cast.forward(),
            camera_settings.distance,
            true,
            SpatialQueryFilter::default().with_masks([Layer::Environment]),
        ) {
            // Ensure we are not in a solid object
            // TODO: This is not working :((((((
            if hit.time_of_impact != 0.0 {
                new_camera_position *= hit.time_of_impact;
                // take off a little distance to keep the camera from intersecting
                // the floor
                // TODO: shape cast and take the middle so that this is not necessary
                new_camera_position *= 0.80f32;
            } else {
                new_camera_position *= camera_settings.distance;
            }
        } else {
            new_camera_position *= camera_settings.distance;
        }

        let mut camera_transform = *focus_transform;
        camera_transform.translation += new_camera_position;
        camera_transform.look_at(cast.translation, Vec3::Z);

        // Send out a ray from the focal point to see if it intersects with any
        // walls

        // TODO: this is a bad way to tween
        cmd.entity(camera_entity).insert(camera_transform);
    }
}
