use bevy::prelude::{Changed, EventWriter, Query};
use bevy_ecs_ldtk::GridCoords;

use crate::levels::tiles::Door;

use super::{color_control::ColorControl, ignore_doors::IgnoreDoors};

pub struct Death;

pub fn die_on_tile_with_door(
    mut death_event: EventWriter<Death>,
    player_q: Query<(&GridCoords, &ColorControl, &IgnoreDoors), Changed<ColorControl>>,
    door_q: Query<(&GridCoords, &Door)>,
) {
    for (player_coords, color_control, ignore_doors) in player_q.iter() {
        for (door_coords, door) in door_q.iter() {
            if player_coords == door_coords && !ignore_doors.ignores_door(color_control, door) {
                death_event.send(Death);
            }
        }
    }
}
