use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{levels::tiles::Laser, loading::AudioAssets, player::Player, GameState};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_bgm.in_schedule(OnEnter(GameState::Playing)))
            .add_systems((step_audio,laser_audio).in_set(OnUpdate(GameState::Playing)));
    }
}

fn start_bgm(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play_with_settings(
        audio_assets.bgm.clone_weak(),
        PlaybackSettings::LOOP.with_volume(0.3),
    );
}

fn step_audio(
    player_q: Query<(), (Changed<GridCoords>, With<Player>)>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    if player_q.iter().next().is_some() {
        audio.play(audio_assets.step.clone_weak());
    }
}

fn laser_audio(
    laser_q: Query<(&Transform, &Laser), Changed<Laser>>,
    player_q: Query<&Transform, With<Player>>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    let Ok(player_transform) = player_q.get_single() else { return; };
    for (laser_transform, laser) in laser_q.iter() {
        if !laser.is_open {
            let distance = laser_transform
                .translation
                .distance(player_transform.translation);
            info!("Playing laser sound at distance {}", distance);
            if distance < 100.0 {
                audio.play_spatial(audio_assets.laser.clone_weak(), *player_transform, 1.0, laser_transform.translation);
            }
        }
    }
}
