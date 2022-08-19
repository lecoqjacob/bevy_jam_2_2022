use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    Left,
    Right,
    Up,
    Down,
}

impl Facing {
    pub fn index(&self) -> usize {
        match self {
            Facing::Left => 0,
            Facing::Right => 1,
            Facing::Up => 2,
            Facing::Down => 3,
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub map_level: i32,
    pub facing: Facing,
}
