use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::*;
use bevy_ecs_tilemap::{prelude::TilemapTileSize, tiles::TileStorage};

use crate::{
    actions::{Actions, MovementDirection},
    levels::tiles::{Door, Floor, Wall},
};

use super::{
    color_control::ColorControl, death::Dying, forbid_movement::ForbiddenMovement,
    ignore_doors::IgnoreDoors, Player,
};

#[derive(Component, Default, Clone, Copy)]
pub enum MovementState {
    #[default]
    Idle,
    Moving(MovementDirection),
}

pub struct AnimationInfo {
    pub tag_name: &'static str,
    pub flip_x: bool,
}

impl MovementState {
    pub fn anim_info(&self, color_control: &ColorControl) -> AnimationInfo {
        let tag_name = match self {
            MovementState::Idle => match color_control {
                ColorControl::Red => "red_idle",
                ColorControl::Blue => "blue_idle",
            },
            MovementState::Moving(direction) => match direction {
                MovementDirection::Up => "walk_up",
                MovementDirection::Down => match color_control {
                    ColorControl::Red => "red_walk_down",
                    ColorControl::Blue => "blue_walk_down",
                },
                MovementDirection::Left => match color_control {
                    ColorControl::Red => "red_walk_right",
                    ColorControl::Blue => "blue_walk_right",
                },
                MovementDirection::Right => match color_control {
                    ColorControl::Red => "red_walk_right",
                    ColorControl::Blue => "blue_walk_right",
                },
            },
        };
        let flip_x = match self {
            MovementState::Idle => false,
            MovementState::Moving(direction) => match direction {
                MovementDirection::Up => false,
                MovementDirection::Down => false,
                MovementDirection::Left => true,
                MovementDirection::Right => false,
            },
        };
        AnimationInfo { tag_name, flip_x }
    }

    pub fn is_moving(&self) -> bool {
        match self {
            MovementState::Idle => false,
            MovementState::Moving(_) => true,
        }
    }
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct TweenTranslation {
    pub start: Vec3,
    pub end: Vec3,
    pub duration: Duration,
    pub elapsed: Duration,
}

#[allow(clippy::type_complexity)]
pub fn move_player_on_grid(
    mut actions: EventReader<Actions>,
    mut player_query: Query<
        (
            &mut GridCoords,
            Option<&IgnoreDoors>,
            Option<&ColorControl>,
            Option<&ForbiddenMovement>,
            Option<&mut MovementState>,
        ),
        (With<Player>, Without<Dying>),
    >,
    tile_storage_q: Query<(&TileStorage, &Name)>,
    tiles_q: Query<(Option<&Wall>, Option<&Floor>, Option<&Door>)>,
) {
    let Some(int_grid_tiles) = tile_storage_q
        .iter()
        .find(|(_tile, name)| name.as_str() == "IntGrid")
        .map(|(tile, _name)| tile) else { return;};
    for actions in actions.iter() {
        if let Some(player_movement) = &actions.player_movement {
            let player_movement_vec = player_movement.as_ivec2();
            for (
                mut grid_coords,
                ignore_doors,
                color_control,
                forbidden_movement,
                movement_state,
            ) in player_query.iter_mut()
            {
                if let Some(forbidden_movement) = forbidden_movement {
                    if forbidden_movement.forbidden.contains(player_movement) {
                        continue;
                    }
                }

                let target_tile_pos = GridCoords {
                    x: grid_coords.x + player_movement_vec.x,
                    y: grid_coords.y + player_movement_vec.y,
                };

                let Some(tile_entity) = int_grid_tiles.get(&target_tile_pos.into()) else { continue };
                let Ok((wall, floor, door)) = tiles_q.get(tile_entity) else { continue };
                let is_moving = movement_state
                    .as_ref()
                    .map(|movement_state| movement_state.is_moving())
                    .unwrap_or(false);

                if wall.is_none()
                    && !is_moving
                    && ignores_door(ignore_doors, color_control, door).unwrap_or(floor.is_some())
                {
                    *grid_coords = target_tile_pos;
                    if let Some(mut movement_state) = movement_state {
                        *movement_state = MovementState::Moving(*player_movement);
                    }
                }
            }
        }
    }
}

fn ignores_door(
    ignore_doors: Option<&IgnoreDoors>,
    color_control: Option<&ColorControl>,
    door: Option<&Door>,
) -> Option<bool> {
    Some(ignore_doors?.ignores_door(color_control?, door?))
}

#[allow(clippy::type_complexity)]
pub fn change_transform_based_on_grid(
    mut commands: Commands,
    player_query: Query<
        (Entity, &Transform, &MovementState),
        (Without<TweenTranslation>, Changed<MovementState>),
    >,
    tilemap_size_q: Query<&TilemapTileSize>,
) {
    let Some(tilemap_size) = tilemap_size_q.iter().next() else { return; };
    for (entity, transform, movement_state) in player_query.iter() {
        if let MovementState::Moving(player_movement) = movement_state {
            let movement_vec = player_movement.as_ivec2();
            let target_pos = transform.translation
                + Vec3::new(
                    movement_vec.x as f32 * tilemap_size.x,
                    movement_vec.y as f32 * tilemap_size.y,
                    0.0,
                );
            commands.entity(entity).insert(TweenTranslation {
                start: transform.translation,
                end: target_pos,
                duration: Duration::from_secs_f32(0.2),
                elapsed: Duration::default(),
            });
        }
    }
}

pub fn tween_translations(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform, &mut TweenTranslation)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut tween_translation) in player_query.iter_mut() {
        tween_translation.elapsed += time.delta();
        let t = tween_translation.elapsed.as_secs_f32() / tween_translation.duration.as_secs_f32();
        if t >= 1.0 {
            transform.translation = tween_translation.end;
            commands.entity(entity).remove::<TweenTranslation>();
        } else {
            transform.translation = tween_translation.start.lerp(tween_translation.end, t);
        }
    }
}

pub fn return_to_idle(
    mut removed: RemovedComponents<TweenTranslation>,
    mut player_query: Query<&mut MovementState>,
) {
    for entity in removed.iter() {
        if let Ok(mut movement_state) = player_query.get_mut(entity) {
            *movement_state = MovementState::Idle;
        }
    }
}
