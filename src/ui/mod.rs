pub mod bg_color_tween;
pub mod buttons_styles;
pub mod color_control_view;
pub mod debug;
pub mod notifications;
pub mod wasd;

use bevy::prelude::*;

use crate::{actions::MovementDirection, loading::FontAssets, GameState};

use self::{
    bg_color_tween::*,
    buttons_styles::style_button_interactions,
    color_control_view::*,
    debug::{add_debug_button, toggle_int_grid},
    notifications::*,
    wasd::*,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Notification>()
            .add_event::<CleanNotificationQueue>()
            .add_system(spawn_game_ui.in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (
                    style_wasd_on_player_movement_action,
                    tween_background_color.after(style_wasd_on_player_movement_action),
                    style_button_interactions,
                    switch_red_or_blue_door_ignore_on_color_control_interaction,
                    update_notifications,
                    display_notifications,
                    toggle_int_grid,
                    clean_notifications.before(display_notifications),
                    change_button_text_on_color_control_change,
                    set_wasd_forbidden,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct LevelScreen;

#[derive(Component)]
pub struct RootUI;

pub fn spawn_game_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    let text_style = TextStyle {
        font: font_assets.fira_sans.clone_weak(),
        color: Color::rgb(0.3, 0.3, 0.3),
        font_size: 24.,
    };
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(80.)),
                        ..default()
                    },
                    ..default()
                })
                .insert(LevelScreen);
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(80.), Val::Percent(20.)),
                        justify_content: JustifyContent::SpaceBetween,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.5)),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(120.), Val::Auto),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            add_color_control(parent, &text_style);
                            #[cfg(debug_assertions)]
                            {
                                add_debug_button(parent, &text_style);
                            }
                        });
                    add_notifications_ui(parent, &text_style);
                    let wasd_size = 42.;
                    let wasd_margin = 8.;
                    let wasd_node_style = Style {
                        size: Size::new(Val::Px(wasd_size), Val::Px(wasd_size)),
                        margin: UiRect::all(Val::Px(wasd_margin)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    };
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Px(wasd_size * 3. + wasd_margin * 5.),
                                    Val::Px(wasd_size * 2. + wasd_margin * 3.),
                                ),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            background_color: Color::rgb(0.25, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            add_wasd(
                                parent,
                                &wasd_node_style,
                                &text_style,
                                "W",
                                MovementDirection::Up,
                            );
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(wasd_size * 3. + wasd_margin * 2.),
                                            Val::Px(wasd_size),
                                        ),
                                        margin: UiRect::all(Val::Px(wasd_margin)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        flex_direction: FlexDirection::Row,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    add_wasd(
                                        parent,
                                        &wasd_node_style,
                                        &text_style,
                                        "A",
                                        MovementDirection::Left,
                                    );
                                    add_wasd(
                                        parent,
                                        &wasd_node_style,
                                        &text_style,
                                        "S",
                                        MovementDirection::Down,
                                    );
                                    add_wasd(
                                        parent,
                                        &wasd_node_style,
                                        &text_style,
                                        "D",
                                        MovementDirection::Right,
                                    );
                                });
                        });
                });
        })
        .insert(RootUI);
}
