use bevy::prelude::SystemLabel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InGameState {
    Playing,
    Controls,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    AssetLoading,
    MenuMain,
    WorldGen,
    Controls,
    InGame,
    Win,
}

#[derive(SystemLabel, Debug, Clone, Hash, Eq, PartialEq)]
pub enum SystemLabels {
    CameraMove,
    Input,
    Spawning,

    PlayerMove,
    Collection,
    PlayerDamage,

    BulletReload,
    BulletMove,

    ZombieMove,
    ApplyForce,
    ZombieDamage,
}
