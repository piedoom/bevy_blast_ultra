use std::ops::Mul;

use bevy::prelude::*;
use bevy_gltf_blueprints::{BluePrintBundle, BlueprintName};
use bevy_xpbd_3d::prelude::*;

use crate::prelude::*;

// Radius where the powerup will count as being activated.
// Note that very large values may result in being able to activate from below
const PICKUP_RADIUS: f32 = 2f32;

pub struct PowerupsPlugin;

impl Plugin for PowerupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (setup_powerups, manage_powerups).run_if(in_state(GameState::Main)),
        );
    }
}

/// Add necessary components to powerups so that they function
fn setup_powerups(mut cmd: Commands, query: Query<Entity, Added<Powerup>>) {
    // add collider
    query.for_each(|entity| {
        cmd.entity(entity).insert((
            Sensor,
            Active,
            Collider::ball(PICKUP_RADIUS),
            CollisionLayers::new([Layer::Sensor], [Layer::Player]),
        ));
    });
}

fn manage_powerups(
    mut cmd: Commands,
    active_players: Query<
        (Entity, &LinearVelocity, &AngularVelocity),
        (With<Player>, With<Active>),
    >,
    active_powerups: Query<(Entity, &Powerup, &Transform, &CollidingEntities), With<Active>>,
    children: Query<&Children>,
    indicators: Query<&Indicator>,
) {
    const SPAWN_DISTANCE: f32 = 2f32;
    // let active_player_entities = active_players.iter().map(|x| x.0).collect();
    active_powerups.for_each(|(powerup_entity, powerup, powerup_transform, collisions)| {
        let maybe_colliding_player = collisions.iter().find(|x| active_players.contains(**x));
        if let Some(colliding_player) = maybe_colliding_player {
            match powerup {
                Powerup::Spawner => {
                    // Get some components from the colliding player
                    let (_, lin_vel, ang_vel) = active_players.get(*colliding_player).unwrap();
                    // Spawn new player entity
                    cmd.spawn(BluePrintBundle {
                        // TODO v issue below
                        blueprint: BlueprintName("player".to_string()),
                        ..Default::default()
                    })
                    .insert((
                        Active,
                        Name::new("player.spawn"),
                        Transform::from_translation(
                            powerup_transform.translation
                                + (Vec3::Z.mul(SPAWN_DISTANCE)
                                    + ((lin_vel.0.normalize_or_zero() * -1f32) * SPAWN_DISTANCE)),
                        ),
                        *lin_vel,
                        *ang_vel,
                    ));
                }
            }

            // Deactivate the powerup
            cmd.entity(powerup_entity).remove::<Active>();

            // Find [`Indicator`] marker components to mark as invisible and
            // show the powerup is inactive
            children
                .get(powerup_entity)
                .unwrap()
                .iter()
                .for_each(|child| {
                    if indicators.contains(*child) {
                        cmd.entity(*child).despawn_recursive();
                    }
                });
        }
    });
}
