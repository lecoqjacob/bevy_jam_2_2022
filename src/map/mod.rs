use crate::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct MapPlugin;

const TILE_MAP_SIZE: u32 = 64;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::RoundLocal, startup);
        app.add_enter_system(AppState::RoundOnline, startup);
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
                    texture: TileTexture(0),
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
        texture: TilemapTexture(textures.tiles.clone()),
        tile_size,
        transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
            &tilemap_size,
            &tile_size,
            0.0,
        ),
        ..Default::default()
    });
}
