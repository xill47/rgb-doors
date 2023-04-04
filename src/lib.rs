mod actions;
mod audio;
mod levels;
mod loading;
mod menu;
mod player;
mod ui;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::levels::LevelsPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_ecs_ldtk::{GridCoords, EntityInstance};
use bevy_ecs_tilemap::prelude::TilemapSize;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ui::UIPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Playing,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(LevelsPlugin { level_index: 0 })
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(UIPlugin)
            .add_startup_system(spawn_camera);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(WorldInspectorPlugin::new());
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

pub fn grid_coords_from_instance(ldtk_instance: &EntityInstance, tilemap_size: &TilemapSize) -> GridCoords {
    GridCoords {
        x: ldtk_instance.grid.x,
        y: tilemap_size.y as i32 - ldtk_instance.grid.y - 1,
    }
}
