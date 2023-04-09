use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::FieldValue::Enum, EntityInstance, GridCoords, LdtkEntity};
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation};

use crate::loading::SpriteAssets;

use super::{
    tiles::{Laser, LaserType},
    RgbEntityAsepriteBundle,
};

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LaserSprite(LaserType);

#[derive(Bundle, LdtkEntity)]
pub struct LaserBundle {
    laser_sprite: LaserSprite,

    #[from_entity_instance]
    entity_instance: EntityInstance,

    #[grid_coords]
    grid_coords: GridCoords,
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
        if let Some((aseprite_bundle, y_offset)) =
            laser_aseprite_bundle(entity_instance, &mut laser_sprite, &sprites, &aseprites)
        {
            transform.translation.y += y_offset;
            commands.entity(entity).insert(aseprite_bundle);
        }
    }
}

fn laser_aseprite_bundle(
    entity_instance: &EntityInstance,
    laser_sprite: &mut LaserSprite,
    sprites: &SpriteAssets,
    aseprites: &Assets<Aseprite>,
) -> Option<(RgbEntityAsepriteBundle, f32)> {
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
    let y_offset = match axis_value.as_str() {
        "Horizontal" => Some(0.),
        "Vertical" => Some(8.),
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
        "Red" => Some(LaserSprite(LaserType::Red)),
        "Green" => Some(LaserSprite(LaserType::Green)),
        "Blue" => Some(LaserSprite(LaserType::Blue)),
        _ => None,
    }?;
    *laser_sprite = actual_laser_sprite;
    let aseprite = aseprites.get(&sprite)?;
    let laser_animation = AsepriteAnimation::new(aseprite.info(), tag_name);
    (
        RgbEntityAsepriteBundle {
            texture_atlas: aseprite.atlas().clone_weak(),
            sprite: TextureAtlasSprite::new(laser_animation.current_frame()),
            aseprite: sprite,
            animation: laser_animation,
        },
        y_offset,
    )
        .into()
}

#[allow(clippy::type_complexity)]
pub fn laser_visibility(
    mut laser_sprite_q: Query<(&mut Visibility, &GridCoords), With<LaserSprite>>,
    laser_q: Query<(&Laser, &GridCoords), Changed<Laser>>,
) {
    for (mut laser_visibility, sprite_coords) in laser_sprite_q.iter_mut() {
        let visible = laser_q
            .iter()
            .find(|(_, laser_coords)| **laser_coords == *sprite_coords)
            .map(|(laser, _)| !laser.is_open);
        if let Some(visible) = visible {
            *laser_visibility = if visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        };
    }
}
