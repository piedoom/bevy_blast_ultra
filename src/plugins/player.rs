use bevy::{audio::VolumeLevel, prelude::*};
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{prelude::*, resources::settings::PlayerSettings};

use super::input::Action;

const INFLUENCE_ACTIVE_PLAYER_SCALING: f32 = 0.3;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_grounded,
                add_extra_to_player,
                move_player,
                jump,
                manage_player_influence,
                magnetize,
                manage_active_players.after(manage_player_influence),
            )
                .run_if(in_state(GameState::Main)),
        );
    }
}

fn move_player(
    mut active_player_torques: Query<
        &mut ExternalTorque,
        (With<Player>, Or<(With<Active>, With<ScoredPlayer>)>),
    >,
    camera: Query<&Transform, With<Camera3d>>,
    focus_transform: Query<&Transform, With<FocalPoint>>,
    actions: Res<ActionState<Action>>,
    player_settings: Res<settings::PlayerSettings>,
) {
    if let (Ok(camera_transform), Ok(focus_transform)) =
        (camera.get_single(), focus_transform.get_single())
    {
        // Obtain the 2d forwards quat
        let forwards = {
            let camera_location = camera_transform.translation.truncate();
            let target_location = focus_transform.translation.truncate();

            let transform_a = Transform::from_translation(camera_location.extend(0f32));
            transform_a
                .looking_at(target_location.extend(0f32), Vec3::Z)
                .rotation
        };

        active_player_torques.for_each_mut(|mut torque| {
            if let Some(axes) = actions.axis_pair(Action::Move) {
                torque.set_torque(forwards.mul_vec3(
                    player_settings.torque_strength * Vec3::new(-axes.y(), 0f32, -axes.x()),
                ));
            } else {
                torque.set_torque(Vec3::ZERO);
            }
        });
    }
}

pub fn magnetize(
    mut query: Query<
        (&mut ExternalImpulse, &Transform),
        (With<Player>, With<Active>, Without<ScoredPlayer>),
    >,
    influence_transform: Query<&Transform, With<PlayerInfluenceRadius>>,
    time: Res<Time>,
    player_settings: Res<settings::PlayerSettings>,
) {
    if let Ok(influence_transform) = influence_transform.get_single() {
        query
            .par_iter_mut()
            .for_each(|(mut player_force, player_transform)| {
                let facing: Transform =
                    player_transform.looking_at(influence_transform.translation, Vec3::Z);
                let deadzone = player_settings.magnetism_deadzone;
                let distance: f32 = player_transform
                    .translation
                    .distance(influence_transform.translation);

                let distance_with_offset = if distance < deadzone {
                    0.0
                } else {
                    distance - deadzone
                };
                player_force.apply_impulse(
                    facing.forward()
                        * (distance_with_offset)
                        * player_settings.magnetism_strength
                        * time.delta_seconds(),
                );
            });
    }
}

/// Check if the player is on the ground. If it is, add a [`Grounded`] component.
fn check_grounded(
    mut cmd: Commands,
    mut players: Query<(Entity, &Transform, &Collider), (With<Player>, With<Active>)>,
    gravity: Res<Gravity>,
    spatial_query: SpatialQuery,
) {
    let gravity_direction = gravity.0.normalize_or_zero();

    players
        .iter_mut()
        .for_each(|(entity, transform, collider)| {
            // Check to see that the entity is grounded
            if spatial_query
                .cast_shape(
                    collider,              // Shape
                    transform.translation, // Origin
                    Quat::default(),       // Shape rotation
                    gravity_direction,     // Direction
                    0.1,                   // Maximum time of impact (travel distance)
                    true,                  // Should initial penetration at the origin be ignored
                    SpatialQueryFilter::new()
                        .with_masks_from_bits(
                            CollisionLayers::default()
                                .add_groups([Layer::Environment])
                                .masks_bits(),
                        )
                        .without_entities([entity]), // Query filter
                )
                .is_some()
            {
                cmd.entity(entity).insert(Grounded);
            } else {
                cmd.entity(entity).remove::<Grounded>();
            }
        });
}

/// Check for jumping, and if on a surface, apply an impulse against gravity
fn jump(
    mut players: Query<&mut ExternalImpulse, (With<Player>, With<Active>, With<Grounded>)>,
    player_settings: Res<settings::PlayerSettings>,
    actions: Res<ActionState<Action>>,
    gravity: Res<Gravity>,
) {
    if actions.just_pressed(Action::Jump) {
        let gravity_direction = gravity.0.normalize_or_zero();
        let impulse = (gravity_direction * -1f32) * player_settings.jump_strength;

        players.iter_mut().for_each(|mut external_impulse| {
            external_impulse.apply_impulse(impulse);
        });
    }
}

/// Move the player influence to the average of the active players
pub(crate) fn manage_player_influence(
    mut transforms: Query<&mut Transform>,
    influence_entity: Query<Entity, (With<Transform>, With<PlayerInfluenceRadius>)>,
    active_players: Query<
        Entity,
        (
            With<Transform>,
            With<Active>,
            With<Player>,
            Without<ScoredPlayer>,
        ),
    >,
) {
    let active_player_count = active_players.iter().count();

    if active_player_count > 0 {
        if let Ok(influence_entity) = influence_entity.get_single() {
            // First, find the average position of the players that were active since
            // last step
            let active_player_translations_sum =
                active_players.iter().fold(Vec3::ZERO, |acc, entity| {
                    acc + transforms
                        .get(entity)
                        .expect("should have transform on every player")
                        .translation
                });

            let average = active_player_translations_sum / Vec3::splat(active_player_count as f32);

            let mut influence_transform = transforms
                .get_mut(influence_entity)
                .expect("should have transform on every influence");

            influence_transform.translation = average;
        }
    }
}

/// Add and remove active players depending on the influence. This should be run
/// after [`calculate_player_influence`].
fn manage_active_players(
    mut cmd: Commands,
    players: Query<(Entity, &Transform), (With<Player>, Without<ScoredPlayer>)>,
    active_players: Query<Entity, (With<Player>, With<Active>)>,
    influence: Query<(&Transform, &PlayerInfluenceRadius)>,
) {
    // Compare with the squared value as to not need sqrt
    if let Ok((influence_transform, influence)) = influence.get_single() {
        // increase influence by some factor by counting active players
        let active_player_count = active_players.iter().len();
        let influence =
            influence.0 + (INFLUENCE_ACTIVE_PLAYER_SCALING * (active_player_count + 1) as f32);

        let squared_max_distance = influence.powi(2);

        players.for_each(|(player_entity, player_transform)| {
            // Disable players that are outside the max radius
            let active = influence_transform
                .translation
                .distance_squared(player_transform.translation)
                <= squared_max_distance;

            match active {
                true => {
                    cmd.entity(player_entity).insert(Active);
                }
                false => {
                    cmd.entity(player_entity).remove::<Active>();
                }
            }
        });
    }
}

//

fn add_extra_to_player(
    mut cmd: Commands,
    query: Query<Entity, Added<Player>>,
    player_settings: Res<PlayerSettings>,
    game_assets: Res<GameAssets>,
) {
    query.for_each(|e| {
        let collider = Collider::ball(0.5);

        cmd.entity(e).insert((
            collider.clone(),
            RigidBody::Dynamic,
            ExternalTorque::default().with_persistence(false),
            Friction::new(player_settings.friction),
            CollisionLayers::new(
                [Layer::Player],
                [Layer::Player, Layer::Environment, Layer::Sensor],
            ),
            ColliderDensity(player_settings.density),
            AudioBundle {
                source: game_assets.sfx.get("audio/sfx/roll.ogg").unwrap().clone(),
                settings: PlaybackSettings::LOOP
                    .with_spatial(true)
                    .with_volume(bevy::audio::Volume::Relative(VolumeLevel::new(0f32))),
            },
        ));
    });
}
