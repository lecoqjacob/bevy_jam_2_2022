use crate::prelude::*;

////////////////////////////////////////////////////////////////////////////////
/// Render Utility
////////////////////////////////////////////////////////////////////////////////

pub fn convert_pos(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&TileSize, &mut Transform)>) {
    if let Some(window) = windows.get_primary() {
        for (sprite_size, mut transform) in q.iter_mut() {
            let scale = Vec3::new(
                sprite_size.width / SCREEN_WIDTH as f32 * window.width() as f32,
                sprite_size.height / SCREEN_HEIGHT as f32 * window.height() as f32,
                1.0,
            );
            transform.scale = scale;
        }
    }
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Point, &mut Transform)>) {
    if let Some(window) = windows.get_primary() {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                convert_pos(pos.x as f32, window.width() as f32, SCREEN_WIDTH as f32),
                convert_pos(pos.y as f32, window.height() as f32, SCREEN_HEIGHT as f32),
                // convert_pos((pos.y + UI_HEIGHT / 2) as f32, window.height() as f32, SCREEN_HEIGHT as f32),
                transform.translation.z,
            );
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct MainCamera;

fn setup_game_camera(mut commands: Commands) {
    // Add a 2D Camera
    commands.spawn_bundle(Camera2dBundle::default()).insert(MainCamera);
}

pub fn camera_follow(
    local_handles: Res<LocalHandles>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_query: Query<(&Player, &Transform), (Without<MainCamera>, Changed<Transform>)>,
) {
    let local_handle = local_handles.handles[0];
    let mut camera_transform = camera_query.single_mut();
    player_query.iter().filter(|(p, _)| p.handle == local_handle).for_each(
        |(_, player_transform)| {
            let pos = player_transform.translation;
            camera_transform.translation.x = pos.x;
            camera_transform.translation.y = pos.y;
        },
    );
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_game_camera);
        app.add_system(camera_follow.run_in_state(AppState::RoundOnline));
        // TODO: splitscreen for local, follow all players
        app.add_system(camera_follow.run_in_state(AppState::RoundLocal));
    }
}
