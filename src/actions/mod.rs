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
            .map(|direction| match direction {
                GameControl::Up => PlayerMovement::Up,
                GameControl::Down => PlayerMovement::Down,
                GameControl::Left => PlayerMovement::Left,
                GameControl::Right => PlayerMovement::Right,
            })
    };
    if let Some(player_movement) = player_movement {
        actions.send(Actions {
            player_movement: Some(player_movement),
        });
    }
}
