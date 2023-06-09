use bevy::prelude::{Input, KeyCode, Res};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
    ColorSwitch,
    LevelReset,
}

impl GameControl {
    pub fn check_input(
        &self,
        checker: &dyn Fn(&Res<Input<KeyCode>>, KeyCode) -> bool,
        keyboard_input: &Res<Input<KeyCode>>,
    ) -> bool {
        match self {
            GameControl::Up => {
                checker(keyboard_input, KeyCode::W) || checker(keyboard_input, KeyCode::Up)
            }
            GameControl::Down => {
                checker(keyboard_input, KeyCode::S) || checker(keyboard_input, KeyCode::Down)
            }
            GameControl::Left => {
                checker(keyboard_input, KeyCode::A) || checker(keyboard_input, KeyCode::Left)
            }
            GameControl::Right => {
                checker(keyboard_input, KeyCode::D) || checker(keyboard_input, KeyCode::Right)
            }
            GameControl::ColorSwitch => checker(keyboard_input, KeyCode::Space),
            GameControl::LevelReset => checker(keyboard_input, KeyCode::R),
        }
    }
}
