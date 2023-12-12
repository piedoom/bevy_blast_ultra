use std::ops::RangeInclusive;

use bevy::{ecs::system::Resource, math::Vec2};

#[derive(serde::Deserialize, Clone, Resource)]
pub struct GameSettings {
    pub player: PlayerSettings,
    pub camera: CameraSettings,
}

#[derive(serde::Deserialize, Clone, Resource)]
pub struct PlayerSettings {
    pub torque_strength: f32,
    pub jump_strength: f32,
    pub friction: f32,
    pub density: f32,
    pub magnetism_deadzone: f32,
    pub magnetism_strength: f32,
    pub influence_radius: f32,
}

#[derive(serde::Deserialize, Clone, Resource)]
pub struct CameraSettings {
    /// Permitted camera angles in degrees
    pub angle_range: RangeInclusive<f32>,
    pub distance: f32,
    pub sensitivity: Vec2,
    pub fov: f32,
    pub fov_multiplier_range: RangeInclusive<f32>,
}

impl CameraSettings {}

#[derive(serde::Deserialize, Clone, Resource)]
pub struct UiSettings {
    pub font: FontSettings,
    pub scene: UiSceneSettings,
}

#[derive(serde::Deserialize, Clone, Resource)]
pub struct UiSceneSettings {
    pub rotation_speed: f32,
}

#[derive(serde::Deserialize, Clone, Resource)]
pub struct FontSettings {
    /// The literal size that will be scaled from. Consider it like
    /// `rem` in CSS
    pub base: f32,
    title_scale: f32,
    subtitle_scale: f32,
    body_scale: f32,
}

impl FontSettings {
    #[inline(always)]
    pub fn size_title(&self) -> f32 {
        self.base * self.title_scale
    }
    #[inline(always)]
    pub fn size_subtitle(&self) -> f32 {
        self.base * self.subtitle_scale
    }
    #[inline(always)]
    pub fn size_body(&self) -> f32 {
        self.base * self.body_scale
    }
}

#[derive(serde::Deserialize, Clone, Resource)]
pub struct AudioSettings {
    pub spatial_scale: f32,
}
