use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance, GridCoords, LdtkEntity};
use bevy_ecs_tilemap::prelude::TilemapSize;

use crate::{grid_coords_from_instance, player::Player};

use super::tiles::Door;

#[derive(Clone, Bundle, LdtkEntity)]
pub struct PanelBundle {
    panel: Panel,

    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Component, Clone, Default, Copy, Debug)]
pub struct Panel {
    pub opens_door: Option<Door>,
    active: bool,
}

impl Panel {
    pub fn is_active(&self) -> bool {
        self.active
    }
}

pub fn setup_panel(
    mut commands: Commands,
    mut panel_q: Query<(Entity, &mut Panel, &EntityInstance), Without<GridCoords>>,
    tilemap_q: Query<&TilemapSize>,
) {
    let Some(tilemap_size) = tilemap_q.iter().next() else { return;};
    for (entity, mut panel, entity_instance) in panel_q.iter_mut() {
        println!("Setting up panel");
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
                "Red" => Some(Door::Red),
                "Green" => Some(Door::Green),
                "Blue" => Some(Door::Blue),
                _ => None,
            })
        {
            panel.opens_door = Some(door);
            commands
                .entity(entity)
                .insert(grid_coords_from_instance(entity_instance, tilemap_size));
        }
    }
}

pub fn step_on_panel(
    player_q: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    mut panel_q: Query<(&GridCoords, &mut Panel)>,
) {
    for (panel_coord, mut panel) in panel_q.iter_mut() {
        for player_coords in player_q.iter() {
            if panel_coord == player_coords {
                println!("Player stepped on panel: {:?}", panel_coord);
                panel.active = true;
            }
        }
    }
}
