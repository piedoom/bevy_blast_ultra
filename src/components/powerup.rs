use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub enum Powerup {
    #[default]
    Spawner,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Indicator;
