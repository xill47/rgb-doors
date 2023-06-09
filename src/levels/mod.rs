mod camera_fit;
pub mod lasers;
mod level_transition;
pub mod panel;
pub mod reset;
pub mod tiles;

use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::{FieldValue, LdtkEntityAppExt, LdtkIntCellAppExt},
    *,
};
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation};

use crate::{
    loading::LevelAssets,
    ui::notifications::{CleanNotificationQueue, Notification},
    GameState,
};

use self::{
    camera_fit::camera_fit_inside_current_level,
    lasers::{laser_visibility, spawn_lasers, LaserBundle},
    level_transition::{
        finish_system, level_transition, spawn_finish, FinishBundle, LevelTransition,
    },
    panel::{setup_panel, step_on_panel, PanelBundle},
    reset::{reset_level, respawn_on_death, respawn_on_level_reset, ResetLevelEvent},
    tiles::WallBundle,
    tiles::{DoorBundle, FloorBundle},
};

pub struct LevelsPlugin {
    pub level_index: usize,
}

#[derive(Default, Bundle)]
pub struct RgbEntityAsepriteBundle {
    pub aseprite: Handle<Aseprite>,
    pub animation: AsepriteAnimation,
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
}

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .insert_resource(LevelSelection::Index(self.level_index))
            .add_event::<LevelTransition>()
            .add_event::<ResetLevelEvent>()
            .register_ldtk_int_cell_for_layer::<WallBundle>("IntGrid", 1)
            .register_ldtk_int_cell_for_layer::<FloorBundle>("IntGrid", 2)
            .register_ldtk_int_cell_for_layer::<DoorBundle>("IntGrid", 3)
            .register_ldtk_int_cell_for_layer::<DoorBundle>("IntGrid", 4)
            .register_ldtk_int_cell_for_layer::<DoorBundle>("IntGrid", 5)
            .register_ldtk_entity::<PanelBundle>("Panel")
            .register_ldtk_entity::<LaserBundle>("Laser")
            .register_ldtk_entity::<FinishBundle>("Finish")
            .add_systems((spawn_level, hide_int_grid).in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (
                    setup_panel,
                    respawn_on_level_reset,
                    spawn_lasers,
                    laser_visibility.after(spawn_lasers),
                    spawn_finish,
                    respawn_on_death,
                    reset_level,
                    hide_int_grid,
                    step_on_panel,
                    finish_system,
                    level_transition.after(finish_system),
                )
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(camera_fit_inside_current_level);
    }
}

pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    level_selection: Res<LevelSelection>,
    ldtk_world: Res<Assets<LdtkAsset>>,
    mut notification: EventWriter<Notification>,
    mut clean_notification: EventWriter<CleanNotificationQueue>,
) {
    info!("Spawning level: {:?}", level_selection);
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_assets.level.clone_weak(),
        ..default()
    });
    let ldtk_world = ldtk_world
        .get(&level_assets.level.clone_weak())
        .expect("Level asset not loaded");
    let Some(level) = ldtk_world.get_level(&level_selection) else { return; };
    clean_notification.send(CleanNotificationQueue);
    level.field_instances.iter().for_each(|field| {
        if field.identifier == "Notifications" {
            if let FieldValue::Strings(notifications) = &field.value {
                notifications.iter().for_each(|notification_text| {
                    if let Some(notification_text) = notification_text {
                        notification.send(Notification {
                            text: notification_text.clone(),
                            duration: Duration::from_secs_f32(2.5),
                        });
                    }
                });
            }
        }
    });
}

fn hide_int_grid(mut ldtk_int_grid_q: Query<(&mut Visibility, &Name), Added<LayerMetadata>>) {
    for (mut visibility, name) in ldtk_int_grid_q.iter_mut() {
        if name.as_str() == "IntGrid" {
            *visibility = Visibility::Hidden;
        }
    }
}
