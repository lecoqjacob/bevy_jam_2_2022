use crate::prelude::*;

#[derive(Eq, PartialEq, Debug)]
pub enum GameKey {
    // These are local keys for when the game is running locally
    LocalUp,
    LocalDown,
    LocalLeft,
    LocalRight,
    LocalAttack,
    LocalPickup,

    // These are remote keys for when the game is running online
    Up,
    Down,
    Left,
    Right,
    Attack,
    Pickup,
}

impl GameKey {
    pub fn key_code(&self) -> KeyCode {
        match self {
            GameKey::LocalUp => KeyCode::W,
            GameKey::LocalDown => KeyCode::S,
            GameKey::LocalLeft => KeyCode::A,
            GameKey::LocalRight => KeyCode::D,
            GameKey::LocalAttack => KeyCode::Space,
            GameKey::LocalPickup => KeyCode::E,
            GameKey::Up => KeyCode::Up,
            GameKey::Down => KeyCode::Down,
            GameKey::Left => KeyCode::Left,
            GameKey::Right => KeyCode::Right,
            GameKey::Attack => KeyCode::Return,
            GameKey::Pickup => KeyCode::E,
        }
    }

    pub fn reset(&self, keyboard_input: &mut Input<KeyCode>) {
        keyboard_input.reset(self.key_code());
    }

    pub fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            // Local
            GameKey::LocalUp => keyboard_input.just_released(KeyCode::W),
            GameKey::LocalDown => keyboard_input.just_released(KeyCode::S),
            GameKey::LocalLeft => keyboard_input.just_released(KeyCode::A),
            GameKey::LocalRight => keyboard_input.just_released(KeyCode::D),
            GameKey::LocalPickup => keyboard_input.just_released(KeyCode::G),
            GameKey::LocalAttack => keyboard_input.just_released(KeyCode::Space),
            //Online
            GameKey::Up => keyboard_input.just_released(KeyCode::Up),
            GameKey::Down => keyboard_input.just_released(KeyCode::Down),
            GameKey::Left => keyboard_input.just_released(KeyCode::Left),
            GameKey::Right => keyboard_input.just_released(KeyCode::Right),
            GameKey::Pickup => keyboard_input.just_released(KeyCode::Return),
            GameKey::Attack => keyboard_input.just_released(KeyCode::RShift),
        }
    }

    pub fn pressed(&self, keyboard_input: &Input<KeyCode>) -> bool {
        match self {
            // Local
            GameKey::LocalUp => keyboard_input.pressed(KeyCode::W),
            GameKey::LocalDown => keyboard_input.pressed(KeyCode::S),
            GameKey::LocalLeft => keyboard_input.pressed(KeyCode::A),
            GameKey::LocalRight => keyboard_input.pressed(KeyCode::D),
            GameKey::LocalPickup => keyboard_input.pressed(KeyCode::G),
            GameKey::LocalAttack => keyboard_input.pressed(KeyCode::Space),
            //Online
            GameKey::Up => keyboard_input.pressed(KeyCode::Up),
            GameKey::Down => keyboard_input.pressed(KeyCode::Down),
            GameKey::Left => keyboard_input.pressed(KeyCode::Left),
            GameKey::Right => keyboard_input.pressed(KeyCode::Right),
            GameKey::Pickup => keyboard_input.pressed(KeyCode::Return),
            GameKey::Attack => keyboard_input.pressed(KeyCode::RShift),
        }
    }

    pub fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            // Local
            GameKey::LocalUp => keyboard_input.just_pressed(KeyCode::W),
            GameKey::LocalDown => keyboard_input.just_pressed(KeyCode::S),
            GameKey::LocalLeft => keyboard_input.just_pressed(KeyCode::A),
            GameKey::LocalRight => keyboard_input.just_pressed(KeyCode::D),
            GameKey::LocalPickup => keyboard_input.just_pressed(KeyCode::G),
            GameKey::LocalAttack => keyboard_input.just_pressed(KeyCode::Space),
            //Online
            GameKey::Up => keyboard_input.just_pressed(KeyCode::Up),
            GameKey::Down => keyboard_input.just_pressed(KeyCode::Down),
            GameKey::Left => keyboard_input.just_pressed(KeyCode::Left),
            GameKey::Right => keyboard_input.just_pressed(KeyCode::Right),
            GameKey::Pickup => keyboard_input.just_pressed(KeyCode::Return),
            GameKey::Attack => keyboard_input.just_pressed(KeyCode::RShift),
        }
    }
}
