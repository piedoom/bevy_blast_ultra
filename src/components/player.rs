use bevy::prelude::*;

/// Used in our scene files to specify the player entry point, and then add
/// additional components for game setup
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Spawn;

/// Marks an entity as player-controlled. Some other components must be added to
/// the player post-spawn in order for it to work correctly.
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Player;

/// Marks the entity as already scored
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct ScoredPlayer;

/// Averages the locations of active player marbles. This is used as a focal
/// point target, and to calculate which way to push marbles, as we need to
/// obtain that information from the camera. The `PlayerInfluence` also contains
/// an active radius. Any marbles not within that radius will be marked
/// inactive, and not controllable until in radius once again.
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct PlayerInfluenceRadius(pub f32);

/// The focal point of the camera. This attempts to follow the center of the
/// entity containing a [`PlayerInfluence`] component.
#[derive(Component, Debug)]
pub struct FocalPoint;

#[derive(Component, Debug)]
pub struct Grounded;
