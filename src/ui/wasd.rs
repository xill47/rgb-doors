use std::collections::HashSet;

use bevy::prelude::*;

use crate::{
    actions::{Actions, MovementDirection},
    player::movement_effects::{
        MovementSideEffects,
        SideEffect::{DisabledMovement, MultiMove, None},
    },
};

use super::bg_color_tween::BackgroundColorTween;

#[derive(Component, PartialEq, Eq, Hash, Clone, Debug)]
pub struct Wasd {
    player_movement: MovementDirection,
    forbidden: bool,
}

impl From<MovementDirection> for Wasd {
    fn from(player_movement: MovementDirection) -> Self {
        Self {
            player_movement,
            forbidden: false,
        }
    }
}

const WASD_DEFAULT_COLOR: Color = Color::rgb(0.6, 0.6, 0.6);
const WASD_PRESSED_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const WASD_FORBID_COLOR: Color = Color::rgb(0.6, 0.2, 0.2);
const WASD_MULTIMOVE_COLOR: Color = Color::rgb(0.4, 0.4, 0.2);

pub fn add_wasd(
    parent: &mut ChildBuilder,
    node_style: &Style,
    text_style: &TextStyle,
    str: &str,
    movement: MovementDirection,
) {
    let wasd = Wasd::from(movement);
    let wasd_bg_color = WASD_DEFAULT_COLOR;
    parent
        .spawn(NodeBundle {
            style: node_style.clone(),
            background_color: wasd_bg_color.into(),
            ..default()
        })
        .insert(wasd)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(str, text_style.clone()));
        });
}

pub fn style_wasd_on_player_movement_action(
    mut commands: Commands,
    mut wasd_query: Query<(&BackgroundColor, &Wasd, Entity)>,
    mut actions: EventReader<Actions>,
) {
    let wasd_actions = actions
        .iter()
        .filter_map(|action| action.player_movement.map(Wasd::from))
        .collect::<HashSet<_>>();
    for (background_color, wasd, entity) in wasd_query.iter_mut() {
        if wasd_actions.contains(wasd) && !wasd.forbidden {
            commands.entity(entity).insert(BackgroundColorTween {
                start_color: background_color.0,
                end_color: WASD_PRESSED_COLOR,
                after_color: WASD_DEFAULT_COLOR,
                duration: 0.1,
                elapsed: 0.0,
            });
        }
    }
}

pub fn set_wasd_forbidden(
    mut commands: Commands,
    mut wasd_query: Query<(Entity, &mut BackgroundColor, &mut Wasd, &Children)>,
    mut text_q: Query<&mut Text>,
    side_effects_q: Query<&MovementSideEffects, Changed<MovementSideEffects>>,
) {
    for side_effects in side_effects_q.iter() {
        for (entity, mut background_color, mut wasd, children) in wasd_query.iter_mut() {
            let side_effect = side_effects.get(wasd.player_movement);
            match side_effect {
                None => {
                    wasd.forbidden = false;
                    background_color.0 = WASD_DEFAULT_COLOR;
                    for child in children.iter() {
                        if let Ok(mut text) = text_q.get_mut(*child) {
                            text.sections[0] = TextSection {
                                value: match wasd.player_movement {
                                    MovementDirection::Up => "W".to_owned(),
                                    MovementDirection::Down => "S".to_owned(),
                                    MovementDirection::Left => "A".to_owned(),
                                    MovementDirection::Right => "D".to_owned(),
                                },
                                style: text.sections[0].style.clone(),
                            };
                        }
                    }
                }
                DisabledMovement => {
                    wasd.forbidden = true;
                    commands.entity(entity).insert(BackgroundColorTween {
                        start_color: background_color.0,
                        end_color: WASD_FORBID_COLOR,
                        after_color: WASD_FORBID_COLOR,
                        duration: 0.7,
                        elapsed: 0.0,
                    });
                    for child in children.iter() {
                        if let Ok(mut text) = text_q.get_mut(*child) {
                            text.sections[0] = TextSection {
                                value: match wasd.player_movement {
                                    MovementDirection::Up => "W".to_owned(),
                                    MovementDirection::Down => "S".to_owned(),
                                    MovementDirection::Left => "A".to_owned(),
                                    MovementDirection::Right => "D".to_owned(),
                                },
                                style: text.sections[0].style.clone(),
                            };
                        }
                    }
                }
                MultiMove(count) => {
                    wasd.forbidden = false;
                    commands.entity(entity).insert(BackgroundColorTween {
                        start_color: background_color.0,
                        end_color: WASD_MULTIMOVE_COLOR,
                        after_color: WASD_MULTIMOVE_COLOR,
                        duration: 0.7,
                        elapsed: 0.0,
                    });
                    for child in children.iter() {
                        if let Ok(mut text) = text_q.get_mut(*child) {
                            text.sections[0] = TextSection {
                                value: count.to_string(),
                                style: text.sections[0].style.clone(),
                            };
                        }
                    }
                }
            }
        }
    }
}
