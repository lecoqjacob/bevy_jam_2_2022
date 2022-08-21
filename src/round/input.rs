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

pub fn is_firing(input: u8) -> bool {
    input & INPUT_FIRE != 0
}
