use bevy::{audio::VolumeLevel, prelude::*};
use bevy_gltf_blueprints::{BluePrintBundle, BlueprintName, GltfBlueprintsSet};
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::prelude::*;

use super::input::Action;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                transition_on_win,
                manage_goals,
                deactivate_out_of_bounds_players,
                check_for_loss,
                // restart,
                setup_goals.after(GltfBlueprintsSet::AfterSpawn),
            )
                .run_if(in_state(GameState::Main)),
        )
        .add_systems(Update, toggle_pause)
        .add_systems(OnEnter(GameState::Pause), pause_physics)
        .add_systems(OnEnter(GameState::Main), unpause_physics);
    }
}

fn setup_goals(
    mut cmd: Commands,
    goals: Query<Entity, Added<GoalArea>>,
    game_assets: Res<GameAssets>,
) {
    goals.for_each(|goal| {
        cmd.entity(goal)
            .insert(CollisionLayers::new([Layer::Sensor], [Layer::Player]))
            .insert(AudioBundle {
                source: game_assets.sfx.get("audio/sfx/score.ogg").unwrap().clone(),
                settings: PlaybackSettings::ONCE
                    .with_spatial(true)
                    .with_volume(bevy::audio::Volume::Relative(VolumeLevel::new(0.3f32)))
                    .paused(),
            });
    })
}

/// Consume marbles within the playable area
fn manage_goals(
    mut cmd: Commands,
    mut game_manager: Query<&mut GameManager>,
    unscored_players: Query<Entity, (With<Player>, Without<ScoredPlayer>)>,
    // TODO: This logic isn't always workingl,;m
    unscored_active_players: Query<Entity, (With<Player>, Without<ScoredPlayer>, With<Active>)>,
    goals: Query<(Entity, &CollidingEntities), (With<Sensor>, With<GoalArea>)>,
) {
    if let Ok(mut game_manager) = game_manager.get_single_mut() {
        goals.for_each(|(goal_entity, goal_collisions)| {
            let players_in_goal = goal_collisions
                .iter()
                .filter(|x| unscored_players.contains(**x));
            let players_in_goal_count = players_in_goal.clone().count();
            // We need to ensure that if the scoring players is equal to the
            // current amount of unscored players in general that we either
            // A. check for a win state, and trigger it, or
            // B. leave one player unscored to complete the level
            let players_to_skip = if players_in_goal_count == unscored_active_players.iter().len() {
                // All active players are also all in the score area. Check if
                // this results in a win condition
                if game_manager.current + players_in_goal_count >= game_manager.goal {
                    // Win condition success! Score everything normally, then
                    // the game will win
                    0
                } else {
                    // Remove one of the entities from scoring so it can finish
                    // the level
                    1
                }
            } else {
                // There are still active entities outside the goal, so score
                // everything
                0
            };
            players_in_goal.skip(players_to_skip).for_each(|player| {
                cmd.entity(*player).remove::<Active>().insert((
                    ScoredPlayer,
                    GravityScale(0f32),
                    LinearDamping(0.8f32),
                    ForceTowards {
                        speed: 0.2f32,
                        target: goal_entity,
                        offset: Vec3::Z * 3f32,
                    },
                ));
                game_manager.current += 1;
            });
        });
    }
}

/// Go to the post game state when we win
fn transition_on_win(
    mut next_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<GameTime>,
    game_manager: Query<&mut GameManager>,
    time: Res<Time>,
) {
    if let Ok(game_manager) = game_manager.get_single() {
        if game_manager.current >= game_manager.goal {
            // Game win
            next_state.set(GameState::Post { win: true });
        }
    }
}

/// Deactivate out of bounds players and add a [`OutOfBounds`] component to mark
/// it for cleanup/respawn
fn deactivate_out_of_bounds_players(
    mut cmd: Commands,
    players: Query<(Entity, &Transform), (With<Player>, With<Active>)>,
    floor: Query<&Transform, With<FloorBounds>>,
) {
    if let Ok(floor) = floor.get_single() {
        players.for_each(|(entity, transform)| {
            if transform.translation.z < floor.translation.z {
                cmd.entity(entity).remove::<Active>().insert(OutOfBounds);
            }
        });
    }
}

fn check_for_loss(
    mut next_state: ResMut<NextState<GameState>>,
    all_players: Query<Entity, (With<Player>, Without<ScoredPlayer>)>,
    active_players: Query<
        (),
        (
            With<Player>,
            Without<ScoredPlayer>,
            With<Active>,
            Without<OutOfBounds>,
        ),
    >,
    new_oob_players: Query<(), (With<Player>, Added<OutOfBounds>)>,
) {
    if !new_oob_players.is_empty() {
        // ensure we still have some active players
        if active_players.is_empty() {
            next_state.set(GameState::Post { win: false })
        }
    }
}

/// Checks to see if we should be pausing or unpausing the game, and changes the state
fn toggle_pause(
    actions: Res<ActionState<Action>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if actions.just_pressed(Action::TogglePause) {
        match state.get() {
            GameState::Pause => {
                next_state.set(GameState::Main);
            }
            GameState::Main => {
                next_state.set(GameState::Pause);
            }
            _ => (),
        }
    }
}

fn pause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.pause();
}

fn unpause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.unpause();
}

// fn restart(
//     mut cmd: Commands,
//     actions: Res<ActionState<Action>>,
//     used_powerups: Query<Entity, (With<Powerup>, Without<Active>)>,
//     players: Query<Entity, With<Player>>,
//     spawn: Query<&Transform, With<Spawn>>,
//     player_settings: Res<settings::PlayerSettings>,
//     mut timer: ResMut<GameTime>,
//     influence: Query<Entity, With<PlayerInfluenceRadius>>,
//     focus: Query<Entity, With<FocalPoint>>,
//     // mut state: ResMut<NextState<GameState>>,
//     time: Res<Time>,
// ) {
//     if actions.just_pressed(Action::TogglePause) {
//         timer.reset(&time);
//         // Reset all game objects to their initials
//         players.for_each(|e| cmd.entity(e).despawn_recursive());
//         used_powerups.for_each(|e| {
//             cmd.entity(e).insert(Active);
//         });
//         cmd.entity(influence.single()).despawn_recursive();
//         cmd.entity(focus.single()).despawn_recursive();

//         // Reset the player spawn, focus area, and influence
//         let transform = spawn.single();

//         // TODO: This is the same as in the levels plugin. There's some cleanup
//         // work to do there to make this code more maintainable
//         cmd.spawn((
//             PlayerInfluenceRadius(player_settings.influence_radius),
//             *transform,
//         ));

//         // Spawn in the player entity
//         // Spawn new entity
//         cmd.spawn(BluePrintBundle {
//             blueprint: BlueprintName("player".to_string()),
//             transform: TransformBundle::from_transform(*transform),
//             ..Default::default()
//         })
//         .insert((Active, Name::new("player")));

//         cmd.spawn((FocalPoint, *transform));
//     }
// }
