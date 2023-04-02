use crate::actions::Actions;
use crate::loading::SpriteAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
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
            .add_system(move_player.in_set(OnUpdate(GameState::Playing)));
    }
}

#[allow(clippy::type_complexity)]
fn spawn_player_sprite(
    mut commands: Commands,
    transform_q: Query<&Transform, With<Player>>,
    player_q: Query<Entity, (With<Player>, Without<AsepriteAnimation>)>,
    sprites: Res<SpriteAssets>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for entity in player_q.iter() {
        let player_ase_handle = sprites.player.clone_weak();
        let player_ase = aseprites.get(&player_ase_handle).unwrap();
        let player_anim = AsepriteAnimation::new(player_ase.info(), "idle");
        let transform = transform_q.get(entity).expect("Player has no transform");
        commands.entity(entity).insert(AsepriteBundle {
            texture_atlas: player_ase.atlas().clone_weak(),
            sprite: TextureAtlasSprite::new(player_anim.current_frame()),
            aseprite: player_ase_handle,
            animation: player_anim,
            transform: *transform,
            ..default()
        });
    }
}

const TILE_SIZE: f32 = 16.0;

fn move_player(actions: Res<Actions>, mut player_query: Query<&mut Transform, With<Player>>) {
    let Some(player_movement) = actions.player_movement else { return };
    let player_movement = player_movement.movement();

    for mut transform in player_query.iter_mut() {
        transform.translation.x += player_movement.x * TILE_SIZE;
        transform.translation.y += player_movement.y * TILE_SIZE;
    }
}
