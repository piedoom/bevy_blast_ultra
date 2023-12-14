use bevy::{gltf::Gltf, prelude::*};

#[derive(Default, States, Debug, Clone, Eq)]
pub enum GameState {
    // #[default]
    // Preloading,
    #[default]
    Loading,
    Menu,
    Main,
    Pause,
    Post {
        win: bool,
    },
    // Allows us to consolidate level spawn and cleanup code.
    // If no level specified, go to the main menu
    LevelTransition {
        level: Handle<Gltf>,
    },
}

// Discard data so we can use the gamestate to hold relevant information
impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl std::hash::Hash for GameState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl GameState {
    #[inline(always)]
    pub fn level_transition() -> Self {
        Self::LevelTransition {
            level: Default::default(),
        }
    }
    #[inline(always)]
    pub fn post() -> Self {
        Self::Post { win: false }
    }
}
