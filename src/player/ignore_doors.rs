use bevy::prelude::*;

use crate::levels::{panel::Panel, tiles::Door};

use super::color_control::ColorControl;

#[derive(Component, Default, Clone, Debug)]
pub struct IgnoreDoors {
    pressed_door_panels: Vec<Door>,
}

impl IgnoreDoors {
    pub fn ignores_door(&self, color_control: &ColorControl, door: &Door) -> bool {
        self.pressed_door_panels
            .iter()
            .any(|panel_door| panel_door == door)
            || match door {
                Door::Green => false,
                Door::Red => *color_control == ColorControl::Red,
                Door::Blue => *color_control == ColorControl::Blue,
            }
    }
}

pub fn ignore_doors_on_panel_press(
    mut ignore_doors: Query<&mut IgnoreDoors>,
    panel_q: Query<&Panel, Changed<Panel>>,
) {
    for panel in panel_q.iter() {
        for mut ignore_doors in ignore_doors.iter_mut() {
            if panel.is_active() {
                if let Some(door) = panel.opens_door {
                    ignore_doors.pressed_door_panels.push(door);
                }
            }
        }
    }
}
