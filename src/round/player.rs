use crate::round::*;

pub mod player_settings {
    pub const DEFAULT_ROT_SPEED: f32 = 360.;
    pub const DEFAULT_PLAYER_SIZE: f32 = 15.;
    pub const DEFAULT_MOVE_SPEED: f32 = 300.;

    pub const MAX_SPEED: f32 = 300.;
    pub const FRICTION: f32 = 0.98;
    pub const DRIFT: f32 = 1.0;
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

impl Facing {
    pub fn index(&self) -> usize {
        match self {
            Facing::Left => 0,
            Facing::Right => 1,
            Facing::Up => 2,
            Facing::Down => 3,
        }
    }
}

#[derive(Default, Component)]
pub struct Player {
    pub size: f32,
    pub handle: usize,
    pub facing: Facing,

    /// rotation speed in radians per second
    pub rotation_speed: f32,
    /// linear speed in meters per second
    pub movement_speed: f32,
    pub rotation: Quat,
}

impl Player {
    pub fn new(handle: usize) -> Self {
        Self {
            handle,
            size: player_settings::DEFAULT_PLAYER_SIZE,
            movement_speed: player_settings::DEFAULT_MOVE_SPEED,
            rotation_speed: f32::to_radians(player_settings::DEFAULT_ROT_SPEED),
            ..Default::default()
        }
    }
}

#[derive(Default, Reflect, Component, Debug)]
pub struct PlayerControls {
    pub accel: f32,
    pub steer: f32,
}

impl PlayerControls {
    pub const INPUT_UP: u8 = 1 << 0;
    pub const INPUT_DOWN: u8 = 1 << 1;
    pub const INPUT_LEFT: u8 = 1 << 2;
    pub const INPUT_RIGHT: u8 = 1 << 3;
    pub const INPUT_ATTACK: u8 = 1 << 4;
}

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
            inp |= PlayerControls::INPUT_UP;
        }
        if GameKey::LocalLeft.pressed(&keyboard_input) {
            inp |= PlayerControls::INPUT_LEFT;
        }
        if GameKey::LocalDown.pressed(&keyboard_input) {
            inp |= PlayerControls::INPUT_DOWN;
        }
        if GameKey::LocalRight.pressed(&keyboard_input) {
            inp |= PlayerControls::INPUT_RIGHT;
        }
        if GameKey::LocalAttack.pressed(&keyboard_input) {
            inp |= PlayerControls::INPUT_ATTACK;
        }
    } else {
        if GameKey::Up.pressed(&keyboard_input) {
            inp |= PlayerControls::INPUT_UP;
        }
        if GameKey::Left.pressed(&keyboard_input) {
            inp |= PlayerControls::INPUT_LEFT;
        }
        if GameKey::Down.pressed(&keyboard_input) {
            inp |= PlayerControls::INPUT_DOWN;
        }
        if GameKey::Right.pressed(&keyboard_input) {
            inp |= PlayerControls::INPUT_RIGHT;
        }
        if GameKey::Attack.pressed(&keyboard_input) {
            inp |= PlayerControls::INPUT_ATTACK;
        }
    }

    GameInput { inp }
}
