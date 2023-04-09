use bevy::prelude::*;

use crate::{actions::Actions, levels::tiles::LaserType};

#[derive(Component, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorControl {
    Red,
    #[default]
    Blue,
}

impl ColorControl {
    pub fn switch(&mut self) {
        match *self {
            ColorControl::Red => {
                *self = ColorControl::Blue;
            }
            ColorControl::Blue => {
                *self = ColorControl::Red;
            }
        }
    }

    pub fn as_laser_type(&self) -> LaserType {
        match *self {
            ColorControl::Red => LaserType::Red,
            ColorControl::Blue => LaserType::Blue,
        }
    }
}

pub fn set_color_control_from_action(
    mut actions: EventReader<Actions>,
    mut color_control_q: Query<&mut ColorControl>,
) {
    for action in actions.iter() {
        if action.color_switch.is_some() {
            for mut color_control in color_control_q.iter_mut() {
                color_control.switch();
            }
        }
    }
}
