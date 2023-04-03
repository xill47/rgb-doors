use bevy::prelude::*;

use crate::levels::tiles::Door;

#[derive(Component, Default, Clone, Debug)]
pub struct IgnoreDoors {
    ignore_green: bool,
    ignore_red: bool,
    ignore_blue: bool,
    ignore_green_permanent: bool,
    ignore_red_permanent: bool,
    ignore_blue_permanent: bool,
}

pub struct SetIgnoreDoor {
    pub door: Door,
    pub permanent: bool,
}

impl IgnoreDoors {
    pub fn ignores_door(&self, door: &Door) -> bool {
        match door {
            Door::Green => self.ignore_green || self.ignore_green_permanent,
            Door::Red => self.ignore_red || self.ignore_red_permanent,
            Door::Blue => self.ignore_blue || self.ignore_blue_permanent,
        }
    }
}

pub fn apply_ignore_door(
    mut ignore_doors: Query<&mut IgnoreDoors>,
    mut add_ignore_door: EventReader<SetIgnoreDoor>,
) {
    for add_ignore_door in add_ignore_door.iter() {
        for mut ignore_doors in ignore_doors.iter_mut() {
            ignore_doors.ignore_green = false;
            ignore_doors.ignore_red = false;
            ignore_doors.ignore_blue = false;
            match add_ignore_door.door {
                Door::Green => ignore_doors.ignore_green = true,
                Door::Red => ignore_doors.ignore_red = true,
                Door::Blue => ignore_doors.ignore_blue = true,
            }
        }
    }
}
