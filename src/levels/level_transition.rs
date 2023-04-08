use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::FieldValue, *};
use bevy_ecs_tilemap::prelude::TilemapSize;
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation};

use crate::{
    actions::PlayerMovement,
    grid_coords_from_instance,
    loading::SpriteAssets,
    player::{forbid_movement::ForbiddenMovement, Player},
    ui::{bg_color_tween::BackgroundColorTween, notifications::Notification, LevelScreen},
};

use super::RgbEntityAsepriteBundle;

#[derive(Component, Debug, Default)]
pub struct Finish {
    next_level: Option<LevelSelection>,
    message: Option<Notification>,
}

pub struct LevelTransition;

#[derive(Bundle, LdtkEntity)]
pub struct FinishBundle {
    finish: Finish,

    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[allow(clippy::type_complexity)]
pub fn spawn_finish(
    mut commands: Commands,
    mut finish_query: Query<
        (Entity, &EntityInstance, &mut Finish, &mut Transform),
        Without<AsepriteAnimation>,
    >,
    tilemap_size_q: Query<&TilemapSize>,
    sprites: Res<SpriteAssets>,
    aseprites: Res<Assets<Aseprite>>,
) {
    let Some(tilemap_size) = tilemap_size_q.iter().next() else { return; };
    for (entity, entity_instance, mut finish, mut transform) in finish_query.iter_mut() {
        finish.next_level = entity_instance
            .field_instances
            .iter()
            .find(|field_instance| field_instance.identifier == "NextLevel")
            .and_then(|field_instance| match &field_instance.value {
                FieldValue::String(Some(value)) => Some(LevelSelection::Iid(value.to_owned())),
                _ => None,
            });
        finish.message = entity_instance
            .field_instances
            .iter()
            .find(|field_instance| field_instance.identifier == "Message")
            .and_then(|field_instance| match &field_instance.value {
                FieldValue::String(Some(value)) => Some(Notification::new(value.to_owned())),
                _ => None,
            });
        transform.translation.y += 8.0;
        if let Some(aseprite_bundle) = finish_sprite(&sprites, &aseprites) {
            commands
                .entity(entity)
                .insert(aseprite_bundle)
                .insert(grid_coords_from_instance(entity_instance, tilemap_size));
        }
    }
}

fn finish_sprite(
    sprites: &SpriteAssets,
    aseprites: &Assets<Aseprite>,
) -> Option<RgbEntityAsepriteBundle> {
    let lift_ase_handle = sprites.lift.cast_weak();
    let lift_ase = aseprites.get(&lift_ase_handle)?;
    let mut lift_anim = AsepriteAnimation::new(lift_ase.info(), "left_door_close");
    lift_anim.pause();
    Some(RgbEntityAsepriteBundle {
        aseprite: lift_ase_handle.clone_weak(),
        texture_atlas: lift_ase.atlas().clone_weak(),
        sprite: TextureAtlasSprite::new(lift_anim.current_frame()),
        animation: lift_anim,
    })
}

#[allow(clippy::type_complexity)]
pub fn finish_system(
    player_q: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    mut finish_query: Query<(&Finish, &GridCoords)>,
    mut notifications: EventWriter<Notification>,
    mut level_transition: EventWriter<LevelTransition>,
) {
    for player_grid_coords in player_q.iter() {
        for (finish, grid_coords) in finish_query.iter_mut() {
            if player_grid_coords == grid_coords {
                if let Some(message) = &finish.message {
                    notifications.send(message.clone());
                }
                if finish.next_level.is_some() {
                    level_transition.send(LevelTransition);
                }
            }
        }
    }
}

#[derive(Default)]
pub enum LevelTransitionStep {
    #[default]
    None,
    LiftWillClose,
    LiftCloses,
    ScreenWillClose,
    ScreenCloses,
    ScreenWillOpen,
    ScreenOpens,
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn level_transition(
    mut commmands: Commands,
    mut transition_step: Local<LevelTransitionStep>,
    mut level_selection: ResMut<LevelSelection>,
    mut level_transition_req: EventReader<LevelTransition>,
    sprites: Res<SpriteAssets>,
    aseprites: Res<Assets<Aseprite>>,
    mut lift_q: Query<&mut AsepriteAnimation, With<Finish>>,
    mut player_q: Query<&mut ForbiddenMovement, With<Player>>,
    mut screen_q: Query<(Entity, &BackgroundColor), With<LevelScreen>>,
    finish_q: Query<&Finish>,
    time: Res<Time>,
    mut timer: Local<Timer>,
) {
    match *transition_step {
        LevelTransitionStep::None => {
            if level_transition_req.iter().next().is_some() {
                *transition_step = LevelTransitionStep::LiftWillClose;
                for mut forbid_movement in player_q.iter_mut() {
                    forbid_movement.forbidden = PlayerMovement::all().into_iter().collect();
                }
            }
        }
        LevelTransitionStep::LiftWillClose => {
            let mut successful = false;
            for mut lift_anim in lift_q.iter_mut() {
                let lift_ase_handle = sprites.lift.clone_weak();
                let Some(lift_ase) = aseprites.get(&lift_ase_handle) else { successful = false; continue; };
                *lift_anim = AsepriteAnimation::new(lift_ase.info(), "left_door_close");
                lift_anim.play();
                successful = true;
            }
            *transition_step = if successful {
                LevelTransitionStep::LiftCloses
            } else {
                LevelTransitionStep::ScreenWillClose
            };
        }
        LevelTransitionStep::LiftCloses => {
            for mut lift_anim in lift_q.iter_mut() {
                if let Some(level_transition_step) =
                    lift_closes(&sprites, &aseprites, &mut lift_anim, &time)
                {
                    *transition_step = level_transition_step;
                } else {
                    *transition_step = LevelTransitionStep::ScreenWillClose;
                }
            }
        }
        LevelTransitionStep::ScreenWillClose => {
            let duration = 1.5;
            *timer = Timer::from_seconds(duration, TimerMode::Once);
            for (entity, &background_color) in screen_q.iter_mut() {
                commmands.entity(entity).insert(BackgroundColorTween {
                    start_color: background_color.0,
                    end_color: Color::BLACK,
                    duration,
                    after_color: Color::BLACK,
                    elapsed: 0.,
                });
            }
            *transition_step = LevelTransitionStep::ScreenCloses;
        }
        LevelTransitionStep::ScreenCloses => {
            if timer.tick(time.delta()).just_finished() {
                *transition_step = LevelTransitionStep::ScreenWillOpen;
            }
        }
        LevelTransitionStep::ScreenWillOpen => {
            let duration = 3.;
            *timer = Timer::from_seconds(duration, TimerMode::Once);
            for finish in finish_q.iter() {
                if let Some(next_level) = &finish.next_level {
                    *level_selection = next_level.clone();
                }
            }
            for (entity, &background_color) in screen_q.iter_mut() {
                commmands.entity(entity).insert(BackgroundColorTween {
                    start_color: background_color.0,
                    end_color: Color::rgba(0., 0., 0., 0.),
                    duration,
                    after_color: Color::rgba(0., 0., 0., 0.),
                    elapsed: 0.,
                });
            }
            *transition_step = LevelTransitionStep::ScreenOpens;
        }
        LevelTransitionStep::ScreenOpens => {
            if timer.tick(time.delta()).just_finished() {
                *transition_step = LevelTransitionStep::None;
            }
        }
    }
}

fn lift_closes(
    sprites: &SpriteAssets,
    aseprites: &Assets<Aseprite>,
    lift_anim: &mut AsepriteAnimation,
    time: &Time,
) -> Option<LevelTransitionStep> {
    let lift_ase_handle = sprites.lift.clone_weak();
    let lift_ase = aseprites.get(&lift_ase_handle)?;
    let remaining_frames = lift_anim.remaining_tag_frames(lift_ase.info())?;
    if remaining_frames < 1 && lift_anim.frame_finished(time.delta()) {
        lift_anim.pause();
        return Some(LevelTransitionStep::ScreenWillClose);
    }
    Some(LevelTransitionStep::LiftCloses)
}
