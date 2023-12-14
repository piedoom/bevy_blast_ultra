use bevy::prelude::*;
use bevy_egui::{
    egui::{self, epaint::Shadow, Color32, FontId, Frame, Stroke},
    EguiContexts, EguiSettings,
};
use bevy_egui_kbgp::prelude::*;
use bevy_gltf_blueprints::GameWorldTag;
use bevy_xpbd_3d::plugins::debug::PhysicsDebugConfig;
use leafwing_input_manager::action_state::ActionState;

use crate::prelude::*;

use super::input::Action;

pub struct UiPlugin;

#[derive(Clone)]
pub enum UiActions {
    ToggleMenu,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_debug)
            .add_systems(OnEnter(GameState::Menu), setup)
            .add_systems(
                Update,
                (menu_ui, move_menu_camera).run_if(in_state(GameState::Menu)),
            )
            .add_systems(Update, (game_ui,).run_if(in_state(GameState::Main)))
            .add_systems(Update, post_game_ui.run_if(in_state(GameState::post())))
            .add_systems(Update, pause_menu_ui.run_if(in_state(GameState::Pause)))
            .insert_resource(KbgpSettings {
                bindings: {
                    bevy_egui_kbgp::KbgpNavBindings::default()
                        .with_wasd_navigation()
                        .with_arrow_keys_navigation()
                        .with_gamepad_dpad_navigation_and_south_button_activation()
                        .with_key(KeyCode::Escape, KbgpNavCommand::user(UiActions::ToggleMenu))
                    //  .with_gamepad_button(GamepadButtonType::Start, KbgpNavCommand::user(UiActions::Menu))
                },
                ..default()
            });
    }
}

/// Set up the egui stuff and also spawn our menu scene
fn setup(
    mut cmd: Commands,
    mut egui_settings: ResMut<EguiSettings>,
    game_assets: Res<GameAssets>,
    gltf: Res<Assets<bevy::gltf::Gltf>>,
) {
    egui_settings.scale_factor = 2f64;
    cmd.spawn((
        SceneBundle {
            scene: gltf
                .get(game_assets.menu.id())
                .expect("level to load")
                .scenes[0]
                .clone(),
            ..default()
        },
        GameWorldTag,
        // // LoadedMarker,
        Name::new("menu"),
    ));

    let mut camera_transform = Transform::from_translation(Vec3::new(12.7695, -4.82982, 5.8834));
    camera_transform.look_at(Vec3::ZERO, Vec3::Z);
}

fn move_menu_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    ui_scene_settings: Res<settings::UiSceneSettings>,
) {
    if let Ok(mut transform) = camera.get_single_mut() {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_axis_angle(
                Vec3::Z,
                ui_scene_settings.rotation_speed * time.delta_seconds(),
            ),
        );
    }
}

fn menu_ui(
    mut cmd: Commands,
    mut ctx: EguiContexts,
    mut state: ResMut<NextState<GameState>>,
    mut selected_level: Local<CurrentLevelIndex>,
    game_assets: Res<GameAssets>,
) {
    let my_frame = egui::containers::Frame {
        inner_margin: egui::style::Margin {
            left: 10.,
            right: 10.,
            top: 10.,
            bottom: 10.,
        },
        outer_margin: egui::style::Margin {
            left: 10.,
            right: 10.,
            top: 10.,
            bottom: 10.,
        },
        rounding: egui::Rounding {
            nw: 1.0,
            ne: 1.0,
            sw: 1.0,
            se: 1.0,
        },

        fill: Color32::TRANSPARENT,
        stroke: egui::Stroke::new(0f32, Color32::TRANSPARENT),
        ..default()
    };

    let level_paths = game_assets.levels.clone();

    egui::CentralPanel::default()
        .frame(my_frame)
        .show(ctx.ctx_mut(), |ui| {
            let screen_size = ui.ctx().screen_rect();
            egui::Frame::default().show(ui, |ui| {
                // ui.vertical(|ui| {
                //     if ui.button("Start").clicked() {
                //         state.set(GameState::Main);
                //     }
                ui.allocate_ui_at_rect(
                    egui::Rect::from_center_size(
                        screen_size.center(),
                        egui::Vec2::new(300f32, 300f32),
                    ),
                    |ui| {
                        ui.vertical_centered_justified(|ui| {
                            egui_extras::install_image_loaders(ui.ctx());
                            ui.image(egui::include_image!("../../assets/images/logo.png"));
                            level_paths.iter().enumerate().for_each(|(i, level_path)| {
                                if ui
                                    .add(egui::SelectableLabel::new(
                                        selected_level.0 == i,
                                        format!("Level {i}"),
                                    ))
                                    .kbgp_navigation()
                                    .clicked()
                                {
                                    selected_level.0 = i
                                }
                            });
                            if ui
                                .button("Start")
                                .kbgp_navigation()
                                .kbgp_initial_focus()
                                .clicked()
                            {
                                cmd.insert_resource(CurrentLevelIndex(selected_level.0));
                                state.set(GameState::LevelTransition {
                                    level: level_paths[selected_level.0].clone(),
                                });
                            }
                        })
                    },
                );
                // });
            });

            //
        });
}

fn game_ui(
    mut ctx: EguiContexts,
    assets: Res<GameAssets>,
    font: Res<settings::FontSettings>,
    game_manager: Query<&GameManager>,
    game_start_time: Option<Res<GameTime>>,
    time: Res<Time>,
) {
    if let (Ok(game_manager), Some(game_start_time)) = (game_manager.get_single(), game_start_time)
    {
        egui::TopBottomPanel::new(egui::panel::TopBottomSide::Top, "game")
            .show_separator_line(false)
            .frame(Frame {
                fill: Color32::from_rgba_unmultiplied(0, 0, 0, 70),
                stroke: Stroke::NONE,
                ..default()
            })
            .show(ctx.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new(format!(
                            "Goal: {}/{}",
                            game_manager.current, game_manager.goal
                        ))
                        .color(Color32::WHITE)
                        .font(FontId::proportional(font.size_subtitle())),
                    );
                    if game_manager.current >= game_manager.goal {
                        ui.horizontal_centered(|ui| {
                            ui.label(
                                egui::RichText::new("Level complete")
                                    .font(FontId::proportional(font.size_body()))
                                    .color(Color32::GREEN),
                            )
                        });
                    };

                    let elapsed = game_start_time.elapsed(&time);
                    let seconds = elapsed.as_secs();
                    let millis = elapsed.as_millis() % 1000;
                    ui.label(
                        egui::RichText::new(format!("{:0>2}:{:0>3}", seconds, millis))
                            .color(Color32::WHITE)
                            .font(FontId::proportional(font.size_body())),
                    );
                });
            });
    }
}

fn post_game_ui(
    mut cmd: Commands,
    mut ctx: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    font: Res<settings::FontSettings>,
    game_time: Res<GameTime>,
    mut current_level: ResMut<CurrentLevelIndex>,
    game_assets: Res<GameAssets>,
) {
    let my_frame = egui::containers::Frame {
        fill: Color32::TRANSPARENT,
        ..default()
    };
    egui::CentralPanel::default()
        .frame(my_frame)
        .show(ctx.ctx_mut(), |ui| {
            let screen_size = ui.ctx().screen_rect();
            egui::Frame::default().show(ui, |ui| {
                // ui.vertical(|ui| {
                //     if ui.button("Start").clicked() {
                //         state.set(GameState::Main);
                //     }
                ui.allocate_ui_at_rect(
                    egui::Rect::from_center_size(
                        screen_size.center(),
                        egui::Vec2::new(500f32, 300f32),
                    ),
                    |ui| {
                        ui.vertical_centered_justified(|ui| {
                            let win = *match state.get() {
                                GameState::Post { win } => win,
                                _ => unreachable!(),
                            };
                            if win {
                                let time = game_time.final_time().unwrap();
                                let seconds = time.as_secs();
                                let millis = time.as_millis() % 1000;
                                ui.label(
                                    egui::RichText::new(format!("{:0>2}:{:0>3}", seconds, millis))
                                        .font(FontId::proportional(font.size_subtitle())),
                                );

                                // Go to the next nevel if there is a next level
                                let maybe_next_level = current_level.0 + 1;
                                match game_assets.levels.get(maybe_next_level) {
                                    Some(level) => {
                                        ui.label(
                                            egui::RichText::new("You win!")
                                                .font(FontId::proportional(font.size_title()))
                                                .color(Color32::DARK_GREEN),
                                        );
                                        if ui
                                            .button("Next level")
                                            .kbgp_navigation()
                                            .kbgp_initial_focus()
                                            .clicked()
                                        {
                                            current_level.0 += 1;
                                            next_state.set(GameState::LevelTransition {
                                                level: level.clone(),
                                            });
                                        }
                                    }
                                    None => {
                                        ui.label(
                                            egui::RichText::new("You beat the game epic style")
                                                .font(FontId::proportional(font.size_title()))
                                                .color(Color32::DARK_GREEN),
                                        );

                                        if ui
                                            .button("Main menu")
                                            .kbgp_navigation()
                                            .kbgp_initial_focus()
                                            .clicked()
                                        {
                                            next_state.set(GameState::Menu);
                                        }
                                    }
                                }
                            } else {
                                ui.label(
                                    egui::RichText::new("Out of bounds!")
                                        .font(FontId::proportional(font.size_title()))
                                        .color(Color32::DARK_RED),
                                );
                                if ui
                                    .button("Restart level")
                                    .kbgp_navigation()
                                    .kbgp_initial_focus()
                                    .clicked()
                                {
                                    next_state.set(GameState::LevelTransition {
                                        level: game_assets
                                            .levels
                                            .get(current_level.0)
                                            .unwrap()
                                            .clone(),
                                    });
                                }
                            }
                        })
                    },
                );
                // });
            });

            //
        });
}

fn pause_menu_ui(
    mut cmd: Commands,
    mut ctx: EguiContexts,
    mut state: ResMut<NextState<GameState>>,
    current_level: Res<CurrentLevelIndex>,
    assets: Res<GameAssets>,
) {
    egui::CentralPanel::default()
        .frame(
            Frame::default()
                .fill(Color32::TRANSPARENT)
                .stroke(Stroke::NONE),
        )
        .show(ctx.ctx_mut(), |ui| {
            let screen_size = ui.ctx().screen_rect();
            egui::Frame::default().show(ui, |ui| {
                ui.allocate_ui_at_rect(
                    egui::Rect::from_center_size(
                        screen_size.center(),
                        egui::Vec2::new(300f32, 300f32),
                    ),
                    |ui| {
                        ui.vertical_centered_justified(|ui| {
                            if ui
                                .button("Resume")
                                .kbgp_navigation()
                                .kbgp_initial_focus()
                                .clicked()
                            {
                                state.set(GameState::Main);
                            }

                            if ui.button("Restart level").kbgp_navigation().clicked() {
                                state.set(GameState::LevelTransition {
                                    level: assets.levels[current_level.0].clone(),
                                });
                            }

                            if ui.button("Main menu").kbgp_navigation().clicked() {
                                state.set(GameState::Menu);
                            }
                        })
                    },
                );
                // });
            });

            //
        });
}

fn toggle_debug(
    mut debug: ResMut<DebugMode>,
    mut physics_settings: ResMut<PhysicsDebugConfig>,
    actions: Res<ActionState<Action>>,
) {
    if actions.just_pressed(Action::ToggleDebug) {
        *debug = match debug.as_ref() {
            DebugMode::On => DebugMode::Off,
            DebugMode::Off => DebugMode::On,
        };
        // Also, we need to set our physics to draw conditionally as it doesn't have
        // the `run_if` like the egui inspector does
        physics_settings.enabled = !physics_settings.enabled;
    }
}
