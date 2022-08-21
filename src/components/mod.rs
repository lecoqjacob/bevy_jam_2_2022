use crate::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
pub struct BulletReady(pub bool);

#[derive(Component, Reflect, Default)]
pub struct Bullet;
