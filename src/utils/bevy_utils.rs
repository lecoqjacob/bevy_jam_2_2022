use crate::prelude::*;

// despawn all with specific component
pub fn despawn_all_with<C: Component>(mut commands: Commands, query: Query<Entity, With<C>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

/// Load the specified spritesheet at return a handle to the resulting [`TextureAtlas`]
// pub fn get_texture_atlas_handle(
//     spritesheet_path: &str,
//     sprite_size: Vec2,
//     columns: usize,
//     rows: usize,
//     asset_server: &AssetServer,
//     texture_atlases: &mut Assets<TextureAtlas>,
// ) -> Handle<TextureAtlas> {
//     let texture_handle = asset_server.load(spritesheet_path);
//     let texture_atlas = TextureAtlas::from_grid(texture_handle, sprite_size, columns, rows);
//     texture_atlases.add(texture_atlas)
// }

/// Build a properly sized [`TextureAtlasSprite`] with the given index
pub fn get_sprite(index: usize) -> TextureAtlasSprite {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::new(1.0, 1.0));
    sprite
}

/// Build a properly sized [`TextureAtlasSprite`] with the given index
pub fn get_sprite_with_color(index: usize, color: Color) -> TextureAtlasSprite {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::new(1.0, 1.0));
    sprite.color = color;
    sprite
}
