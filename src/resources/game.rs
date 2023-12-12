use bevy::prelude::*;

#[derive(Resource, Clone, Default, Copy)]
pub struct CurrentLevelIndex(pub usize);

#[derive(Resource, Clone, Copy, PartialEq, Eq)]
pub enum DebugMode {
    On,
    Off,
}
