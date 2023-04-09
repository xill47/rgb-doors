use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkAsset, LevelSelection};

use crate::{
    actions::Actions,
    loading::LevelAssets,
    player::death::Death,
    ui::notifications::{CleanNotificationQueue, Notification},
};

use super::spawn_level;

pub struct ResetLevelEvent;

#[allow(clippy::too_many_arguments)]
pub fn reset_level(
    mut commands: Commands,
    mut reset_level_event: EventReader<ResetLevelEvent>,
    level_assets: Res<LevelAssets>,
    ldtk_wrold_q: Query<Entity, With<Handle<LdtkAsset>>>,
    notification: EventWriter<Notification>,
    level_selection: Res<LevelSelection>,
    ldtk_world: Res<Assets<LdtkAsset>>,
    clean_notification: EventWriter<CleanNotificationQueue>,
) {
    if !reset_level_event.is_empty() {
        reset_level_event.clear();

        for entity in ldtk_wrold_q.iter() {
            commands.entity(entity).despawn_recursive();
        }
        spawn_level(
            commands,
            level_assets,
            level_selection,
            ldtk_world,
            notification,
            clean_notification,
        );
    }
}

pub fn respawn_on_level_reset(
    mut actions: EventReader<Actions>,
    mut notify: EventWriter<Notification>,
    mut reset_level_event: EventWriter<ResetLevelEvent>,
) {
    if actions.iter().any(|action| action.level_reset.is_some()) {
        info!("Respawning on level reset");
        notify.send(Notification {
            text: "Resetting level...".into(),
            duration: Duration::from_secs(1),
        });
        reset_level_event.send(ResetLevelEvent);
    }
}

pub fn respawn_on_death(
    mut death: EventReader<Death>,
    mut notify: EventWriter<Notification>,
    mut reset_level_event: EventWriter<ResetLevelEvent>,
) {
    if !death.is_empty() {
        death.clear();
        info!("Respawning on death");
        notify.send(Notification {
            text: "You died :(".into(),
            duration: Duration::from_secs(1),
        });
        reset_level_event.send(ResetLevelEvent);
    }
}
