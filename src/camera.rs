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

#[derive(Default, Debug)]
pub struct CursorCoordinates(pub Vec2);

fn setup_game_camera(mut commands: Commands) {
    // Add a 2D Camera
    // commands.spawn_bundle(Camera2dBundle::default()).insert(MainCamera);
    let mut cam = Camera2dBundle::default();
    // cam.transform.scale = Vec3::new(0.5, 0.5, 1.0);
    commands.spawn_bundle(cam).insert(MainCamera);
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

// fn cursor_coordinates(
//     mut commands: Commands,
//     wnds: Res<Windows>,
//     q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
// ) {
//     let (camera, camera_transform) = q_camera.single();
//     if let Some(wnd) = if let RenderTarget::Window(id) = camera.target {
//         wnds.get(id)
//     } else {
//         wnds.get_primary()
//     } {
//         // check if the cursor is inside the window and get its position
//         if let Some(screen_pos) = wnd.cursor_position() {
//             // get the size of the window
//             let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

//             // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
//             let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

//             // matrix for undoing the projection and camera transform
//             let ndc_to_world =
//                 camera_transform.compute_matrix() * camera.projection_matrix().inverse();

//             // use it to convert ndc to world-space coordinates
//             let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

//             // reduce it to a 2D value
//             let world_pos: Vec2 = world_pos.truncate();

//             commands.insert_resource(CursorCoordinates(world_pos));
//         }
//     }
// }

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_game_camera);

        // Online
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::RoundOnline)
                .with_system(camera_follow)
                // .with_system(cursor_coordinates)
                .into(),
        );

        // Local
        // TODO: splitscreen for local, follow all players
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::RoundLocal)
                .with_system(camera_follow)
                // .with_system(cursor_coordinates)
                .into(),
        );
    }
}
