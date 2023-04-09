use std::collections::HashMap;

use bevy::prelude::*;

use crate::actions::MovementDirection;

use super::movement::MovementState;

#[derive(Debug, Default, Clone, Component, Eq, PartialEq)]
pub struct MovementSideEffects(HashMap<MovementDirection, SideEffect>);

impl MovementSideEffects {
    pub fn get(&self, direction: MovementDirection) -> SideEffect {
        self.0.get(&direction).copied().unwrap_or_default()
    }

    pub fn set(&mut self, direction: MovementDirection, side_effect: SideEffect) {
        self.0.insert(direction, side_effect);
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum SideEffect {
    #[default]
    None,
    DisabledMovement,
    MultiMove(u32),
}

impl SideEffect {
    pub fn transform_movement_state(&self, movement_state: MovementState) -> MovementState {
        match self {
            SideEffect::None => movement_state,
            SideEffect::DisabledMovement => MovementState::Idle,
            SideEffect::MultiMove(count) => match movement_state {
                MovementState::MultiMoving { direction, left } => {
                    MovementState::MultiMoving { direction, left }
                }
                MovementState::Moving(direction) => MovementState::MultiMoving {
                    direction,
                    left: *count,
                },
                MovementState::Idle => MovementState::Idle,
            },
        }
    }
}
