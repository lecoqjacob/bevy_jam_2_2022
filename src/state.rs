use bevy::prelude::SystemLabel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    AssetLoading,
    MenuMain,
    MenuOnline,
    MenuConnect,
    WorldGen,
    RoundLocal,
    RoundOnline,
    Win,
}

#[derive(SystemLabel, Debug, Clone, Hash, Eq, PartialEq)]
pub enum SystemLabels {
    Input,
    Velocity,
}
