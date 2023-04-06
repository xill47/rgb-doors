pub mod forbid_movement;
pub mod ignore_doors;

use crate::levels::tiles::*;
use crate::loading::SpriteAssets;
use crate::GameState;
use crate::{actions::Actions, grid_coords_from_instance};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::TilemapTileSize;
use bevy_ecs_tilemap::{prelude::TilemapSize, tiles::TileStorage};
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation, AsepriteBundle};

use self::forbid_movement::ForbiddenMovement;
use self::ignore_doors::*;

pub struct PlayerPlugin;

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[from_entity_instance]
    entity_instance: EntityInstance,

    player: Player,
    ignore_doors: IgnoreDoors,
    forbidden_movement: ForbiddenMovement,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(
                (
                    spawn_player_sprite,
                    move_player_on_grid,
                    ignore_doors_on_color_control_change,
                    ignore_doors_on_panel_press,
                    set_initial_color_control,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[allow(clippy::type_complexity)]
fn spawn_player_sprite(
    mut commands: Commands,
    player_q: Query<
        (Entity, &Transform, &EntityInstance),
        (With<Player>, Without<AsepriteAnimation>),
    >,
    tilemap_q: Query<&TilemapSize>,
    sprites: Res<SpriteAssets>,
    aseprites: Res<Assets<Aseprite>>,
) {
    let Some(tilemap_size) = tilemap_q.iter().next() else { return;};
    for (entity, transform, ldtk_instance) in player_q.iter() {
        let player_ase_handle = sprites.player.clone_weak();
        let player_ase = aseprites.get(&player_ase_handle).unwrap();
        let player_anim = AsepriteAnimation::new(player_ase.info(), "blue_idle");
        let mut transform = *transform;
        transform.translation.z += 1.;
        commands
            .entity(entity)
            .insert(AsepriteBundle {
                texture_atlas: player_ase.atlas().clone_weak(),
                sprite: TextureAtlasSprite::new(player_anim.current_frame()),
                aseprite: player_ase_handle,
                animation: player_anim,
                transform,
                ..default()
            })
            .insert(grid_coords_from_instance(ldtk_instance, tilemap_size));
    }
}

#[allow(clippy::type_complexity)]
fn move_player_on_grid(
    mut actions: EventReader<Actions>,
    mut player_query: Query<
        (
            &mut GridCoords,
            &mut Transform,
            Option<&IgnoreDoors>,
            Option<&ForbiddenMovement>,
        ),
        With<Player>,
    >,
    tile_storage_q: Query<(&TileStorage, &Name)>,
    tile_size_q: Query<&TilemapTileSize>,
    tiles_q: Query<(Option<&Wall>, Option<&Floor>, Option<&Door>)>,
) {
    let Some(int_grid_tiles) = tile_storage_q
        .iter()
        .find(|(_tile, name)| name.as_str() == "IntGrid")
        .map(|(tile, _name)| tile) else { return;};
    let Some(tile_size) = tile_size_q.iter().next() else { return;};
    for actions in actions.iter() {
        if let Some(player_movement) = &actions.player_movement {
            let player_movement_vec = player_movement.movement();

            for (mut grid_coords, mut transform, ignore_doors, forbidden_movement) in
                player_query.iter_mut()
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

                if wall.is_some() {
                    continue;
                } else if let Some(door) = door {
                    if let Some(ignore_doors) = ignore_doors {
                        if ignore_doors.ignores_door(door) {
                            move_on_grid(
                                &mut grid_coords,
                                target_tile_pos,
                                &mut transform,
                                player_movement_vec,
                                tile_size,
                            );
                        }
                    }
                } else if floor.is_some() {
                    move_on_grid(
                        &mut grid_coords,
                        target_tile_pos,
                        &mut transform,
                        player_movement_vec,
                        tile_size,
                    );
                }
            }
        }
    }
}

fn move_on_grid(
    grid_coords: &mut Mut<GridCoords>,
    target_tile_pos: GridCoords,
    transform: &mut Mut<Transform>,
    player_movement: IVec2,
    tile_size: &TilemapTileSize,
) {
    **grid_coords = target_tile_pos;
    transform.translation.x += player_movement.x as f32 * tile_size.x;
    transform.translation.y += player_movement.y as f32 * tile_size.y;
}
