use crate::{impl_new, prelude::*};

#[derive(Component, Debug)]
pub struct ApplyForceEvent(pub Entity, pub Vec2, pub f32);

#[derive(Component, Debug)]
pub struct DamageEvent {
    pub victim: Entity,
    pub attacker: Entity,
}

impl_new!(DamageEvent, victim: Entity, attacker: Entity);

#[derive(Component, Debug, Default, Eq, PartialEq)]
pub enum SpawnType {
    #[default]
    Player,
    Zombie,
}

#[derive(Debug, Default)]
pub struct SpawnEvent {
    pub handle: Option<usize>,
    pub color: Option<Color>,
    pub spawn_type: SpawnType,
}
