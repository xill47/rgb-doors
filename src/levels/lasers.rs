use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::FieldValue::Enum, EntityInstance, LdtkEntity};
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation};

use crate::{
    loading::SpriteAssets,
    player::{color_control::ColorControl, ignore_doors::IgnoreDoors, Player},
};

use super::{tiles::Door, RgbEntityAsepriteBundle};

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LaserSprite(Door);

#[derive(Bundle, LdtkEntity)]
pub struct LaserBundle {
    laser_sprite: LaserSprite,

    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[allow(clippy::type_complexity)]
pub fn spawn_lasers(
    mut commands: Commands,
    mut laser_query: Query<
        (Entity, &EntityInstance, &mut LaserSprite, &mut Transform),
        (With<LaserSprite>, Without<AsepriteAnimation>),
    >,
    sprites: Res<SpriteAssets>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for (entity, entity_instance, mut laser_sprite, mut transform) in laser_query.iter_mut() {
        // get aseprite type by entity instance enum
        if let Some(aseprite_bundle) =
            laser_aseprite_bundle(entity_instance, &mut laser_sprite, &sprites, &aseprites)
        {
            transform.translation.y += 8.0;
            commands.entity(entity).insert(aseprite_bundle);
        }
    }
}


fn laser_aseprite_bundle(
    entity_instance: &EntityInstance,
    laser_sprite: &mut LaserSprite,
    sprites: &SpriteAssets,
    aseprites: &Assets<Aseprite>,
) -> Option<RgbEntityAsepriteBundle> {
    let axis_field = entity_instance
        .field_instances
        .iter()
        .find(|field_instance| field_instance.identifier == "Axis")?;
    let axis_value = match &axis_field.value {
        Enum(value) => value.to_owned(),
        _ => None,
    }?;
    let sprite = match axis_value.as_str() {
        "Horizontal" => sprites.h_lasers.clone_weak().into(),
        "Vertical" => sprites.v_lasers.clone_weak().into(),
        _ => None,
    }?;
    let color_field = entity_instance
        .field_instances
        .iter()
        .find(|field_instance| field_instance.identifier == "Color")?;
    let color_value = match &color_field.value {
        Enum(value) => value.to_owned(),
        _ => None,
    }?;
    let tag_name = match color_value.as_str() {
        "Red" => "red_laser_idle".into(),
        "Green" => "green_laser_idle".into(),
        "Blue" => "blue_laser_idle".into(),
        _ => None,
    }?;
    let actual_laser_sprite = match color_value.as_str() {
        "Red" => Some(LaserSprite(Door::Red)),
        "Green" => Some(LaserSprite(Door::Green)),
        "Blue" => Some(LaserSprite(Door::Blue)),
        _ => None,
    }?;
    *laser_sprite = actual_laser_sprite;
    let aseprite = aseprites.get(&sprite)?;
    let laser_animation = AsepriteAnimation::new(aseprite.info(), tag_name);
    RgbEntityAsepriteBundle {
        texture_atlas: aseprite.atlas().clone_weak(),
        sprite: TextureAtlasSprite::new(laser_animation.current_frame()),
        aseprite: sprite,
        animation: laser_animation,
    }
    .into()
}

#[allow(clippy::type_complexity)]
pub fn hide_lasers(
    mut laser_query: Query<(&mut Visibility, &LaserSprite), With<LaserSprite>>,
    player_query: Query<
        (&ColorControl, &IgnoreDoors),
        (
            With<Player>,
            Or<(Changed<ColorControl>, Changed<IgnoreDoors>, Added<Player>)>,
        ),
    >,
) {
    for (mut laser_visibility, laser_sprite) in laser_query.iter_mut() {
        let visible = player_query
            .iter()
            .fold(None, |visible, (color_control, ignore_doors)| {
                let ignores_door = ignore_doors.ignores_door(color_control, &laser_sprite.0);
                if let Some(visible) = visible {
                    Some(visible && !ignores_door)
                } else {
                    Some(!ignores_door)
                }
            });
        if let Some(visible) = visible {
            *laser_visibility = if visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        };
    }
}
