use bevy::prelude::*;

use crate::actions::Actions;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorControl {
    Red,
    #[default]
    Blue,
}

impl ColorControl {
    fn switch(&mut self) {
        match *self {
            ColorControl::Red => {
                *self = ColorControl::Blue;
            }
            ColorControl::Blue => {
                *self = ColorControl::Red;
            }
        }
    }
}

pub fn switch_red_or_blue_door_ignore_on_color_control_interaction(
    mut color_control_q: Query<(&mut ColorControl, &Interaction), Changed<Interaction>>,
) {
    for (mut color_control, interaction) in color_control_q.iter_mut() {
        if *interaction == Interaction::Clicked {
            color_control.switch();
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

pub fn change_button_text_on_color_control_change(
    mut color_control_q: Query<(&ColorControl, &Children), Changed<ColorControl>>,
    mut text_q: Query<&mut Text>,
) {
    for (color_control, children) in color_control_q.iter_mut() {
        let text_str = match color_control {
            ColorControl::Red => "Red",
            ColorControl::Blue => "Blue",
        };
        for child in children.iter() {
            if let Ok(mut text) = text_q.get_mut(*child) {
                text.sections[0].value = text_str.to_string();
            }
        }
    }
}
