pub mod bg_color_tween;
pub mod color_control;
pub mod wasd;

use bevy::prelude::*;

use crate::{loading::FontAssets, GameState};

use self::{bg_color_tween::tween_background_color, color_control::*, wasd::*};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_game_ui.in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (
                    style_button_interactions,
                    style_wasd_on_player_movement_action,
                    tween_background_color.after(style_wasd_on_player_movement_action),
                    switch_red_or_blue_door_ignore_on_color_control_interaction,
                    change_button_text_on_color_control_change,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

fn spawn_game_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    let text_style = TextStyle {
        font: font_assets.fira_sans.clone_weak(),
        color: Color::rgb(0.3, 0.3, 0.3),
        ..default()
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
                        .insert(ColorControl::default())
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "",
                                TextStyle {
                                    color: Color::rgb(0.1, 0.9, 0.1),
                                    ..text_style.clone()
                                },
                            ));
                        });
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
                    let wasd_background_color = Color::rgb(0.75, 0.75, 0.75);
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
                            parent
                                .spawn(NodeBundle {
                                    style: wasd_node_style.clone(),
                                    background_color: wasd_background_color.into(),
                                    ..default()
                                })
                                .insert(Wasd::Up)
                                .with_children(|parent| {
                                    parent
                                        .spawn(TextBundle::from_section("W", text_style.clone()))
                                        .insert(Wasd::Up);
                                });
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
                                    parent
                                        .spawn(NodeBundle {
                                            style: wasd_node_style.clone(),
                                            background_color: wasd_background_color.into(),
                                            ..default()
                                        })
                                        .insert(Wasd::Left)
                                        .with_children(|parent| {
                                            parent
                                                .spawn(TextBundle::from_section(
                                                    "A",
                                                    text_style.clone(),
                                                ))
                                                .insert(Wasd::Left);
                                        });
                                    parent
                                        .spawn(NodeBundle {
                                            style: wasd_node_style.clone(),
                                            background_color: wasd_background_color.into(),
                                            ..default()
                                        })
                                        .insert(Wasd::Down)
                                        .with_children(|parent| {
                                            parent
                                                .spawn(TextBundle::from_section(
                                                    "S",
                                                    text_style.clone(),
                                                ))
                                                .insert(Wasd::Down);
                                        });
                                    parent
                                        .spawn(NodeBundle {
                                            style: wasd_node_style.clone(),
                                            background_color: wasd_background_color.into(),
                                            ..default()
                                        })
                                        .insert(Wasd::Right)
                                        .with_children(|parent| {
                                            parent
                                                .spawn(TextBundle::from_section(
                                                    "D",
                                                    text_style.clone(),
                                                ))
                                                .insert(Wasd::Right);
                                        });
                                });
                        });
                });
        });
}

fn style_button_interactions(mut button_query: Query<(&mut BackgroundColor, &Interaction)>) {
    for (mut background_color, interaction) in button_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *background_color = BackgroundColor(Color::rgb(0.25, 0.25, 0.25));
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::rgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::rgb(0.15, 0.15, 0.15));
            }
        }
    }
}
