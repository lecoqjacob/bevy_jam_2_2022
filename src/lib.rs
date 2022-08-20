#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod camera;
mod checksum;
mod components;
mod loading;
mod map;
mod menu;
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

    pub use bracket_geometry::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    pub use bracket_random::prelude::*;

    pub use crate::checksum::*;
    pub use crate::menu::*;
    pub use crate::round::*;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::loading::*;
    pub use crate::map::*;
    pub use crate::random::*;
    pub use crate::state::*;
    pub use crate::utils::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 60;
    pub const UI_HEIGHT: i32 = 10;

    pub const NUM_PLAYERS: usize = 2;
    pub const FPS: usize = 60;
    pub const ROLLBACK_SYSTEMS: &str = "rollback_systems";
    pub const CHECKSUM_UPDATE: &str = "checksum_update";
    pub const MAX_PREDICTION: usize = 12;
    pub const INPUT_DELAY: usize = 2;
    pub const CHECK_DISTANCE: usize = 2;
}

pub use prelude::*;

pub const LAUNCHER_TITLE: &str = "Bevy Shell - Template";

#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type State = u8;
    type Input = round::Input;
    type Address = String;
}

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

    GGRSPlugin::<GGRSConfig>::new()
        .with_update_frequency(FPS)
        .with_input_system(round::input)
        .register_rollback_type::<Transform>()
        .register_rollback_type::<Velocity>()
        .register_rollback_type::<FrameCount>()
        .register_rollback_type::<Checksum>()
        .with_rollback_schedule(
            Schedule::default()
                .with_stage(
                    ROLLBACK_SYSTEMS,
                    SystemStage::parallel()
                        .with_system(apply_inputs.label(SystemLabels::Input))
                        .with_system(update_velocity.label(SystemLabels::Velocity).after(SystemLabels::Input))
                        .with_system(move_players.after(SystemLabels::Velocity))
                        .with_system(increase_frame_count),
                )
                .with_stage_after(
                    ROLLBACK_SYSTEMS,
                    CHECKSUM_UPDATE,
                    SystemStage::parallel().with_system(checksum_players),
                ),
        )
        .build(&mut app);

    app.add_loopless_state(AppState::AssetLoading)
        .add_plugins(DefaultPlugins)
        .add_plugin(LoadingPlugin)
        .add_plugins(MenuPlugins)
        .add_plugin(MapPlugin);

    app
}
