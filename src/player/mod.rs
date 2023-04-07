pub mod color_control;
pub mod death;
pub mod forbid_movement;
pub mod ignore_doors;
pub mod movement;

use crate::grid_coords_from_instance;
use crate::loading::SpriteAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::TilemapSize;
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation, AsepriteBundle};

use self::color_control::{set_color_control_from_action, ColorControl};
use self::death::{die_on_tile_with_door, Death};
use self::forbid_movement::ForbiddenMovement;
use self::ignore_doors::*;
use self::movement::{
    change_transform_based_on_grid, move_player_on_grid, return_to_idle, tween_translations,
    MovementState,
};

pub struct PlayerPlugin;

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[from_entity_instance]
    entity_instance: EntityInstance,

    player: Player,
    movement_state: MovementState,
    ignore_doors: IgnoreDoors,
    color_control: ColorControl,
    forbidden_movement: ForbiddenMovement,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Death>()
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(
                (
                    spawn_player_sprite,
                    move_player_on_grid,
                    change_transform_based_on_grid,
                    tween_translations.after(change_transform_based_on_grid),
                    die_on_tile_with_door,
                    return_to_idle,
                    switch_player_animation,
                    ignore_doors_on_panel_press,
                    set_color_control_from_action,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[allow(clippy::type_complexity)]
fn spawn_player_sprite(
    mut commands: Commands,
    player_q: Query<
        (
            Entity,
            &Transform,
            &EntityInstance,
            &ColorControl,
            &MovementState,
        ),
        (With<Player>, Without<AsepriteAnimation>),
    >,
    tilemap_q: Query<&TilemapSize>,
    sprites: Res<SpriteAssets>,
    aseprites: Res<Assets<Aseprite>>,
) {
    let Some(tilemap_size) = tilemap_q.iter().next() else { return;};
    for (entity, transform, ldtk_instance, color_control, movement_state) in player_q.iter() {
        let player_ase_handle = sprites.player.clone_weak();
        let player_ase = aseprites.get(&player_ase_handle).unwrap();
        let anim_info = movement_state.anim_info(color_control);
        let player_anim = AsepriteAnimation::new(player_ase.info(), anim_info.tag_name);
        let mut transform = *transform;
        transform.translation.z += 1.;
        transform.translation.y += 8.;
        commands
            .entity(entity)
            .insert(AsepriteBundle {
                texture_atlas: player_ase.atlas().clone_weak(),
                sprite: TextureAtlasSprite {
                    flip_x: anim_info.flip_x,
                    ..TextureAtlasSprite::new(player_anim.current_frame())
                },
                aseprite: player_ase_handle,
                animation: player_anim,
                transform,
                ..default()
            })
            .insert(grid_coords_from_instance(ldtk_instance, tilemap_size));
    }
}

#[allow(clippy::type_complexity)]
fn switch_player_animation(
    mut player_query: Query<
        (
            &mut AsepriteAnimation,
            &mut TextureAtlasSprite,
            &ColorControl,
            &MovementState,
        ),
        (
            With<Player>,
            Or<(Changed<ColorControl>, Changed<MovementState>)>,
        ),
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
