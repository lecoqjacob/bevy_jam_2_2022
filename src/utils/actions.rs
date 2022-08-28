use crate::prelude::*;

#[derive(Eq, PartialEq, Debug)]
pub enum GameKey {
    // These are local keys for when the game is running locally
    LocalUp,
    LocalDown,
    LocalLeft,
    LocalRight,
    LocalAttack,
    LocalShift,

    // These are remote keys for when the game is running online
    Up,
    Down,
    Left,
    Right,
    Attack,
    Shift,
}

impl GameKey {
    pub fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            // Local
            GameKey::LocalUp => keyboard_input.just_released(KeyCode::W),
            GameKey::LocalDown => keyboard_input.just_released(KeyCode::S),
            GameKey::LocalLeft => keyboard_input.just_released(KeyCode::A),
            GameKey::LocalRight => keyboard_input.just_released(KeyCode::D),
            GameKey::LocalShift => keyboard_input.just_released(KeyCode::LShift),
            GameKey::LocalAttack => keyboard_input.just_released(KeyCode::Space),
            //Online
            GameKey::Up => keyboard_input.just_released(KeyCode::Up),
            GameKey::Down => keyboard_input.just_released(KeyCode::Down),
            GameKey::Left => keyboard_input.just_released(KeyCode::Left),
            GameKey::Right => keyboard_input.just_released(KeyCode::Right),
            GameKey::Shift => keyboard_input.just_released(KeyCode::B),
            GameKey::Attack => keyboard_input.just_released(KeyCode::M),
        }
    }

    pub fn pressed(&self, keyboard_input: &Input<KeyCode>) -> bool {
        match self {
            // Local
            GameKey::LocalUp => keyboard_input.pressed(KeyCode::W),
            GameKey::LocalDown => keyboard_input.pressed(KeyCode::S),
            GameKey::LocalLeft => keyboard_input.pressed(KeyCode::A),
            GameKey::LocalRight => keyboard_input.pressed(KeyCode::D),
            GameKey::LocalShift => keyboard_input.pressed(KeyCode::LShift),
            GameKey::LocalAttack => keyboard_input.pressed(KeyCode::Space),
            //Online
            GameKey::Up => keyboard_input.pressed(KeyCode::Up),
            GameKey::Down => keyboard_input.pressed(KeyCode::Down),
            GameKey::Left => keyboard_input.pressed(KeyCode::Left),
            GameKey::Right => keyboard_input.pressed(KeyCode::Right),
            GameKey::Shift => keyboard_input.pressed(KeyCode::B),
            GameKey::Attack => keyboard_input.pressed(KeyCode::M),
        }
    }

    pub fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            // Local
            GameKey::LocalUp => keyboard_input.just_pressed(KeyCode::W),
            GameKey::LocalDown => keyboard_input.just_pressed(KeyCode::S),
            GameKey::LocalLeft => keyboard_input.just_pressed(KeyCode::A),
            GameKey::LocalRight => keyboard_input.just_pressed(KeyCode::D),
            GameKey::LocalShift => keyboard_input.just_pressed(KeyCode::LShift),
            GameKey::LocalAttack => keyboard_input.just_pressed(KeyCode::Space),
            //Online
            GameKey::Up => keyboard_input.just_pressed(KeyCode::Up),
            GameKey::Down => keyboard_input.just_pressed(KeyCode::Down),
            GameKey::Left => keyboard_input.just_pressed(KeyCode::Left),
            GameKey::Right => keyboard_input.just_pressed(KeyCode::Right),
            GameKey::Shift => keyboard_input.just_pressed(KeyCode::B),
            GameKey::Attack => keyboard_input.just_pressed(KeyCode::M),
        }
    }
}
