use bevy::prelude::*;

pub const BUTTON_DEFAULT_BG_COLOR: Color = Color::rgb(131. / 255., 117. / 255., 131. / 255.);
pub const BUTTON_HOVER_BG_COLOR: Color = Color::rgb(155. / 255., 141. / 255., 152. / 255.);
pub const BUTTON_CLICK_BG_COLOR: Color = Color::rgb(232. / 255., 219. / 255., 216. / 255.);

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
