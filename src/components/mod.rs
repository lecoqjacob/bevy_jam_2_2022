use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Clock {
    pub initial: f32,
    pub current: f32,
}

impl Clock {
    pub fn new(initial: f32) -> Self {
        Self { initial, current: initial }
    }

    pub fn reset(&mut self) {
        self.current = self.initial;
    }
}

#[derive(Component, Debug)]
pub struct Dead;

#[derive(Default, Component, Clone, Debug, PartialEq, Reflect)]
pub struct Direction(pub Vec2);

// Why no work when adding directly to vec2?
impl From<Vec2> for Direction {
    fn from(v: Vec2) -> Self {
        Direction(v)
    }
}

impl From<Direction> for Vec2 {
    fn from(d: Direction) -> Self {
        d.0
    }
}

impl Direction {
    pub fn lerp(&mut self, other: Vec2, t: f32) {
        self.0 = self.0.lerp(other, t).normalize();
    }
}

#[derive(Default, Debug, Eq, PartialEq, Component, Reflect)]
pub struct CreatureType(pub Option<Entity>);

#[derive(Default, Debug, PartialEq, Component, Reflect)]
pub struct CreatureSize(pub f32);

#[derive(Default, Component, Debug, Reflect)]
pub struct CreatureFollow(pub f32);

// Doing the targetting
#[derive(Component, Clone, Debug)]
pub struct CreatureTarget(pub Entity);

#[derive(Default, Component, Clone, Debug)]
pub struct Health(pub i32);

#[derive(Default, Component, Clone, Debug)]
pub struct Boost(pub f32);

// #[derive(Default, Component, Clone, Debug)]
// pub struct Respawn(pub Color);

#[derive(Default, Component, Clone, Debug)]
pub struct MusicController(pub Handle<bevy::audio::AudioSink>);
