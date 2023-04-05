mod camera_fit;
pub mod panel;
pub mod tiles;

use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::{LdtkIntCellAppExt, LdtkEntityAppExt}, *};

use crate::{actions::Actions, loading::LevelAssets, GameState};

use self::{
    camera_fit::camera_fit_inside_current_level,
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
            .add_systems((spawn_level,).in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (setup_panel, respawn_on_level_reset, step_on_panel)
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(camera_fit_inside_current_level);
    }
}

fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_assets.level.clone_weak(),
        ..default()
    });
}

fn respawn_on_level_reset(
    mut commands: Commands,
    mut actions: EventReader<Actions>,
    level_assets: Res<LevelAssets>,
    ldtk_wrold_q: Query<Entity, With<Handle<LdtkAsset>>>,
) {
    for action in actions.iter() {
        if action.level_reset.is_some() {
            for entity in ldtk_wrold_q.iter() {
                commands.entity(entity).despawn_recursive();
            }
            commands.spawn(LdtkWorldBundle {
                ldtk_handle: level_assets.level.clone_weak(),
                ..default()
            });
        }
    }
}
