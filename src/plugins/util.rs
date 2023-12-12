use bevy::prelude::*;
use bevy_xpbd_3d::components::ExternalImpulse;

use crate::prelude::*;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_force_towards);
    }
}

fn apply_force_towards(
    mut query: Query<(Entity, &mut ExternalImpulse, &ForceTowards)>,
    global_transforms: Query<&GlobalTransform>,
) {
    query.for_each_mut(|(entity, mut velocity, towards)| {
        if let (Ok(transform), Ok(target_transform)) = (
            global_transforms.get(entity),
            global_transforms.get(towards.target),
        ) {
            let towards_transform = transform
                .compute_transform()
                .looking_at(target_transform.translation() + towards.offset, Vec3::Z);
            velocity.apply_impulse(towards_transform.forward() * towards.speed);
        }
    });
}
