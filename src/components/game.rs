use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct GoalArea;

/// Game logic data
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct GameManager {
    /// Total goal amount for win state
    pub goal: usize,
    /// Current amount in the goal
    pub current: usize,
}

impl GameManager {
    pub fn normalized_score(&self) -> f32 {
        self.current as f32 / self.goal as f32
    }
}

/// If a player falls below the Z height of this component, it will be marked as
/// out of bounds
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct FloorBounds;
