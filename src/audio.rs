use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{
    levels::panel::PressurePlate, loading::AudioAssets, player::Player, ui::MuteControl, GameState,
};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioConfig>()
            .add_system(start_bgm.in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (step_audio, mute_control, bgm_mute_control).in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Resource, Default)]
pub struct AudioConfig {
    mute: bool,
    bgm_handle: Option<Handle<AudioSink>>,
}

fn start_bgm(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut audio_config: ResMut<AudioConfig>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    if audio_config.bgm_handle.is_none() {
        let weak_handle = audio.play_with_settings(
            audio_assets.bgm.clone_weak(),
            PlaybackSettings::LOOP.with_volume(0.3),
        );
        audio_config.bgm_handle = audio_sinks.get_handle(weak_handle).into();
    }
}

fn bgm_mute_control(audio_config: Res<AudioConfig>, audio_sinks: Res<Assets<AudioSink>>) {
    if audio_config.is_changed() {
        info!("Bgm mute changed: {:?}", audio_config.mute);
        if let Some(handle) = &audio_config.bgm_handle {
            info!("Strong handle: {:?}", handle);
            if let Some(sink) = audio_sinks.get(handle) {
                if audio_config.mute {
                    info!("Pausing bgm");
                    sink.pause();
                } else {
                    sink.play();
                }
            }
        }
    }
}

fn step_audio(
    player_q: Query<&GridCoords, (Changed<GridCoords>, With<Player>)>,
    pressure_plate_q: Query<&GridCoords, With<PressurePlate>>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    audio_config: Res<AudioConfig>,
) {
    if audio_config.mute {
        return;
    }
    for coords in player_q.iter() {
        if pressure_plate_q.iter().any(|l_coords| l_coords == coords) {
            audio.play(audio_assets.switch.clone_weak());
        }
        audio.play(audio_assets.step.clone_weak());
    }
}

fn mute_control(
    mut audio_config: ResMut<AudioConfig>,
    mute_control_q: Query<&Interaction, (With<MuteControl>, Changed<Interaction>)>,
) {
    for interaction in mute_control_q.iter() {
        info!("Mute control interaction: {:?}", interaction);
        if *interaction == Interaction::Clicked {
            audio_config.mute = !audio_config.mute;
            info!("Mute control: {:?}", audio_config.mute);
        }
    }
}
