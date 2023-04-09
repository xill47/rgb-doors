use bevy::prelude::{ChildBuilder, *};

use crate::player::color_control::ColorControl;

use super::buttons_styles::BUTTON_DEFAULT_BG_COLOR;

#[derive(Component)]
pub struct ColorControlView;

pub fn add_color_control(parent: &mut ChildBuilder, text_style: &TextStyle) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.), Val::Px(50.)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BUTTON_DEFAULT_BG_COLOR.into(),
            ..default()
        })
        .insert(ColorControlView)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    color: Color::rgb_u8(21, 18, 23),
                    ..text_style.clone()
                },
            ));
        });
}

pub fn switch_red_or_blue_door_ignore_on_color_control_interaction(
    color_control_view_q: Query<&Interaction, (Changed<Interaction>, With<ColorControlView>)>,
    mut color_control_q: Query<&mut ColorControl>,
) {
    for interaction in color_control_view_q.iter() {
        if *interaction == Interaction::Clicked {
            for mut color_control in color_control_q.iter_mut() {
                color_control.switch();
            }
        }
    }
}

pub fn change_button_text_on_color_control_change(
    color_control_q: Query<&ColorControl, Changed<ColorControl>>,
    color_control_view_q: Query<&Children, With<ColorControlView>>,
    mut text_q: Query<&mut Text>,
) {
    for color_control in color_control_q.iter() {
        let text_str = match color_control {
            ColorControl::Red => "Red",
            ColorControl::Blue => "Blue",
        };
        for child in color_control_view_q.iter().flatten() {
            if let Ok(mut text) = text_q.get_mut(*child) {
                text.sections[0].value = text_str.to_string();
            }
        }
    }
}
