use std::collections::HashSet;

use bevy::prelude::*;

use crate::actions::MovementDirection;

#[derive(Debug, Default, Clone, Component, Eq, PartialEq)]
pub struct ForbiddenMovement {
    pub forbidden: HashSet<MovementDirection>,
}
