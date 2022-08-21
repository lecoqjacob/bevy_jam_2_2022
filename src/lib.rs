#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod camera;
mod components;
mod loading;
mod map;
mod menu;
mod networking;
mod random;
mod round;
mod state;
mod utils;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy::render::texture::ImageSettings;
    pub use bevy::winit::WinitSettings;
    pub use iyes_loopless::prelude::*;

    pub use bevy_ggrs::GGRSPlugin;
    pub use ggrs::Config;

    pub use bevy_ecs_tilemap::prelude::*;

    pub use bracket_geometry::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    pub use bracket_random::prelude::*;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::loading::*;
    pub use crate::map::*;
    pub use crate::menu::*;
    pub use crate::networking::*;
    pub use crate::random::*;
    pub use crate::round::*;
    pub use crate::state::*;
    pub use crate::utils::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 60;
    pub const UI_HEIGHT: i32 = 10;

    pub const NUM_PLAYERS: usize = 2;
    pub const ROLLBACK_SYSTEMS: &str = "rollback_systems";
    pub const CHECKSUM_UPDATE: &str = "checksum_update";
    pub const MAX_PREDICTION: usize = 12;
    pub const INPUT_DELAY: usize = 2;
    pub const CHECK_DISTANCE: usize = 2;
}

pub use prelude::*;

pub const LAUNCHER_TITLE: &str = "Bevy Shell - Template";

pub fn app() -> App {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        fit_canvas_to_parent: true,
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        width: SCREEN_WIDTH as f32 * 10.0,
        height: SCREEN_HEIGHT as f32 * 10.0,
        ..Default::default()
    })
    .insert_resource(ImageSettings::default_nearest())
    .insert_resource(ClearColor(Color::hex("171717").unwrap()));

    app.add_loopless_state(AppState::AssetLoading)
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugins(MenuPlugins)
        .add_plugin(MapPlugin)
        .add_plugin(NetworkingPlugin)
        .add_plugin(RoundPlugin);

    app
}
