use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::*;
use bevy_ecs_tilemap::{prelude::TilemapTileSize, tiles::TileStorage};

use crate::{
    actions::{Actions, MovementDirection},
    levels::tiles::Wall,
};

use super::{
    color_control::ColorControl, death::Dying, movement_effects::MovementSideEffects, Player,
};

#[derive(Component, Default, Clone, Copy)]
pub enum MovementState {
    #[default]
    Idle,
    Moving(MovementDirection),
    MultiMoving {
        direction: MovementDirection,
        left: u32,
    },
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
            MovementState::Moving(direction) | MovementState::MultiMoving { direction, .. } => {
                match direction {
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
                }
            }
        };
        let flip_x = match self {
            MovementState::Idle => false,
            MovementState::Moving(direction) | MovementState::MultiMoving { direction, .. } => {
                match direction {
                    MovementDirection::Up => false,
                    MovementDirection::Down => false,
                    MovementDirection::Left => true,
                    MovementDirection::Right => false,
                }
            }
        };
        AnimationInfo { tag_name, flip_x }
    }

    pub fn is_moving(&self) -> bool {
        match self {
            MovementState::Idle => false,
            MovementState::Moving(_) => true,
            MovementState::MultiMoving { .. } => true,
        }
    }

    pub fn apply_movement(
        &mut self,
        coords: &mut GridCoords,
        tile_storage_q: &Query<(&TileStorage, &Name)>,
        tiles_q: &Query<Option<&Wall>>,
    ) {
        match self {
            MovementState::Idle => {}
            MovementState::Moving(direction) | MovementState::MultiMoving { direction, .. } => {
                let direction = direction.as_ivec2();
                let next_grid_coords = GridCoords {
                    x: coords.x + direction.x,
                    y: coords.y + direction.y,
                };
                let is_wall = tile_storage_q
                    .iter()
                    .find(|(_, name)| name.as_str() == "IntGrid")
                    .and_then(|(tile_storage, _)| tile_storage.get(&next_grid_coords.into()))
                    .map(|tile| {
                        tiles_q
                            .get(tile)
                            .map(|tile| tile.is_some())
                            .unwrap_or(false)
                    })
                    .unwrap_or(false);

                if is_wall {
                    *self = MovementState::Idle;
                } else {
                    *coords = next_grid_coords;
                }
            }
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
pub fn player_action_to_movement(
    mut actions: EventReader<Actions>,
    mut player_query: Query<
        (&MovementSideEffects, &mut MovementState, &mut GridCoords),
        (With<Player>, Without<Dying>),
    >,
    tile_storage_q: Query<(&TileStorage, &Name)>,
    tiles_q: Query<Option<&Wall>>,
) {
    for actions in actions.iter() {
        if let Some(player_movement) = &actions.player_movement {
            for (side_effects, mut movement_state, mut coords) in player_query.iter_mut() {
                if movement_state.is_moving() {
                    continue;
                }
                let default_movement = MovementState::Moving(*player_movement);
                *movement_state = side_effects
                    .get(*player_movement)
                    .transform_movement_state(default_movement);

                movement_state.apply_movement(&mut coords, &tile_storage_q, &tiles_q);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn change_transform_based_on_grid(
    mut commands: Commands,
    player_query: Query<
        (Entity, &Transform, &GridCoords),
        (Without<TweenTranslation>, Changed<GridCoords>, With<Player>),
    >,
    tilemap_size_q: Query<&TilemapTileSize>,
) {
    let Some(tile_size) = tilemap_size_q.iter().next() else { return; };
    for (entity, transform, grid_coords) in player_query.iter() {
        let target_pos = Vec3::new(
            grid_coords.x as f32 * tile_size.x + tile_size.x / 2.0,
            grid_coords.y as f32 * tile_size.y + tile_size.y / 2.0 + 8.,
            transform.translation.z,
        );
        commands.entity(entity).insert(TweenTranslation {
            start: transform.translation,
            end: target_pos,
            duration: Duration::from_secs_f32(0.2),
            elapsed: Duration::default(),
        });
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

pub fn next_movement_state(
    mut removed: RemovedComponents<TweenTranslation>,
    mut player_query: Query<(&mut MovementState, &mut GridCoords), Without<Dying>>,
    tile_storage_q: Query<(&TileStorage, &Name)>,
    tiles_q: Query<Option<&Wall>>,
) {
    for entity in removed.iter() {
        if let Ok((mut movement_state, mut grid_coords)) = player_query.get_mut(entity) {
            *movement_state = match *movement_state {
                MovementState::Idle => MovementState::Idle,
                MovementState::Moving(_) => MovementState::Idle,
                MovementState::MultiMoving { direction, left } => {
                    if left > 1 {
                        movement_state.apply_movement(&mut grid_coords, &tile_storage_q, &tiles_q);
                    }
                    match left {
                        2.. => MovementState::MultiMoving {
                            direction,
                            left: left - 1,
                        },
                        _ => MovementState::Idle,
                    }
                }
            };
        }
    }
}
