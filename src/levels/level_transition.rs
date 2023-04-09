use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::FieldValue, *};
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation};

use crate::{
    actions::MovementDirection,
    animation_finished,
    loading::SpriteAssets,
    player::{
        movement::TweenTranslation,
        movement_effects::{MovementSideEffects, SideEffect},
        Player,
    },
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

    #[grid_coords]
    grid_coords: GridCoords,
}

#[allow(clippy::type_complexity)]
pub fn spawn_finish(
    mut commands: Commands,
    mut finish_query: Query<
        (Entity, &EntityInstance, &mut Finish, &mut Transform),
        Without<AsepriteAnimation>,
    >,
    sprites: Res<SpriteAssets>,
    aseprites: Res<Assets<Aseprite>>,
) {
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
            commands.entity(entity).insert(aseprite_bundle);
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
    mut player_q: Query<
        (&GridCoords, &mut MovementSideEffects),
        (With<Player>, Changed<GridCoords>),
    >,
    mut finish_query: Query<(&Finish, &GridCoords)>,
    mut notifications: EventWriter<Notification>,
    mut level_transition: EventWriter<LevelTransition>,
) {
    for (player_grid_coords, mut forbid_movement) in player_q.iter_mut() {
        for (finish, grid_coords) in finish_query.iter_mut() {
            if player_grid_coords == grid_coords {
                if let Some(message) = &finish.message {
                    notifications.send(message.clone());
                }
                if finish.next_level.is_some() {
                    level_transition.send(LevelTransition);
                } else {
                    for movement_direction in MovementDirection::all() {
                        forbid_movement.set(movement_direction, SideEffect::DisabledMovement);
                    }
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
    mut lift_q: Query<(&mut AsepriteAnimation, Entity, &Transform), With<Finish>>,
    mut player_q: Query<(&mut MovementSideEffects, Entity, &Transform), With<Player>>,
    mut screen_q: Query<(Entity, &BackgroundColor), With<LevelScreen>>,
    finish_q: Query<&Finish>,
    time: Res<Time>,
    mut timer: Local<Timer>,
) {
    match *transition_step {
        LevelTransitionStep::None => {
            if !level_transition_req.is_empty() {
                level_transition_req.clear();
                *transition_step = LevelTransitionStep::LiftWillClose;
                for mut forbid_movement in player_q.iter_mut() {
                    for direction in MovementDirection::all() {
                        forbid_movement
                            .0
                            .set(direction, SideEffect::DisabledMovement);
                    }
                }
            }
        }
        LevelTransitionStep::LiftWillClose => {
            let mut successful = false;
            for (mut lift_anim, _, _) in lift_q.iter_mut() {
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
                if animation_finished(&lift_anim.0, &time, &sprites.lift, &aseprites)
                    .unwrap_or(true)
                {
                    lift_anim.0.pause();
                    *transition_step = LevelTransitionStep::ScreenWillClose
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
            for mut lift_anim in lift_q.iter_mut() {
                let lift_ase_handle = sprites.lift.clone_weak();
                let Some(lift_ase) = aseprites.get(&lift_ase_handle) else { continue; };
                *lift_anim.0 = AsepriteAnimation::new(lift_ase.info(), "lift_move");
                lift_anim.0.play();
                commmands.entity(lift_anim.1).insert(TweenTranslation {
                    start: lift_anim.2.translation,
                    end: lift_anim.2.translation + Vec3::new(0., -30., 0.),
                    duration: Duration::from_secs_f32(duration),
                    ..default()
                });
            }
            for player in player_q.iter_mut() {
                commmands.entity(player.1).insert(TweenTranslation {
                    start: player.2.translation,
                    end: player.2.translation + Vec3::new(0., -30., 0.),
                    duration: Duration::from_secs_f32(duration),
                    ..default()
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
