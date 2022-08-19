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
    let mut cam = Camera2dBundle::default();
    cam.transform.scale = Vec3::new(0.5, 0.5, 1.0);
    commands.spawn_bundle(cam).insert(MainCamera);
}

// Move the camera when the player moves. See the Changed in the query
pub fn camera_move(
    windows: Res<Windows>,
    player_query: Query<&Point, (Changed<Point>, With<Player>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    // if player position got updated
    for player_position in player_query.iter() {
        let mut camera_transform = camera_query.single_mut();

        // get camera transform and window
        let window = windows.get_primary().unwrap();

        // calculate new coordinates and update
        let cam_x = convert_pos(player_position.x as f32, window.width() as f32, SCREEN_WIDTH as f32);
        let cam_y = convert_pos(player_position.y as f32, window.height() as f32, SCREEN_HEIGHT as f32);

        camera_transform.translation = Vec3::new(cam_x, cam_y, 999.0);
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(CoreStage::PostUpdate, RenderStage::Camera, SystemStage::single_threaded())
            .add_stage_after(
                RenderStage::Camera,
                RenderStage::RenderPostUpdate,
                SystemStage::single_threaded(),
            );

        app.add_startup_system(setup_game_camera)
            .add_system_to_stage(RenderStage::Camera, camera_move)
            .add_system_set_to_stage(
                RenderStage::RenderPostUpdate,
                SystemSet::new().with_system(position_translation).with_system(size_scaling),
            );
    }
}
