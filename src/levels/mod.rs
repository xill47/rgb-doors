mod camera_fit;
pub mod tiles;

use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::LdtkIntCellAppExt, *};

use crate::{loading::LevelAssets, GameState};

use self::{
    camera_fit::camera_fit_inside_current_level,
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
            .add_system(spawn_level.in_schedule(OnEnter(GameState::Playing)))
            .add_system(camera_fit_inside_current_level);
    }
}

fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_assets.level.clone_weak(),
        ..default()
    });
}
