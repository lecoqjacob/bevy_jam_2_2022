use bevy::prelude::{StageLabel, SystemLabel};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    AssetLoading,
    MenuMain,
    MenuOnline,
    MenuConnect,
    RoundLocal,
    RoundOnline,
    Win,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, StageLabel)]
pub enum PlayerStage {
    GenerateActions,
    HandleActions,
    Cleanup,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, StageLabel)]
pub enum AIStage {
    HandleAI,
    GenerateActions,
    HandleActions,
    Cleanup,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum RenderStage {
    Camera,
    RenderPostUpdate,
}

#[derive(SystemLabel, Debug, Clone, Hash, Eq, PartialEq)]
pub enum SystemLabels {
    Input,
    Velocity,
}
