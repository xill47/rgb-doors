use bevy::prelude::*;

pub const BUTTON_DEFAULT_BG_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const BUTTON_HOVER_BG_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const BUTTON_CLICK_BG_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);

pub fn style_button_interactions(
    mut button_query: Query<(&mut BackgroundColor, &Interaction), Changed<Interaction>>,
) {
    for (mut background_color, interaction) in button_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *background_color = BackgroundColor(BUTTON_CLICK_BG_COLOR);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(BUTTON_HOVER_BG_COLOR);
            }
            Interaction::None => {
                *background_color = BackgroundColor(BUTTON_DEFAULT_BG_COLOR);
            }
        }
    }
}
