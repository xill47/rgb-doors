use bevy::prelude::*;
use bevy_ecs_ldtk::LayerMetadata;

#[derive(Component)]
pub struct IntGridToggler;

pub fn add_debug_button(parent: &mut ChildBuilder, text_style: &TextStyle) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.), Val::Px(50.)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Debug",
                TextStyle {
                    color: Color::rgb(0.5, 0.6, 0.5),
                    ..text_style.clone()
                },
            ));
        })
        .insert(IntGridToggler);
}

pub fn toggle_int_grid(
    toggler_q: Query<&Interaction, (Changed<Interaction>, With<IntGridToggler>)>,
    mut int_grid_q: Query<(&mut Visibility, &Name), With<LayerMetadata>>,
) {
    for interaction in toggler_q.iter() {
        if *interaction == Interaction::Clicked {
            for (mut visibility, name) in int_grid_q.iter_mut() {
                if name.as_str() == "IntGrid" {
                    *visibility = match *visibility {
                        Visibility::Hidden => Visibility::Visible,
                        _ => Visibility::Hidden,
                    }
                }
            }
        }
    }
}
