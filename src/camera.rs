use crate::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Debug)]
pub struct CursorCoordinates(pub Vec2);

fn setup_game_camera(mut commands: Commands) {
    // Add a 2D Camera
    let mut cam = Camera2dBundle::default();
    cam.transform.translation.z = 999.0;
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

        // Online
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::RoundOnline)
                .with_system(camera_follow)
                .into(),
        );

        // Local
        // TODO: splitscreen for local, follow all players
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::RoundLocal)
                .with_system(camera_follow)
                .into(),
        );
    }
}
