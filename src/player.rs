use crate::actions::Actions;
use crate::levels::tiles::*;
use crate::loading::SpriteAssets;
use crate::GameState;
use bevy::{ecs::entity, prelude::*, transform};
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::{prelude::TilemapSize, tiles::TileStorage};
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation, AsepriteBundle};

pub struct PlayerPlugin;

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[worldly]
    worldly: Worldly,

    #[from_entity_instance]
    entity_instance: EntityInstance,

    player: Player,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player_sprite.in_set(OnUpdate(GameState::Playing)))
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_system(move_player_on_grid.in_set(OnUpdate(GameState::Playing)));
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
        let player_anim = AsepriteAnimation::new(player_ase.info(), "idle");
        commands
            .entity(entity)
            .insert(AsepriteBundle {
                texture_atlas: player_ase.atlas().clone_weak(),
                sprite: TextureAtlasSprite::new(player_anim.current_frame()),
                aseprite: player_ase_handle,
                animation: player_anim,
                transform: *transform,
                ..default()
            })
            .insert(GridCoords {
                x: ldtk_instance.grid.x,
                y: tilemap_size.y as i32 - ldtk_instance.grid.y - 1,
            });
    }
}

const TILE_SIZE: f32 = 16.0;

fn move_player_on_grid(
    mut actions: EventReader<Actions>,
    mut player_query: Query<(&mut GridCoords, &mut Transform), With<Player>>,
    tile_storage_q: Query<(&TileStorage, &Name)>,
    tiles_q: Query<(Option<&Wall>, Option<&Floor>, Option<&Door>)>,
) {
    let Some(int_grid_tiles) = tile_storage_q
        .iter()
        .find(|(_tile, name)| name.as_str() == "IntGrid")
        .map(|(tile, _name)| tile) else { return;};
    for actions in actions.iter() {
        if let Some(player_movement) = &actions.player_movement {
            let player_movement = player_movement.movement();

            for (mut grid_coords, mut transform) in player_query.iter_mut() {
                let target_tile_pos = GridCoords {
                    x: grid_coords.x + player_movement.x,
                    y: grid_coords.y + player_movement.y,
                };
                let Some(tile_entity) = int_grid_tiles.get(&target_tile_pos.into()) else { continue };
                let Ok((wall, floor, door)) = tiles_q.get(tile_entity) else { continue };

                if wall.is_some() || door.is_some() {
                    continue;
                }

                if floor.is_some() {
                    *grid_coords = target_tile_pos;
                    transform.translation.x += player_movement.x as f32 * TILE_SIZE;
                    transform.translation.y += player_movement.y as f32 * TILE_SIZE;
                }
            }
        }
    }
}
