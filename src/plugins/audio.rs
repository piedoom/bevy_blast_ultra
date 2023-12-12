use bevy::{
    audio::{SpatialScale, VolumeLevel},
    prelude::*,
};
use bevy_xpbd_3d::components::AngularVelocity;

use crate::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), (init,)) //play_music.after(init)))
            .add_systems(Update, (manage_player_sounds, manage_goal_sounds));
        //.add_systems(OnEnter(GameState::Main), play_main_music);
    }
}

fn init(
    mut cmd: Commands,
    audio_settings: Res<settings::AudioSettings>,
    existing_music: Query<(), With<MusicTrack>>,
    tracks: Query<(&Name, &AudioSink), With<MusicTrack>>,
    game_assets: Res<GameAssets>,
) {
    cmd.insert_resource(SpatialScale::new(audio_settings.spatial_scale));

    if existing_music.is_empty() {
        cmd.spawn((
            AudioSourceBundle {
                source: game_assets
                    .music
                    .get("audio/music/music.ogg")
                    .unwrap()
                    .clone(),
                settings: PlaybackSettings::LOOP
                    .with_volume(bevy::audio::Volume::Relative(VolumeLevel::new(0.3f32))),
            },
            MusicTrack,
        ));
    }

    // if tracks.is_empty() {
    //     let handles = ["audio/music/menu.mp3", "audio/music/dnb.ogg"];
    //     handles.into_iter().for_each(|track_name| {
    //         let handle = game_assets.music.get(track_name).unwrap();
    //         cmd.spawn((
    //             AudioSourceBundle {
    //                 source: handle.clone(),
    //                 settings: PlaybackSettings::LOOP.paused(bevy::audio::Volume::Relative(VolumeLevel::new(volume))),
    //             },
    //             MusicTrack,
    //             Name::new(track_name),
    //         ));
    //     });
    // }
}

// fn play_music(
//     mut cmd: Commands,
//     tracks: Query<Entity, With<MusicTrack>>,
//     game_assets: Res<GameAssets>,
// ) {
//     tracks.for_each(|e| {
//         cmd.entity(e).despawn_recursive();
//     });

//     cmd.spawn((
//         AudioSourceBundle {
//             source: game_assets
//                 .music
//                 .get("audio/music/trance.mp3")
//                 .unwrap()
//                 .clone(),
//             settings: PlaybackSettings::LOOP
//                 .with_volume(bevy::audio::Volume::Relative(VolumeLevel::new(0.5f32))),
//         },
//         MusicTrack,
//     ));
// }

// fn play_main_music(
//     mut cmd: Commands,
//     game_assets: Res<GameAssets>,
//     tracks: Query<Entity, With<MusicTrack>>,
// ) {
//     tracks.for_each(|e| {
//         cmd.entity(e).despawn_recursive();
//     });

//     cmd.spawn((
//         AudioSourceBundle {
//             source: game_assets
//                 .music
//                 .get("audio/music/trance.mp3")
//                 .unwrap()
//                 .clone(),
//             settings: PlaybackSettings::LOOP
//                 .with_volume(bevy::audio::Volume::Relative(VolumeLevel::new(0.5f32))),
//         },
//         MusicTrack,
//     ));
// }

fn manage_player_sounds(
    mut players: Query<(
        &mut SpatialAudioSink,
        &AngularVelocity,
        Option<&Grounded>,
        Option<&ScoredPlayer>,
    )>,
) {
    players.for_each_mut(|(audio, angvel, is_grounded, is_scored)| {
        // adjust rolling sound volume
        if is_grounded.is_some() && !is_scored.is_some() {
            let angvel = (angvel.length() * 0.05);
            audio.set_volume((angvel.max(0.1) - 0.1) * 0.1);
            audio.set_speed((0.8f32..=1.1f32).lerp(angvel));
        } else {
            audio.set_volume(0.0);
        }
    });
}

fn manage_goal_sounds(
    mut cmd: Commands,
    mut goals: Query<&mut PlaybackSettings, With<GoalArea>>,
    scored: Query<(), Added<ScoredPlayer>>,
    game_assets: Option<Res<GameAssets>>,
    game_manager: Query<&GameManager>,
) {
    scored.for_each(|_| {
        if let (Some(game_assets), Ok(game_manager)) = (&game_assets, game_manager.get_single()) {
            cmd.spawn(AudioSourceBundle {
                source: game_assets.sfx.get("audio/sfx/score.ogg").unwrap().clone(),
                settings: PlaybackSettings::ONCE
                    .with_volume(bevy::audio::Volume::Relative(VolumeLevel::new(0.7f32)))
                    .with_speed((0.7..=1.0).lerp(game_manager.normalized_score())),
            });
        }
    });
}
