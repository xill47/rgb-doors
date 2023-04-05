use std::collections::HashSet;

use bevy::prelude::*;

use crate::actions::PlayerMovement;

#[derive(Debug, Default, Clone, Component, Eq, PartialEq)]
pub struct ForbiddenMovement {
    pub forbidden: HashSet<PlayerMovement>,
}
