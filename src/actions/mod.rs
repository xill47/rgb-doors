use bevy::prelude::*;

use crate::actions::game_control::*;
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            .add_system(set_movement_actions.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<GameControl>,
}

pub fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    actions.player_movement = {
        let directions = [
            GameControl::Up,
            GameControl::Down,
            GameControl::Left,
            GameControl::Right,
        ];
        directions
            .iter()
            .find(|&direction| {
                direction.check(&|input, code| input.just_pressed(code), &keyboard_input)
            })
            .copied()
    }
}
