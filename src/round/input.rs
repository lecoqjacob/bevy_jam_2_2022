use crate::round::*;

pub const INPUT_UP: u8 = 1 << 0;
pub const INPUT_DOWN: u8 = 1 << 1;
pub const INPUT_LEFT: u8 = 1 << 2;
pub const INPUT_RIGHT: u8 = 1 << 3;
pub const INPUT_FIRE: u8 = 1 << 4;
pub const INPUT_SHIFT: u8 = 1 << 5;

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Pod, Zeroable)]
pub struct GameInput(pub u8, pub u8);

pub fn input(keyboard_input: Res<Input<KeyCode>>) -> [u8; 2] {
    let mut left_inp: u8 = 0;
    let mut right_inp: u8 = 0;

    // Left Player
    if GameKey::LocalUp.pressed(&keyboard_input) {
        left_inp |= INPUT_UP;
    }
    if GameKey::LocalLeft.pressed(&keyboard_input) {
        left_inp |= INPUT_LEFT;
    }
    if GameKey::LocalDown.pressed(&keyboard_input) {
        left_inp |= INPUT_DOWN;
    }
    if GameKey::LocalRight.pressed(&keyboard_input) {
        left_inp |= INPUT_RIGHT;
    }
    if GameKey::LocalAttack.pressed(&keyboard_input) {
        left_inp |= INPUT_FIRE;
    }
    if GameKey::LocalShift.pressed(&keyboard_input) {
        left_inp |= INPUT_SHIFT;
    }

    // Right Player
    if GameKey::Up.pressed(&keyboard_input) {
        right_inp |= INPUT_UP;
    }
    if GameKey::Left.pressed(&keyboard_input) {
        right_inp |= INPUT_LEFT;
    }
    if GameKey::Down.pressed(&keyboard_input) {
        right_inp |= INPUT_DOWN;
    }
    if GameKey::Right.pressed(&keyboard_input) {
        right_inp |= INPUT_RIGHT;
    }
    if GameKey::Attack.pressed(&keyboard_input) {
        right_inp |= INPUT_FIRE;
    }
    if GameKey::Shift.pressed(&keyboard_input) {
        right_inp |= INPUT_SHIFT;
    }

    // GameInput(left_inp, right_inp)
    [left_inp, right_inp]
}

pub fn apply_inputs(In(inputs): In<[u8; 2]>, mut query: Query<&mut PlayerControls>) {
    for (i, mut c) in query.iter_mut().enumerate() {
        let input = inputs[i];

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

        c.firing = input & INPUT_FIRE != 0;
        c.shift = input & INPUT_SHIFT != 0;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////////////////////

pub fn apply_forward_delta(time: &Time, transform: &mut Transform, acc: f32, move_speed: f32) {
    // get the player's forward vector by applying the current rotation to the players initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the player will move based on direction, the player's movement speed and delta time
    let movement_distance = acc * move_speed * time.delta_seconds();
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;

    transform.translation += translation_delta;
}
