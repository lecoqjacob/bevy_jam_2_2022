// This is the folder to handle the `round` or `in_game` state

use crate::prelude::*;
use bevy::math::Vec3Swizzles;
use bevy_ggrs::{Rollback, RollbackIdProvider, SessionType};
use bytemuck::{Pod, Zeroable};
use ggrs::{InputStatus, P2PSession, PlayerHandle};

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_ATTACK: u8 = 1 << 4;

const BLUE: Color = Color::rgb(0.8, 0.6, 0.2);
const ORANGE: Color = Color::rgb(0., 0.35, 0.8);
const MAGENTA: Color = Color::rgb(0.9, 0.2, 0.2);
const GREEN: Color = Color::rgb(0.35, 0.7, 0.35);
const PLAYER_COLORS: [Color; 4] = [BLUE, ORANGE, MAGENTA, GREEN];

const PLAYER_SIZE: f32 = 50.;
const MOV_SPEED: f32 = 0.1;
const MAX_SPEED: f32 = 7.5;
const FRICTION: f32 = 0.98;
const DRIFT: f32 = 0.95;
const ARENA_SIZE: f32 = 720.0;

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Pod, Zeroable)]
pub struct GameInput {
    pub inp: u8,
}

#[derive(Component)]
pub struct RoundEntity;

#[derive(Default, Reflect, Component)]
pub struct Velocity(pub Vec2);

#[derive(Default, Reflect, Component)]
pub struct PlayerControls {
    accel: f32,
}

#[derive(Default, Reflect, Hash, Component)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
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
            inp |= INPUT_ATTACK;
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
            inp |= INPUT_ATTACK;
        }
    }

    GameInput { inp }
}

pub fn setup_round(mut commands: Commands) {
    commands.init_resource::<FrameCount>();
    commands.init_resource::<CursorCoordinates>();
}

pub fn spawn_players(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut rip: ResMut<RollbackIdProvider>,
) {
    let r = ARENA_SIZE / 4.;

    for (handle, color) in PLAYER_COLORS.iter().enumerate().take(NUM_PLAYERS) {
        let rot = handle as f32 / NUM_PLAYERS as f32 * 2. * std::f32::consts::PI;
        let x = r * rot.cos();
        let y = r * rot.sin();

        let mut transform = Transform::from_translation(Vec3::new(x, y, 1.));
        transform.rotate(Quat::from_rotation_z(rot));

        commands
            .spawn_bundle(SpriteSheetBundle {
                transform,
                texture_atlas: textures.tiles_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: 3,
                    color: *color,
                    custom_size: Some(Vec2::new(PLAYER_SIZE * 0.5, PLAYER_SIZE * 0.5)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player { handle, facing: Facing::Down })
            .insert(Velocity::default())
            .insert(PlayerControls::default())
            .insert(Checksum::default())
            .insert(Rollback::new(rip.next_id()))
            .insert(RoundEntity);
    }
}

pub fn print_p2p_events(mut session: ResMut<P2PSession<GGRSConfig>>) {
    for event in session.events() {
        info!("GGRS Event: {:?}", event);
    }
}

pub fn check_win(mut commands: Commands) {
    let condition = false;
    let confirmed = false;

    if condition && confirmed {
        commands.insert_resource(NextState(AppState::Win));
        commands.insert_resource(MatchData { result: "Orange won!".to_owned() });
    }
}

pub fn cleanup_round(mut commands: Commands) {
    commands.remove_resource::<FrameCount>();
    commands.remove_resource::<LocalHandles>();
    commands.remove_resource::<P2PSession<GGRSConfig>>();
    commands.remove_resource::<SessionType>();
}

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

        c.accel = if input & INPUT_DOWN != 0 && input & INPUT_UP == 0 {
            -1.
        } else if input & INPUT_DOWN == 0 && input & INPUT_UP != 0 {
            1.
        } else {
            0.
        };
    }
}

pub fn update_velocity(mut query: Query<(&Transform, &mut Velocity, &PlayerControls)>) {
    for (t, mut v, c) in query.iter_mut() {
        let vel = &mut v.0;
        let up = t.up().xy();
        let right = t.right().xy();

        // car drives forward / backward
        *vel += (c.accel * MOV_SPEED) * up;

        // very realistic tire friction
        let forward_vel = up * vel.dot(up);
        let right_vel = right * vel.dot(right);

        *vel = forward_vel + right_vel * DRIFT;
        if c.accel.abs() <= 0.0 {
            *vel *= FRICTION;
        }

        // constrain velocity
        *vel = vel.clamp_length_max(MAX_SPEED);
    }
}

pub fn move_players(
    cursor_coords: Res<CursorCoordinates>,
    mut query: Query<(&mut Transform, &Velocity), With<Rollback>>,
    tilemap_query: Query<(&TilemapSize, &TilemapTileSize)>,
) {
    let mut map_width = 0.0;
    let mut map_height = 0.0;
    for (map_size, tile_size) in tilemap_query.iter() {
        map_width = map_size.x as f32 * tile_size.x;
        map_height = map_size.y as f32 * tile_size.y;
    }
    for (mut t, v) in query.iter_mut() {
        let vel = &v.0;

        // Rotate transform towards cursor
        let mouse_translation = cursor_coords.0;
        let to_mouse = (mouse_translation - t.translation.xy()).normalize();
        let rotate_to_mouse = Quat::from_rotation_arc(Vec3::Y, to_mouse.extend(0.));
        t.rotation = rotate_to_mouse;

        // apply velocity
        t.translation.x += vel.x;
        t.translation.y += vel.y;

        // constrain cube to plane
        t.translation.x = t.translation.x.clamp(-map_width / 2.0, map_width / 2.0);
        t.translation.y = t.translation.y.clamp(-map_height / 2.0, map_height / 2.0);
    }
}

pub struct OnlineRoundPlugin;
impl Plugin for OnlineRoundPlugin {
    fn build(&self, app: &mut App) {
        // online round
        app.add_enter_system_set(
            AppState::RoundOnline,
            ConditionSet::new().with_system(setup_round).with_system(spawn_players).into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::RoundOnline)
                .with_system(print_p2p_events)
                .with_system(check_win)
                .into(),
        )
        .add_exit_system_set(
            AppState::RoundOnline,
            ConditionSet::new()
                .with_system(cleanup_round)
                .with_system(despawn_all_with::<RoundEntity>)
                .into(),
        );
    }
}

pub struct LocalRoundPlugin;
impl Plugin for LocalRoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::RoundLocal,
            ConditionSet::new().with_system(setup_round).with_system(spawn_players).into(),
        )
        .add_system(check_win.run_in_state(AppState::RoundLocal))
        .add_exit_system_set(
            AppState::RoundLocal,
            ConditionSet::new()
                .with_system(cleanup_round)
                .with_system(despawn_all_with::<RoundEntity>)
                .into(),
        );
    }
}

pub struct RoundPlugin;
impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LocalRoundPlugin);
        app.add_plugin(OnlineRoundPlugin);
    }
}
