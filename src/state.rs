use bevy::prelude::StageLabel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,

    MainMenu,
    GameOver,

    WorldGeneration,
    DungeonCrawlEnter,
    DungeonCrawl(TurnState),
    DungeonCrawlExitToMenu,
    DungeonCrawlDescend,
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
