use std::{cmp::min, time::Duration};

use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance, GridCoords, LdtkEntity};
use bevy_mod_aseprite::Aseprite;

use crate::{
    actions::MovementDirection,
    loading::SpriteAssets,
    player::{
        movement_effects::{MovementSideEffects, SideEffect},
        Player,
    },
    ui::notifications::Notification,
};

use super::tiles::LaserType;

#[derive(Clone, Bundle, LdtkEntity)]
pub struct PanelBundle {
    panel: PressurePlate,

    #[from_entity_instance]
    entity_instance: EntityInstance,

    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Component, Clone, Default, Debug)]
pub struct PressurePlate {
    pub opens_laser: Option<LaserType>,
    forbids_movement: Vec<MovementDirection>,
    multi_movement: Vec<MovementDirection>,
    multi_move_values: Vec<i32>,
    active: bool,
}

impl PressurePlate {
    pub fn is_active(&self) -> bool {
        self.active
    }
}

pub fn setup_panel(
    mut commands: Commands,
    mut panel_q: Query<(Entity, &mut PressurePlate, &EntityInstance, &Transform), Without<Sprite>>,
    sprites: Res<SpriteAssets>,
    aseprites: Res<Assets<Aseprite>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (entity, mut panel, entity_instance, transform) in panel_q.iter_mut() {
        if let Some(door) = entity_instance
            .field_instances
            .iter()
            .find(|field| field.identifier == "Door")
            .and_then(|door| {
                if let FieldValue::Enum(Some(door_type)) = door.value.clone() {
                    Some(door_type)
                } else {
                    None
                }
            })
            .and_then(|door| match door.as_str() {
                "Red" => Some(LaserType::Red),
                "Green" => Some(LaserType::Green),
                "Blue" => Some(LaserType::Blue),
                _ => None,
            })
        {
            panel.opens_laser = Some(door);
        }
        if let Some(forbidden_movement) = entity_instance
            .field_instances
            .iter()
            .find(|field| field.identifier == "Wasd_Disable")
            .and_then(|movement| {
                if let FieldValue::Enums(forbidden_movements) = movement.value.clone() {
                    Some(forbidden_movements)
                } else {
                    None
                }
            })
            .map(|forbidden_movements| {
                forbidden_movements
                    .iter()
                    .flatten()
                    .filter_map(|movement| match movement.as_str() {
                        "W" => Some(MovementDirection::Up),
                        "S" => Some(MovementDirection::Down),
                        "A" => Some(MovementDirection::Left),
                        "D" => Some(MovementDirection::Right),
                        _ => None,
                    })
                    .collect::<Vec<MovementDirection>>()
            })
        {
            panel.forbids_movement = forbidden_movement;
        }

        if let Some(multi_movement) = entity_instance
            .field_instances
            .iter()
            .find(|field| field.identifier == "Wasd_Multi_Move")
            .and_then(|multi_movement| {
                if let FieldValue::Enums(multi_movements) = multi_movement.value.clone() {
                    Some(multi_movements)
                } else {
                    None
                }
            })
            .map(|forbidden_movements| {
                forbidden_movements
                    .iter()
                    .flatten()
                    .filter_map(|movement| match movement.as_str() {
                        "W" => Some(MovementDirection::Up),
                        "S" => Some(MovementDirection::Down),
                        "A" => Some(MovementDirection::Left),
                        "D" => Some(MovementDirection::Right),
                        _ => None,
                    })
                    .collect::<Vec<MovementDirection>>()
            })
        {
            panel.multi_movement = multi_movement;
        }

        if let Some(multi_move_values) = entity_instance
            .field_instances
            .iter()
            .find(|field| field.identifier == "Multi_Move_Values")
            .and_then(|multi_move_values| {
                if let FieldValue::Ints(multi_move_values) = multi_move_values.value.clone() {
                    Some(multi_move_values)
                } else {
                    None
                }
            })
            .map(|multi_move_values| {
                multi_move_values
                    .iter()
                    .flatten()
                    .copied()
                    .collect::<Vec<i32>>()
            })
        {
            panel.multi_move_values = multi_move_values;
        }

        if let Some((atlas, sprite)) =
            sprite_for_panel(&panel, &sprites.plates, &aseprites, &texture_atlases)
        {
            commands.entity(entity).insert(SpriteBundle {
                texture: atlas,
                sprite,
                transform: *transform,
                ..default()
            });
        }
    }
}

fn sprite_for_panel(
    panel: &PressurePlate,
    panel_aseprite: &Handle<Aseprite>,
    aseprites: &Assets<Aseprite>,
    texture_atlases: &Assets<TextureAtlas>,
) -> Option<(Handle<Image>, Sprite)> {
    let Some(panel_aseprite) = aseprites.get(panel_aseprite) else { return None; };
    let Some(atlas) = texture_atlases.get(panel_aseprite.atlas()) else { return None; };
    let Some(door) = panel.opens_laser else { return None; };
    let slice_name = match door {
        LaserType::Red => {
            if panel.is_active() {
                "RedPressed"
            } else {
                "RedUnpressed"
            }
        }
        LaserType::Green => {
            if panel.is_active() {
                "GreenPressed"
            } else {
                "GreenUnpressed"
            }
        }
        LaserType::Blue => {
            if panel.is_active() {
                "BluePressed"
            } else {
                "BlueUnpressed"
            }
        }
    };
    let Some(slice) = panel_aseprite.info().slices.get(slice_name) else { return None; };
    (
        atlas.texture.clone_weak(),
        Sprite {
            rect: Rect {
                min: Vec2::new(slice.position_x as f32, slice.position_y as f32),
                max: Vec2::new(
                    slice.position_x as f32 + slice.width as f32,
                    slice.position_y as f32 + slice.height as f32,
                ),
            }
            .into(),
            ..default()
        },
    )
        .into()
}

#[allow(clippy::type_complexity)]
pub fn step_on_panel(
    mut player_q: Query<
        (&GridCoords, &mut MovementSideEffects),
        (With<Player>, Changed<GridCoords>),
    >,
    mut panel_q: Query<(
        &GridCoords,
        &mut PressurePlate,
        Option<&mut Handle<Image>>,
        Option<&mut Sprite>,
    )>,
    mut notify: EventWriter<Notification>,
    sprites: Res<SpriteAssets>,
    aseprites: Res<Assets<Aseprite>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (panel_coord, mut panel, mut image, mut sprite) in panel_q.iter_mut() {
        for (player_coords, mut forbidden_movement) in player_q.iter_mut() {
            if panel_coord == player_coords {
                notify.send(Notification {
                    text: "You stepped on a panel!".into(),
                    duration: Duration::from_secs(2),
                });
                panel.active = true;
                if let Some((atlas, new_sprite)) =
                    sprite_for_panel(&panel, &sprites.plates, &aseprites, &texture_atlases)
                {
                    if let Some(entity_image) = image.as_mut() {
                        **entity_image = atlas;
                    }
                    if let Some(sprite) = sprite.as_mut() {
                        **sprite = new_sprite;
                    }
                }
                for movement in panel.forbids_movement.iter() {
                    forbidden_movement.set(*movement, SideEffect::DisabledMovement);
                }
                for i in 0..min(panel.multi_movement.len(), panel.multi_move_values.len()) {
                    forbidden_movement.set(
                        panel.multi_movement[i],
                        SideEffect::MultiMove(panel.multi_move_values[i] as u32),
                    );
                }
            }
        }
    }
}
