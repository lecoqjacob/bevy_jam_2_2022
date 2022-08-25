use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    render::camera::Viewport,
    window::{WindowId, WindowResized},
};

use crate::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct LeftCamera;

#[derive(Component)]
pub struct RightCamera;

#[derive(Component)]
pub struct MiniMapCamera;

fn setup_game_camera(mut commands: Commands) {
    // Left Camera
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 999.).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera { priority: 0, ..Default::default() },
            ..default()
        })
        .insert(LeftCamera);

    // Right Camera
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 999.).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                // Renders the right camera after the left camera, which has a default priority of 0
                priority: 1,
                ..default()
            },
            camera_2d: Camera2d {
                // don't clear on the second camera because the first camera already cleared the window
                clear_color: ClearColorConfig::None,
            },
            ..default()
        })
        .insert(RightCamera);
}

pub fn camera_follow(
    // mut cameras: ParamSet<(Query<&mut Transform, With<LeftCamera>)>,
    player_query: Query<
        &Transform,
        (Without<LeftCamera>, Without<RightCamera>, Changed<Transform>, With<Player>),
    >,
    mut cameras: ParamSet<(
        Query<&mut Transform, (With<LeftCamera>, Without<MiniMapCamera>)>,
        Query<&mut Transform, (With<RightCamera>, Without<MiniMapCamera>)>,
    )>,
) {
    let players = player_query.iter().collect::<Vec<_>>();

    // Left Cam
    for mut t in cameras.p0().iter_mut() {
        if let Some(player_transform) = players.get(0) {
            let pos = player_transform.translation;
            t.translation.x = pos.x;
            t.translation.y = pos.y;
        }
    }

    // Right Cam
    for mut t in cameras.p1().iter_mut() {
        if let Some(player_transform) = players.get(1) {
            let pos = player_transform.translation;
            t.translation.x = pos.x;
            t.translation.y = pos.y;
        }
    }
}

fn update_camera_viewports(
    windows: Res<Windows>,
    mut resize_events: EventReader<WindowResized>,
    mut right_camera: Query<&mut Camera, With<RightCamera>>,
    mut left_camera: Query<&mut Camera, (With<LeftCamera>, Without<RightCamera>)>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.iter() {
        if resize_event.id == WindowId::primary() {
            let window = windows.primary();
            let mut left_camera = left_camera.single_mut();
            left_camera.viewport = Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(window.physical_width() / 2, window.physical_height()),
                ..default()
            });

            let mut right_camera = right_camera.single_mut();
            right_camera.viewport = Some(Viewport {
                physical_position: UVec2::new(window.physical_width() / 2, 0),
                physical_size: UVec2::new(window.physical_width() / 2, window.physical_height()),
                ..default()
            });
        }
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_game_camera);

        // Online
        app.add_system_set(
            ConditionSet::new()
                .label(SystemLabels::CameraMove)
                .run_in_state(AppState::InGame)
                .with_system(camera_follow)
                .with_system(update_camera_viewports)
                .into(),
        );
    }
}
