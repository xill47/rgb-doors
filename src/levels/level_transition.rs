use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::FieldValue, *};
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation};

use crate::{loading::SpriteAssets, ui::notifications::Notification};

use super::RgbEntityAsepriteBundle;

#[derive(Component, Debug, Default)]
pub struct Finish {
    next_level: Option<LevelSelection>,
    message: Option<Notification>,
}

#[derive(Bundle, LdtkEntity)]
pub struct FinishBundle {
    finish: Finish,

    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[allow(clippy::type_complexity)]
pub fn spawn_finish(
    mut commands: Commands,
    mut finish_query: Query<
        (Entity, &EntityInstance, &mut Finish, &mut Transform),
        Without<AsepriteAnimation>,
    >,
    sprites: Res<SpriteAssets>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for (entity, entity_instance, mut finish, mut transform) in finish_query.iter_mut() {
        finish.next_level = entity_instance
            .field_instances
            .iter()
            .find(|field_instance| field_instance.identifier == "NextLevel")
            .and_then(|field_instance| match &field_instance.value {
                FieldValue::String(Some(value)) => Some(LevelSelection::Iid(value.to_owned())),
                _ => None,
            });
        finish.message = entity_instance
            .field_instances
            .iter()
            .find(|field_instance| field_instance.identifier == "Message")
            .and_then(|field_instance| match &field_instance.value {
                FieldValue::String(Some(value)) => Some(Notification::new(value.to_owned())),
                _ => None,
            });
        transform.translation.y += 8.0;
        if let Some(aseprite_bundle) = finish_sprite(&sprites, &aseprites) {
            commands.entity(entity).insert(aseprite_bundle);
        }
    }
}

fn finish_sprite(
    sprites: &SpriteAssets,
    aseprites: &Assets<Aseprite>,
) -> Option<RgbEntityAsepriteBundle> {
    let lift_ase_handle = sprites.lift.cast_weak();
    let lift_ase = aseprites.get(&lift_ase_handle)?;
    let mut lift_anim = AsepriteAnimation::new(lift_ase.info(), "left_door_close");
    lift_anim.pause();
    Some(RgbEntityAsepriteBundle {
        aseprite: lift_ase_handle.clone_weak(),
        texture_atlas: lift_ase.atlas().clone_weak(),
        sprite: TextureAtlasSprite::new(lift_anim.current_frame()),
        animation: lift_anim,
    })
}
