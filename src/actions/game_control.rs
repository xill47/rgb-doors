use bevy::prelude::{Input, KeyCode, Res, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
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
        }
    }

    pub fn movement(&self) -> Vec2 {
        match self {
            GameControl::Up => Vec2::new(0.0, 1.0),
            GameControl::Down => Vec2::new(0.0, -1.0),
            GameControl::Left => Vec2::new(-1.0, 0.0),
            GameControl::Right => Vec2::new(1.0, 0.0),
        }
    }
}
