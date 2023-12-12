use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseMotion},
    prelude::*,
};
use leafwing_input_manager::{
    axislike::MouseMotionAxisType,
    prelude::*,
    user_input::{InputKind, RawInputs},
};

use crate::prelude::GameState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActionState<Action>>()
            .insert_resource(
                InputMap::default()
                    .insert(DualAxis::left_stick(), Action::Move)
                    .insert(VirtualDPad::wasd(), Action::Move)
                    .insert(DualAxis::right_stick(), Action::Look)
                    .insert(KeyCode::Space, Action::Jump)
                    .insert(GamepadButtonType::LeftTrigger, Action::Jump)
                    .insert(KeyCode::Escape, Action::TogglePause)
                    .insert(GamepadButtonType::Start, Action::TogglePause)
                    .insert(KeyCode::Tab, Action::ToggleDebug)
                    .insert(GamepadButtonType::Select, Action::ToggleDebug)
                    // .insert(GamepadButtonType::South, Action::MenuSelect)
                    .build(),
            )
            .add_plugins(InputManagerPlugin::<Action>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Move,
    Look,
    Jump,
    TogglePause,
    ToggleDebug,
    // MenuMove,
    // MenuSelect,
}
