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
    #[asset(path = "textures/man_blue_stand.png")]
    pub man_blue: Handle<Image>,

    #[asset(path = "bevy.png")]
    pub bevy_logo: Handle<Image>,

    #[asset(path = "tiles.png")]
    pub tiles: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 6, rows = 1))]
    #[asset(path = "tiles.png")]
    pub tiles_atlas: Handle<TextureAtlas>,
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
            .add_exit_system(AppState::AssetLoading, despawn_all_with::<LoadingMenu>)
            .add_loading_state(
                LoadingState::new(AppState::AssetLoading)
                    .with_collection::<FontAssets>()
                    .with_collection::<TextureAssets>()
                    .continue_to_state(AppState::MenuMain),
            );
    }
}
