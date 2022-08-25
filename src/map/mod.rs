use crate::{impl_new, prelude::*};
use bevy::prelude::*;

mod cache_grid;
mod tiled;

pub use self::tiled::*;
pub use cache_grid::*;

const TILE_MAP_WIDTH: u32 = 128;
const TILE_MAP_HEIGHT: u32 = 128;

#[derive(Debug)]
pub struct MapSettings {
    pub width: f32,
    pub height: f32,
}

impl_new!(MapSettings, width: f32, height: f32);

impl MapSettings {
    pub fn size(&self) -> f32 {
        self.width * self.height
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tilemap_size = TilemapSize { x: TILE_MAP_WIDTH, y: TILE_MAP_HEIGHT };

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let map_handle: Handle<TiledMap> = asset_server.load("maps/map.tmx");

    // Spawn Tilemap
    commands
        .spawn()
        .insert_bundle(TiledMapBundle { tiled_map: map_handle, ..Default::default() })
        .insert(RoundEntity);

    commands.insert_resource(MapSettings::new(
        tilemap_size.x as f32 * tile_size.x,
        tilemap_size.y as f32 * tile_size.y,
    ));

    commands.insert_resource(NextState(AppState::InGame))
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::WorldGen, startup);
    }
}
