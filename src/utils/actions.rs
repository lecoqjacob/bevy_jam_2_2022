use crate::prelude::*;

#[derive(Eq, PartialEq, Debug)]
pub enum GameKey {
    // Movement
    Up,
    Down,
    Left,
    Right,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
    // Actions
    TakeStairs,
    Escape,
    Select,
    SkipTurn,
    Pickup,
    Inventory,
    Drop,
    Remove,
    Apply,
    Equip,
}

fn keycode_to_gamekey(key: KeyCode) -> Option<GameKey> {
    match key {
        // Movement
        KeyCode::Up | KeyCode::Numpad8 | KeyCode::K => Some(GameKey::Up),
        KeyCode::Down | KeyCode::Numpad2 | KeyCode::J => Some(GameKey::Down),
        KeyCode::Left | KeyCode::Numpad4 | KeyCode::H => Some(GameKey::Left),
        KeyCode::Right | KeyCode::Numpad6 | KeyCode::L => Some(GameKey::Right),
        KeyCode::Y | KeyCode::Numpad7 => Some(GameKey::LeftUp),
        KeyCode::U | KeyCode::Numpad9 => Some(GameKey::RightUp),
        KeyCode::B | KeyCode::Numpad1 => Some(GameKey::LeftDown),
        KeyCode::N | KeyCode::Numpad3 => Some(GameKey::RightDown),

        // Actions
        KeyCode::Period => Some(GameKey::TakeStairs),
        KeyCode::Escape => Some(GameKey::Escape),
        KeyCode::Return => Some(GameKey::Select),
        KeyCode::Space => Some(GameKey::SkipTurn),
        KeyCode::G => Some(GameKey::Pickup),
        KeyCode::I => Some(GameKey::Inventory),
        KeyCode::D => Some(GameKey::Drop),
        KeyCode::R => Some(GameKey::Remove),
        KeyCode::A => Some(GameKey::Apply),
        KeyCode::E => Some(GameKey::Equip),
        _ => None,
    }
}

fn gamekey_to_keycode(key: GameKey) -> Vec<KeyCode> {
    match key {
        GameKey::Up => vec![KeyCode::Up, KeyCode::Numpad8, KeyCode::K],
        GameKey::Down => vec![KeyCode::Down, KeyCode::Numpad2, KeyCode::J],
        GameKey::Left => vec![KeyCode::Left, KeyCode::Numpad4, KeyCode::H],
        GameKey::Right => vec![KeyCode::Right, KeyCode::Numpad6, KeyCode::L],
        GameKey::LeftUp => vec![KeyCode::Y, KeyCode::Numpad7],
        GameKey::LeftDown => vec![KeyCode::B, KeyCode::Numpad1],
        GameKey::RightUp => vec![KeyCode::U, KeyCode::Numpad9],
        GameKey::RightDown => vec![KeyCode::N, KeyCode::Numpad3],
        GameKey::TakeStairs => vec![KeyCode::Period],
        GameKey::Escape => vec![KeyCode::Escape],
        GameKey::Select => vec![KeyCode::Return],
        GameKey::SkipTurn => vec![KeyCode::Space],
        GameKey::Pickup => vec![KeyCode::G],
        GameKey::Inventory => vec![KeyCode::I],
        GameKey::Drop => vec![KeyCode::D],
        GameKey::Remove => vec![KeyCode::R],
        GameKey::Apply => vec![KeyCode::A],
        GameKey::Equip => vec![KeyCode::E],
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
