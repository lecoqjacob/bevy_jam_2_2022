use crate::{impl_new, prelude::*};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ggrs::SessionType;

const TILE_MAP_SIZE: u32 = 64;

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

fn startup(mut commands: Commands, textures: Res<TextureAssets>) {
    let tilemap_size = TilemapSize { x: TILE_MAP_SIZE, y: TILE_MAP_SIZE };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(tilemap_size);

    // Spawn the elements of the tilemap.
    for x in 0..TILE_MAP_SIZE {
        for y in 0..TILE_MAP_SIZE {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    texture: TileTexture(23),
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };

    commands.entity(tilemap_entity).insert_bundle(TilemapBundle {
        grid_size: TilemapGridSize { x: 16.0, y: 16.0 },
        size: tilemap_size,
        storage: tile_storage,
        texture: TilemapTexture(textures.tileset.clone()),
        tile_size,
        transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
            &tilemap_size,
            &tile_size,
            0.0,
        ),
        ..Default::default()
    });

    commands.insert_resource(MapSettings::new(
        tilemap_size.x as f32 * tile_size.x,
        tilemap_size.y as f32 * tile_size.y,
    ));

    // let next_state = match *session_type {
    //     SessionType::SyncTestSession => AppState::RoundLocal,
    //     SessionType::P2PSession => AppState::RoundOnline,
    //     _ => unreachable!("We Dont handle spectator D:"),
    // };

    // println!("{:?}", next_state);

    // commands.insert_resource(NextState(next_state))
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // app.add_enter_system(AppState::WorldGen, startup);
        app.add_enter_system(AppState::RoundLocal, startup);
    }
}
