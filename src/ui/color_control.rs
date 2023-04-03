use bevy::prelude::*;

use crate::{levels::tiles::Door, player::ignore_doors::SetIgnoreDoor};

#[derive(Component, Default, Debug)]
pub enum ColorControl {
    Red,
    #[default]
    Blue,
}

pub fn switch_red_or_blue_door_ignore_on_color_control_interaction(
    mut color_control_q: Query<(&mut ColorControl, &Interaction), Changed<Interaction>>,
    mut ignore_door: EventWriter<SetIgnoreDoor>,
) {
    for (mut color_control, interaction) in color_control_q.iter_mut() {
        if *interaction == Interaction::Clicked {
            match color_control.as_ref() {
                ColorControl::Red => {
                    ignore_door.send(SetIgnoreDoor {
                        door: Door::Red,
                        permanent: false,
                    });
                    *color_control = ColorControl::Blue;
                }
                ColorControl::Blue => {
                    ignore_door.send(SetIgnoreDoor {
                        door: Door::Blue,
                        permanent: false,
                    });
                    *color_control = ColorControl::Red;
                }
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
