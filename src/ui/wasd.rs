use std::collections::HashSet;

use bevy::prelude::*;

use crate::actions::{Actions, PlayerMovement};

use super::bg_color_tween::BackgroundColorTween;

#[derive(Component, PartialEq, Eq, Hash, Clone, Debug)]
pub enum Wasd {
    Up,
    Down,
    Left,
    Right,
}

impl From<PlayerMovement> for Wasd {
    fn from(player_movement: PlayerMovement) -> Self {
        match player_movement {
            PlayerMovement::Up => Wasd::Up,
            PlayerMovement::Down => Wasd::Down,
            PlayerMovement::Left => Wasd::Left,
            PlayerMovement::Right => Wasd::Right,
        }
    }
}

pub fn style_wasd_on_player_movement_action(
    mut commands: Commands,
    mut wasd_query: Query<(&mut BackgroundColor, &Wasd, Entity)>,
    mut actions: EventReader<Actions>,
) {
    let wasd_actions = actions
        .iter()
        .filter_map(|action| { action.player_movement.map(Wasd::from) })
        .collect::<HashSet<_>>();
    for (mut background_color, wasd, entity) in wasd_query.iter_mut() {
        if wasd_actions.contains(wasd) {
            commands.entity(entity).insert(BackgroundColorTween {
                start_color: background_color.0,
                end_color: Color::rgb(0.5, 0.5, 0.5),
                duration: 0.1,
                elapsed: 0.0,
            });
        } else {
            background_color.0 = Color::rgb(0.2, 0.2, 0.2);
        }
    }
}
