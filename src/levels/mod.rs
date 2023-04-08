mod camera_fit;
pub mod lasers;
pub mod panel;
pub mod tiles;

use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::{FieldValue, LdtkEntityAppExt, LdtkIntCellAppExt},
    *,
};

use crate::{
    actions::Actions, loading::LevelAssets, player::death::Death, ui::notifications::Notification,
    GameState,
};

use self::{
    camera_fit::camera_fit_inside_current_level,
    lasers::{hide_lasers, spawn_lasers, LaserBundle},
    panel::{setup_panel, step_on_panel, PanelBundle},
    tiles::WallBundle,
    tiles::{DoorBundle, FloorBundle},
};

pub struct LevelsPlugin {
    pub level_index: usize,
}

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .insert_resource(LevelSelection::Index(self.level_index))
            .register_ldtk_int_cell_for_layer::<WallBundle>("IntGrid", 1)
            .register_ldtk_int_cell_for_layer::<FloorBundle>("IntGrid", 2)
            .register_ldtk_int_cell_for_layer::<DoorBundle>("IntGrid", 3)
            .register_ldtk_int_cell_for_layer::<DoorBundle>("IntGrid", 4)
            .register_ldtk_int_cell_for_layer::<DoorBundle>("IntGrid", 5)
            .register_ldtk_entity::<PanelBundle>("Panel")
            .register_ldtk_entity::<LaserBundle>("Laser")
            .add_systems((spawn_level, hide_int_grid).in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (
                    setup_panel,
                    respawn_on_level_reset,
                    spawn_lasers,
                    hide_lasers.after(spawn_lasers),
                    respawn_on_death,
                    hide_int_grid,
                    step_on_panel,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(camera_fit_inside_current_level);
    }
}

fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    level_selection: Res<LevelSelection>,
    ldtk_world: Res<Assets<LdtkAsset>>,
    mut notification: EventWriter<Notification>,
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_assets.level.clone_weak(),
        ..default()
    });
    let ldtk_world = ldtk_world
        .get(&level_assets.level.clone_weak())
        .expect("Level asset not loaded");
    let Some(level) = ldtk_world.get_level(&level_selection) else { return; };
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

fn respawn_on_level_reset(
    mut commands: Commands,
    mut actions: EventReader<Actions>,
    level_assets: Res<LevelAssets>,
    ldtk_wrold_q: Query<Entity, With<Handle<LdtkAsset>>>,
    mut notify: EventWriter<Notification>,
    level_selection: Res<LevelSelection>,
    ldtk_world: Res<Assets<LdtkAsset>>,
) {
    if actions.iter().any(|action| action.level_reset.is_some()) {
        for entity in ldtk_wrold_q.iter() {
            commands.entity(entity).despawn_recursive();
        }
        notify.send(Notification {
            text: "Resetting level...".into(),
            duration: Duration::from_secs(1),
        });
        spawn_level(commands, level_assets, level_selection, ldtk_world, notify);
    }
}

fn respawn_on_death(
    mut commands: Commands,
    mut death: EventReader<Death>,
    level_assets: Res<LevelAssets>,
    ldtk_wrold_q: Query<Entity, With<Handle<LdtkAsset>>>,
    mut notify: EventWriter<Notification>,
    level_selection: Res<LevelSelection>,
    ldtk_world: Res<Assets<LdtkAsset>>,
) {
    if death.iter().len() > 0 {
        for entity in ldtk_wrold_q.iter() {
            commands.entity(entity).despawn_recursive();
        }
        notify.send(Notification {
            text: "You died :(".into(),
            duration: Duration::from_secs(1),
        });
        spawn_level(commands, level_assets, level_selection, ldtk_world, notify);
    }
}
