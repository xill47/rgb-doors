use std::collections::HashSet;

use bevy::prelude::*;

use crate::{
    actions::{Actions, MovementDirection},
    player::forbid_movement::ForbiddenMovement,
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

const WASD_DEFAULT_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const WASD_PRESSED_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const WASD_FORBID_COLOR: Color = Color::rgb(0.6, 0.2, 0.2);

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
        .insert(wasd.clone())
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(str, text_style.clone()))
                .insert(wasd.clone());
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
    mut wasd_query: Query<(Entity, &mut BackgroundColor, &mut Wasd)>,
    forbidden_query: Query<&ForbiddenMovement, Changed<ForbiddenMovement>>,
) {
    for forbidden in forbidden_query.iter() {
        for (entity, mut background_color, mut wasd) in wasd_query.iter_mut() {
            if forbidden.forbidden.contains(&wasd.player_movement) {
                wasd.forbidden = true;
                commands.entity(entity).insert(BackgroundColorTween {
                    start_color: background_color.0,
                    end_color: WASD_FORBID_COLOR,
                    after_color: WASD_FORBID_COLOR,
                    duration: 0.7,
                    elapsed: 0.0,
                });
            } else {
                wasd.forbidden = false;
                background_color.0 = WASD_DEFAULT_COLOR;
            }
        }
    }
}
