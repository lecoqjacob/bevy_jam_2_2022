use crate::prelude::*;
use bevy_asset_loader::prelude::*;

/// Size of the sprite assets
pub const SPRITE_SIZE: f32 = 16.0;

/// Size of each tile for rendering
pub const TILE_SIZE: f32 = 32.0;

/// Z-buffer plane for player entities
pub const ZBUF_PLAYER: f32 = 10.0;

/// Z-buffer plane for moving entities (creatures...)
pub const ZBUF_CREATURES: f32 = 5.0;

/// Z-buffer plane for static entities (items...)
pub const ZBUF_ITEMS: f32 = 1.0;

/// Z-buffer plane for map tiles
pub const ZBUF_TILES: f32 = 0.0;

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 16, rows = 16))]
    #[asset(path = "textures/terminal8x8_transparent.png")]
    pub terminal: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 10, rows = 8))]
    #[asset(path = "textures/Undead0.png")]
    pub undead: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 8, rows = 4))]
    #[asset(path = "textures/Potion.png")]
    pub potions: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 4))]
    #[asset(path = "textures/Rogue.png")]
    pub rogue: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 20, rows = 51))]
    #[asset(path = "textures/Wall.png")]
    pub wall: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 21, rows = 39))]
    #[asset(path = "textures/Floor.png")]
    pub floor: Handle<TextureAtlas>,

    // #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 16, rows = 16))]
    #[asset(path = "textures/tileset.png")]
    pub tileset: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/dos.ttf")]
    pub dos: Handle<Font>,

    #[asset(path = "fonts/SDS_8x8.ttf")]
    pub sds_8x8: Handle<Font>,

    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(Component)]
pub struct LoadingMenu;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Loading Menu
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: UiColor(Color::hex("101010").unwrap()),
            ..Default::default()
        })
        .insert(LoadingMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Loading...",
                    TextStyle {
                        font_size: 100.0,
                        color: Color::WHITE,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    },
                ),
                ..Default::default()
            });
        });
}

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RandomNumbers>();

        app.add_startup_system(setup)
            .add_exit_system(AppState::Loading, despawn_all_with::<LoadingMenu>)
            .add_loading_state(
                LoadingState::new(AppState::Loading)
                    .with_collection::<FontAssets>()
                    .with_collection::<TextureAssets>()
                    .continue_to_state(AppState::WorldGeneration),
            );
    }
}
