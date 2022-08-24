use crate::prelude::*;

#[derive(Reflect, Component, Clone, Debug)]
pub struct ApplyForceEvent(pub Entity, pub Vec2, pub f32);

#[derive(Reflect, Component, Clone, Debug)]
pub enum RespawnType {
    Player,
    Zombie,
}

pub struct SpawnEvent {
    pub point: Option<(f32, f32)>,
    pub color: Option<Color>,
    pub handle: Option<usize>,
    pub spawn_type: RespawnType,
}
