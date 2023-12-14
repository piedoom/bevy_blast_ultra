use std::ops::RangeInclusive;

use bevy::prelude::*;

use crate::prelude::*;

/// Orbiting camera used for the main game view of entities.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct OrbitCamera {
    pub rotation: f32,
    /// Normalized (0,1) value specifying interpolation between values set in zoom range
    view_angle_normalized: f32,
    // pub zoom_range: RangeInclusive<f32>,
    // pub zoom_height_range: RangeInclusive<f32>,
}

impl OrbitCamera {
    pub fn view_angle(&self, range: RangeInclusive<f32>) -> f32 {
        range.slerp(self.view_angle_normalized)
    }
    pub fn add_view_angle_normalized(&mut self, angle: f32) {
        self.view_angle_normalized = (self.view_angle_normalized + angle).min(1.0).max(0.0);
    }
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            rotation: 180f32.to_radians(),
            view_angle_normalized: 0.5f32,
            // zoom_range: 8f32..=15f32,
            // zoom_height_range: 0.2f32..=8f32,
        }
    }
}
