use crate::{impl_new, prelude::*};
use bevy::prelude::*;
use bevy_ggrs::SessionType;

mod cache_grid;
mod tiled;
mod utils;

pub use self::tiled::*;
pub use cache_grid::*;
pub use utils::*;

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

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, session_type: Res<SessionType>) {
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

    let next_state = match *session_type {
        SessionType::SyncTestSession => AppState::RoundLocal,
        SessionType::P2PSession => AppState::RoundOnline,
        _ => unreachable!("We Dont handle spectator D:"),
    };

    commands.insert_resource(NextState(next_state))
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::WorldGen, startup);
    }
}
