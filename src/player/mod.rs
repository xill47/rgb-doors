pub mod color_control;
pub mod death;
pub mod movement;
pub mod movement_effects;
pub mod open_lasers;

use crate::levels::RgbEntityAsepriteBundle;
use crate::loading::SpriteAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation};

use self::color_control::{set_color_control_from_action, ColorControl};
use self::death::{die_on_tile_with_door, play_death_animation, Death, Dying};
use self::movement::{
    change_transform_based_on_grid, next_movement_state, player_action_to_movement,
    tween_translations, MovementState,
};
use self::movement_effects::MovementSideEffects;
use self::open_lasers::*;

pub struct PlayerPlugin;

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[from_entity_instance]
    entity_instance: EntityInstance,

    #[grid_coords]
    grid_coords: GridCoords,

    player: Player,
    movement_state: MovementState,
    color_control: ColorControl,
    forbidden_movement: MovementSideEffects,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Death>()
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(
                (
                    spawn_player_sprite,
                    player_action_to_movement,
                    change_transform_based_on_grid,
                    tween_translations.after(change_transform_based_on_grid),
                    die_on_tile_with_door
                        .after(open_lasers)
                        .after(next_movement_state),
                    next_movement_state,
                    switch_player_animation_color_change,
                    switch_player_animation_movement_change,
                    open_lasers.after(set_color_control_from_action),
                    set_color_control_from_action,
                    play_death_animation,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[allow(clippy::type_complexity)]
fn spawn_player_sprite(
    mut commands: Commands,
    mut player_q: Query<
        (Entity, &mut Transform, &ColorControl, &MovementState),
        (With<Player>, Without<AsepriteAnimation>),
    >,
    sprites: Res<SpriteAssets>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for (entity, mut transform, color_control, movement_state) in player_q.iter_mut() {
        let player_ase_handle = sprites.player.clone_weak();
        let player_ase = aseprites.get(&player_ase_handle).unwrap();
        let anim_info = movement_state.anim_info(color_control);
        let player_anim = AsepriteAnimation::new(player_ase.info(), anim_info.tag_name);
        transform.translation.z += 1.;
        transform.translation.y += 8.;
        info!("spawn player sprite for {:?}", entity);
        commands.entity(entity).insert(RgbEntityAsepriteBundle {
            texture_atlas: player_ase.atlas().clone_weak(),
            sprite: TextureAtlasSprite {
                flip_x: anim_info.flip_x,
                ..TextureAtlasSprite::new(player_anim.current_frame())
            },
            aseprite: player_ase_handle,
            animation: player_anim,
        });
    }
}

#[allow(clippy::type_complexity)]
fn switch_player_animation_color_change(
    mut player_query: Query<
        (
            &mut AsepriteAnimation,
            &mut TextureAtlasSprite,
            &ColorControl,
            &MovementState,
        ),
        (With<Player>, Changed<ColorControl>, Without<Dying>),
    >,
    sprites: Res<SpriteAssets>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for (mut animation, mut sprite, color_control, movement_state) in player_query.iter_mut() {
        let player_ase_handle = sprites.player.clone_weak();
        let player_ase = aseprites.get(&player_ase_handle).unwrap();
        let ase_info = player_ase.info();
        let anim_info = movement_state.anim_info(color_control);
        let anim_tag = anim_info.tag_name;
        let mut next_animation = AsepriteAnimation::new(player_ase.info(), anim_tag);
        let next_animation_frame =
            animation
                .current_tag_frame(ase_info)
                .and_then(|current_tag_frame| {
                    ase_info
                        .tags
                        .get(anim_tag)
                        .map(|tag| tag.frames.start as usize + current_tag_frame)
                });
        if let Some(next_animation_frame) = next_animation_frame {
            next_animation.set_current_frame(next_animation_frame);
        }
        *animation = next_animation;
        *sprite = TextureAtlasSprite {
            flip_x: anim_info.flip_x,
            ..TextureAtlasSprite::new(animation.current_frame())
        };
    }
}

#[allow(clippy::type_complexity)]
fn switch_player_animation_movement_change(
    mut player_query: Query<
        (
            &mut AsepriteAnimation,
            &mut TextureAtlasSprite,
            &ColorControl,
            &MovementState,
        ),
        (With<Player>, Changed<MovementState>, Without<Dying>),
    >,
    sprites: Res<SpriteAssets>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for (mut animation, mut sprite, color_control, movement_state) in player_query.iter_mut() {
        let player_ase_handle = sprites.player.clone_weak();
        let player_ase = aseprites.get(&player_ase_handle).unwrap();
        let anim_info = movement_state.anim_info(color_control);
        let anim_tag = anim_info.tag_name;
        let next_animation = AsepriteAnimation::new(player_ase.info(), anim_tag);
        *animation = next_animation;
        *sprite = TextureAtlasSprite {
            flip_x: anim_info.flip_x,
            ..TextureAtlasSprite::new(animation.current_frame())
        };
    }
}
