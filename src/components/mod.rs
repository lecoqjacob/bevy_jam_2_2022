use crate::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
pub struct BulletReady(pub bool);

#[derive(Component, Reflect, Default)]
pub struct Bullet;

#[derive(Component, Clone, Debug, PartialEq)]
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

// TODO: Maybe generalize this?
#[derive(Default, Clone, Debug, PartialEq, Copy, Component, Eq, Hash)]
pub struct CreatureType(pub usize);

impl From<usize> for CreatureType {
    fn from(val: usize) -> Self {
        CreatureType(val)
    }
}

impl std::fmt::Display for CreatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Type {}", self.0)
    }
}

#[derive(Component, Clone, Debug)]
pub struct CreatureTarget(pub usize, pub Entity);
