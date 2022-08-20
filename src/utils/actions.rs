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

fn keycode_to_gamekey(key: KeyCode) -> Option<GameKey> {
    match key {
        // Local Keys
        KeyCode::W => Some(GameKey::Up),
        KeyCode::S => Some(GameKey::Down),
        KeyCode::A => Some(GameKey::Left),
        KeyCode::D => Some(GameKey::Right),
        KeyCode::L => Some(GameKey::Pickup),
        KeyCode::RShift => Some(GameKey::Attack),

        // Online Keys
        KeyCode::Up => Some(GameKey::Up),
        KeyCode::Down => Some(GameKey::Down),
        KeyCode::Left => Some(GameKey::Left),
        KeyCode::Right => Some(GameKey::Right),
        KeyCode::G => Some(GameKey::Pickup),
        KeyCode::Space => Some(GameKey::Attack),
        _ => None,
    }
}

fn gamekey_to_keycode(key: GameKey) -> Vec<KeyCode> {
    match key {
        // Local Keys
        GameKey::LocalUp => vec![KeyCode::W],
        GameKey::LocalDown => vec![KeyCode::S],
        GameKey::LocalPickup => vec![KeyCode::L],
        GameKey::LocalLeft => vec![KeyCode::Left],
        GameKey::LocalRight => vec![KeyCode::Right],
        GameKey::LocalAttack => vec![KeyCode::RShift],

        // Online Keys
        GameKey::Up => vec![KeyCode::Up],
        GameKey::Down => vec![KeyCode::Down],
        GameKey::Left => vec![KeyCode::Left],
        GameKey::Right => vec![KeyCode::Right],
        GameKey::Pickup => vec![KeyCode::G],
        GameKey::Attack => vec![KeyCode::Space],
    }
}

pub trait VirtualGameKey {
    fn get_key(&self) -> Option<GameKey>;
}

pub trait VirtualInput {
    fn reset_key(&mut self, game_key: GameKey);
}

impl VirtualGameKey for KeyCode {
    fn get_key(&self) -> Option<GameKey> {
        keycode_to_gamekey(*self)
    }
}

impl VirtualGameKey for Option<KeyCode> {
    fn get_key(&self) -> Option<GameKey> {
        match self {
            Some(key) => keycode_to_gamekey(*key),
            None => None,
        }
    }
}

impl VirtualGameKey for Option<&KeyCode> {
    fn get_key(&self) -> Option<GameKey> {
        if let Some(key) = self.as_deref() {
            keycode_to_gamekey(*key)
        } else {
            None
        }
    }
}

impl VirtualGameKey for Input<KeyCode> {
    fn get_key(&self) -> Option<GameKey> {
        if let Some(key) = self.get_pressed().next() {
            keycode_to_gamekey(*key)
        } else {
            None
        }
    }
}

impl VirtualInput for Input<KeyCode> {
    fn reset_key(&mut self, game_key: GameKey) {
        let keys = gamekey_to_keycode(game_key);
        keys.iter().for_each(|key| self.reset(*key));
    }
}
