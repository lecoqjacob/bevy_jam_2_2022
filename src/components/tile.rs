use crate::prelude::*;

#[derive(Component)]
pub struct TileSize {
    pub width: f32,
    pub height: f32,
}
impl TileSize {
    pub fn square(x: f32) -> Self {
        Self { width: x, height: x }
    }
}

#[derive(Component)]
pub struct ExitTile;
