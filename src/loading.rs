use crate::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy_logo: Handle<Image>,

    #[asset(path = "textures/bullet.png")]
    pub bullet: Handle<Image>,

    #[asset(path = "textures/arrow.png")]
    pub arrow: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/dos.ttf")]
    pub dos: Handle<Font>,

    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

pub struct MeshAssets {
    pub ring: Handle<Mesh>,
}

pub struct MaterialAssets {
    pub transparent_red: Handle<ColorMaterial>,
    pub transparent_blue: Handle<ColorMaterial>,
    pub transparent_green: Handle<ColorMaterial>,
    pub transparent_purple: Handle<ColorMaterial>,
}

impl MaterialAssets {
    pub fn get(&self, color: Color) -> Handle<ColorMaterial> {
        if color == BLUE {
            return self.transparent_blue.clone();
        } else if color == RED {
            return self.transparent_red.clone();
        } else if color == PURPLE {
            return self.transparent_purple.clone();
        } else if color == GREEN {
            return self.transparent_green.clone();
        }

        unreachable!("Should never get here")
    }
}

#[derive(Component)]
pub struct LoadingMenu;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ring_mesh = meshes.add(shape::Circle::new(100.).into());
    commands.insert_resource(MeshAssets { ring: ring_mesh });

    let transparent_blue =
        materials.add(ColorMaterial::from(Color::rgba(BLUE.r(), BLUE.g(), BLUE.b(), 0.2)));
    let transparent_green =
        materials.add(ColorMaterial::from(Color::rgba(GREEN.r(), GREEN.g(), GREEN.b(), 0.2)));
    let transparent_red =
        materials.add(ColorMaterial::from(Color::rgba(RED.r(), RED.g(), RED.b(), 0.2)));
    let transparent_purple =
        materials.add(ColorMaterial::from(Color::rgba(PURPLE.r(), PURPLE.g(), PURPLE.b(), 0.2)));

    commands.insert_resource(MaterialAssets {
        transparent_blue,
        transparent_green,
        transparent_red,
        transparent_purple,
    });

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
        app.insert_resource(RandomNumbers::default());

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
