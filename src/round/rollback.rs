use bevy::{math::Vec3Swizzles, render::camera::RenderTarget};

use crate::round::*;

/*
 * ROLLBACK SYSTEMS
 */

pub fn increase_frame_count(mut frame_count: ResMut<FrameCount>) {
    frame_count.frame += 1;
}

pub fn apply_inputs(
    mut query: Query<(&mut PlayerControls, &Player)>,
    inputs: Res<Vec<(GameInput, InputStatus)>>,
) {
    for (mut c, p) in query.iter_mut() {
        let input = match inputs[p.handle].1 {
            InputStatus::Confirmed => inputs[p.handle].0.inp,
            InputStatus::Predicted => inputs[p.handle].0.inp,
            InputStatus::Disconnected => 0, // disconnected players do nothing
        };

        c.steer = if input & PlayerControls::INPUT_LEFT != 0
            && input & PlayerControls::INPUT_RIGHT == 0
        {
            1.
        } else if input & PlayerControls::INPUT_LEFT == 0
            && input & PlayerControls::INPUT_RIGHT != 0
        {
            -1.
        } else {
            0.
        };

        c.accel = if input & PlayerControls::INPUT_DOWN != 0
            && input & PlayerControls::INPUT_UP == 0
        {
            -1.
        } else if input & PlayerControls::INPUT_DOWN == 0 && input & PlayerControls::INPUT_UP != 0 {
            1.
        } else {
            0.
        };
    }
}

pub fn move_players(
    map_settings: Res<MapSettings>,
    mut query: Query<(&mut Transform, &PlayerControls, &Player), With<Rollback>>,
) {
    for (mut t, c, p) in query.iter_mut() {
        t.rotate_z(c.steer * p.rotation_speed * TIME_STEP);
        // t.rotation = p.rotation;

        // get the player's forward vector by applying the current rotation to the players initial facing vector
        let movement_direction = t.rotation * Vec3::Y;
        // get the distance the player will move based on direction, the player's movement speed and delta time
        let movement_distance = c.accel * p.movement_speed * TIME_STEP;
        // create the change in translation using the new movement direction and distance
        let translation_delta = movement_direction * movement_distance;
        // update the player translation with our new translation delta
        t.translation += translation_delta;

        // constrain cube to plane
        let (map_width, map_height) = (map_settings.width, map_settings.height);
        t.translation.x = t.translation.x.clamp(-map_width / 2.0, map_width / 2.0);
        t.translation.y = t.translation.y.clamp(-map_height / 2.0, map_height / 2.0);
    }
}

pub fn camera_coords(
    wnds: Res<Windows>,
    local_handles: Res<LocalHandles>,
    mut q_player: Query<(&mut Player, &Transform), Without<MainCamera>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let local_handle = local_handles.handles[0];
    let (camera, camera_transform) = q_camera.single();

    if let Some(wnd) = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id)
    } else {
        wnds.get_primary()
    } {
        // check if the cursor is inside the window and get its position
        if let Some(screen_pos) = wnd.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();

            q_player.iter_mut().filter(|(p, _)| p.handle == local_handle).for_each(
                |(mut p, player_transform)| {
                    let to_mouse = (world_pos - player_transform.translation.xy()).normalize();
                    let rotate_to_mouse = Quat::from_rotation_arc(Vec3::Y, to_mouse.extend(0.));
                    p.rotation = rotate_to_mouse;
                },
            );
        }
    }
}
