use crate::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
pub struct BulletReady(pub bool);

#[derive(Component, Reflect)]
pub struct Bullet;

#[derive(Component, Reflect, Debug)]
pub struct FiredBy(pub Entity);

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
#[derive(Reflect, Component, Clone, Debug)]
pub struct CreatureTarget(pub Entity);

#[derive(Default, Reflect, Component, Clone, Debug)]
pub struct Health(pub i32);

#[derive(Default, Reflect, Component, Clone, Debug)]
pub struct Respawn {
    pub time: f32,
    pub color: Color,
}
