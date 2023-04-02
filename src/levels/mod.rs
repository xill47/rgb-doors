mod camera_fit;

use bevy::prelude::*;
use bevy_ecs_ldtk::*;

use crate::{loading::LevelAssets, GameState};

use self::camera_fit::camera_fit_inside_current_level;

pub struct LevelsPlugin {
    pub level_index: usize,
}

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(self.level_index))
            .add_system(spawn_level.in_schedule(OnEnter(GameState::Playing)))
            .add_system(camera_fit_inside_current_level);
    }
}

fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_assets.level.clone(),
        ..default()
    });
}
