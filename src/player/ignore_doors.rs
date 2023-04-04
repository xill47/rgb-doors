use bevy::prelude::*;

use crate::{
    levels::{door_panel::Panel, tiles::Door},
    ui::color_control::ColorControl,
};

#[derive(Component, Default, Clone, Debug)]
pub struct IgnoreDoors {
    color_control: ColorControl,
    pressed_panels: Vec<Panel>,
}

impl IgnoreDoors {
    pub fn ignores_door(&self, door: &Door) -> bool {
        self.pressed_panels
            .iter()
            .flat_map(|panel| panel.opens_door)
            .any(|panel_door| panel_door == *door)
            || match door {
                Door::Green => false,
                Door::Red => self.color_control == ColorControl::Red,
                Door::Blue => self.color_control == ColorControl::Blue,
            }
    }
}

pub fn set_initial_color_control(
    mut ignore_doors: Query<&mut IgnoreDoors, Added<IgnoreDoors>>,
    color_control_q: Query<&ColorControl>,
) {
    for color_control in color_control_q.iter() {
        for mut ignore_doors in ignore_doors.iter_mut() {
            ignore_doors.color_control = *color_control;
        }
    }
}

pub fn ignore_doors_on_color_control_change(
    mut ignore_doors: Query<&mut IgnoreDoors>,
    color_control_q: Query<&ColorControl, Changed<ColorControl>>,
) {
    for color_control in color_control_q.iter() {
        for mut ignore_doors in ignore_doors.iter_mut() {
            ignore_doors.color_control = *color_control;
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
                ignore_doors.pressed_panels.push(*panel);
            }
        }
    }
}
