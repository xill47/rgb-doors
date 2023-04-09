use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation};

use crate::{animation_finished, levels::tiles::Laser, loading::SpriteAssets};

use super::{color_control::ColorControl, Player};

pub struct Death;

#[derive(Component)]
pub struct Dying;

#[allow(clippy::type_complexity)]
pub fn die_on_tile_with_door(
    mut commands: Commands,
    player_q: Query<
        (Entity, &GridCoords),
        (
            Or<(Changed<ColorControl>, Changed<GridCoords>)>,
            With<Player>,
        ),
    >,
    door_q: Query<(&GridCoords, &Laser)>,
) {
    for (entity, player_coords) in player_q.iter() {
        for (door_coords, door) in door_q.iter() {
            if player_coords == door_coords && !door.is_open {
                commands.entity(entity).insert(Dying);
            }
        }
    }
}

#[derive(Default)]
pub enum DyingState {
    #[default]
    None,
    Animation,
    Dead,
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn play_death_animation(
    mut dying_player_q: Query<
        (&mut AsepriteAnimation, &mut TextureAtlasSprite),
        (With<Player>, With<Dying>),
    >,
    alive_player_q: Query<Entity, (With<Player>, Without<Dying>)>,
    time: Res<Time>,
    mut dying_state: Local<DyingState>,
    mut death: EventWriter<Death>,
    aseprites: Res<Assets<Aseprite>>,
    sprites: Res<SpriteAssets>,
) {
    match *dying_state {
        DyingState::None => {
            for (mut animation, mut sprite) in dying_player_q.iter_mut() {
                let player_ase_handle = sprites.player.clone_weak();
                let player_ase = aseprites.get(&player_ase_handle).unwrap();
                let next_animation = AsepriteAnimation::new(player_ase.info(), "death");
                *animation = next_animation;
                *sprite = TextureAtlasSprite::new(animation.current_frame());
                *dying_state = DyingState::Animation;
            }
        }
        DyingState::Animation => {
            for (animation, _) in dying_player_q.iter_mut() {
                if animation_finished(&animation, &time, &sprites.player, &aseprites)
                    .unwrap_or(true)
                {
                    info!("Sending death signal");
                    death.send(Death);
                    *dying_state = DyingState::Dead;
                }
            }
        }
        DyingState::Dead => {
            for _ in alive_player_q.iter() {
                *dying_state = DyingState::None;
            }
        }
    }
}
