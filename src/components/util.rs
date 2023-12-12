use bevy::prelude::*;

/// Apply forces to attempt to reach a target vector
#[derive(Component, Debug)]
pub struct ForceTowards {
    pub speed: f32,
    pub target: Entity,
    pub offset: Vec3,
}

/// Generic marker component for things that have an active and inactive state
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Active;

/// Marks something as being out of bounds in some way and ready for cleanup
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct OutOfBounds;

/// All entities with this component will be despawned when cleaning up levels
/// for transition. Add this to entities that will be spawned in again outside
/// of the map scene, especially if they are singletons like
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Cleanup;
