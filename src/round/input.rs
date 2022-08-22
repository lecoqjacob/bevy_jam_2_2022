use crate::round::*;

pub const INPUT_UP: u8 = 1 << 0;
pub const INPUT_DOWN: u8 = 1 << 1;
pub const INPUT_LEFT: u8 = 1 << 2;
pub const INPUT_RIGHT: u8 = 1 << 3;
pub const INPUT_FIRE: u8 = 1 << 4;

pub const BULLET_SPEED: f32 = 500.;

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Pod, Zeroable)]
pub struct GameInput {
    pub inp: u8,
}

pub fn input(
    handle: In<PlayerHandle>,
    local_handles: Res<LocalHandles>,
    keyboard_input: Res<Input<KeyCode>>,
) -> GameInput {
    let mut inp: u8 = 0;

    if handle.0 == local_handles.handles[0] {
        if GameKey::LocalUp.pressed(&keyboard_input) {
            inp |= INPUT_UP;
        }
        if GameKey::LocalLeft.pressed(&keyboard_input) {
            inp |= INPUT_LEFT;
        }
        if GameKey::LocalDown.pressed(&keyboard_input) {
            inp |= INPUT_DOWN;
        }
        if GameKey::LocalRight.pressed(&keyboard_input) {
            inp |= INPUT_RIGHT;
        }
        if GameKey::LocalAttack.pressed(&keyboard_input) {
            inp |= INPUT_FIRE;
        }
    } else {
        if GameKey::Up.pressed(&keyboard_input) {
            inp |= INPUT_UP;
        }
        if GameKey::Left.pressed(&keyboard_input) {
            inp |= INPUT_LEFT;
        }
        if GameKey::Down.pressed(&keyboard_input) {
            inp |= INPUT_DOWN;
        }
        if GameKey::Right.pressed(&keyboard_input) {
            inp |= INPUT_RIGHT;
        }
        if GameKey::Attack.pressed(&keyboard_input) {
            inp |= INPUT_FIRE;
        }
    }

    GameInput { inp }
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

////////////////////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////////////////////

pub fn is_firing(input: u8) -> bool {
    input & INPUT_FIRE != 0
}

pub fn apply_forward_delta(transform: &mut Transform, acc: f32, move_speed: f32) {
    // get the player's forward vector by applying the current rotation to the players initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the player will move based on direction, the player's movement speed and delta time
    let movement_distance = acc * move_speed * TIME_STEP;
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;

    transform.translation += translation_delta;
}
