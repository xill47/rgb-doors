use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::LdtkAsset;
use bevy_mod_aseprite::Aseprite;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        )
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, LevelAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, SpriteAssets>(GameState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "sounds/laser.ogg")]
    pub laser: Handle<AudioSource>,
    #[asset(path = "sounds/switch.ogg")]
    pub switch: Handle<AudioSource>,
    #[asset(path = "sounds/bgm.ogg")]
    pub bgm: Handle<AudioSource>,
    #[asset(path = "sounds/step.ogg")]
    pub step: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    #[asset(path = "levels/levels.ldtk")]
    pub level: Handle<LdtkAsset>,
}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "sprites/player.aseprite")]
    pub player: Handle<Aseprite>,
    #[asset(path = "sprites/plates.aseprite")]
    pub plates: Handle<Aseprite>,
    #[asset(path = "sprites/h_lasers.aseprite")]
    pub h_lasers: Handle<Aseprite>,
    #[asset(path = "sprites/v_lasers.aseprite")]
    pub v_lasers: Handle<Aseprite>,
    #[asset(path = "sprites/lift.aseprite")]
    pub lift: Handle<Aseprite>,
}
