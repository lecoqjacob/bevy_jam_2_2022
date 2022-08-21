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

        c.steer = if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
            1.
        } else if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            -1.
        } else {
            0.
        };

        c.accel = if input & INPUT_DOWN != 0 && input & INPUT_UP == 0 {
            -1.
        } else if input & INPUT_DOWN == 0 && input & INPUT_UP != 0 {
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

pub fn move_bullet(mut query: Query<&mut Transform, With<Bullet>>) {
    for mut t in query.iter_mut() {
        // get the player's forward vector by applying the current rotation to the players initial facing vector
        let movement_direction = t.rotation * Vec3::Y;
        // get the distance the player will move based on direction, the player's movement speed and delta time
        let movement_distance = BULLET_SPEED * TIME_STEP;
        // create the change in translation using the new movement direction and distance
        let translation_delta = movement_direction * movement_distance;
        // update the player translation with our new translation delta
        t.translation += translation_delta;
    }
}
