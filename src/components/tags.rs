use crate::prelude::*;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    #[default]
    Down,
    Up,
    Left,
    Right,
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

#[derive(Default, Component)]
pub struct Player {
    pub handle: usize,
    pub facing: Facing,
}
