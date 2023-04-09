use bevy::prelude::*;

use crate::levels::{
    panel::PressurePlate,
    tiles::{Laser, LaserType},
};

use super::color_control::ColorControl;

pub fn open_lasers(
    color_control_q: Query<&ColorControl>,
    pressure_plates_q: Query<&PressurePlate>,
    mut lasers_q: Query<&mut Laser>,
) {
    for mut laser in lasers_q.iter_mut() {
        laser.is_open = is_open(laser.laser_type, &pressure_plates_q, &color_control_q);
    }
}

fn is_open(
    laser_type: LaserType,
    pressure_plates_q: &Query<&PressurePlate>,
    color_control_q: &Query<&ColorControl>,
) -> bool {
    for color_control in color_control_q.iter() {
        if laser_type == color_control.as_laser_type() {
            return true;
        }
    }
    for panel in pressure_plates_q.iter() {
        if panel.opens_laser == Some(laser_type) && panel.is_active() {
            return true;
        }
    }
    false
}
