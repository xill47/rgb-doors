use bevy::prelude::*;

use crate::actions::game_control::*;
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Actions>()
            .add_system(set_movement_actions.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PlayerMovement {
    Up,
    Down,
    Left,
    Right,
}

impl PlayerMovement {
    pub fn movement(&self) -> IVec2 {
        match self {
            PlayerMovement::Up => IVec2::new(0, 1),
            PlayerMovement::Down => IVec2::new(0, -1),
            PlayerMovement::Left => IVec2::new(-1, 0),
            PlayerMovement::Right => IVec2::new(1, 0),
        }
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_movement: Option<PlayerMovement>,
    pub color_switch: Option<()>,
    pub level_reset: Option<()>,
}

pub fn set_movement_actions(
    mut actions: EventWriter<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let player_movement = {
        let directions = [
            GameControl::Up,
            GameControl::Down,
            GameControl::Left,
            GameControl::Right,
        ];
        directions
            .iter()
            .find(|&direction| {
                direction.check_input(&|input, code| input.just_pressed(code), &keyboard_input)
            })
            .and_then(|direction| match direction {
                GameControl::Up => Some(PlayerMovement::Up),
                GameControl::Down => Some(PlayerMovement::Down),
                GameControl::Left => Some(PlayerMovement::Left),
                GameControl::Right => Some(PlayerMovement::Right),
                _ => None,
            })
    };
    let color_switch = {
        GameControl::ColorSwitch
            .check_input(&|input, code| input.just_pressed(code), &keyboard_input)
    };
    let level_reset = {
        GameControl::LevelReset
            .check_input(&|input, code| input.just_pressed(code), &keyboard_input)
    };
    if player_movement.is_some() || color_switch || level_reset {
        actions.send(Actions {
            player_movement,
            color_switch: if color_switch { Some(()) } else { None },
            level_reset: if level_reset { Some(()) } else { None },
        });
    }
}
