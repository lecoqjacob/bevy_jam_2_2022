// This is the folder to handle the `round` or `in_game` state

use crate::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider, SessionType};
use bytemuck::{Pod, Zeroable};
use ggrs::{InputStatus, P2PSession, PlayerHandle};

mod player;
mod rollback;
pub use player::*;
pub use rollback::*;

const BLUE: Color = Color::rgb(0.8, 0.6, 0.2);
const ORANGE: Color = Color::rgb(0., 0.35, 0.8);
const MAGENTA: Color = Color::rgb(0.9, 0.2, 0.2);
const GREEN: Color = Color::rgb(0.35, 0.7, 0.35);
const PLAYER_COLORS: [Color; 4] = [BLUE, ORANGE, MAGENTA, GREEN];

#[derive(Component)]
pub struct RoundEntity;

#[derive(Default, Reflect, Hash, Component)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
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
    let r = 720. / 4.;

    for (handle, color) in PLAYER_COLORS.iter().enumerate().take(NUM_PLAYERS) {
        let rot = handle as f32 / NUM_PLAYERS as f32 * 2. * std::f32::consts::PI;
        let x = r * rot.cos();
        let y = r * rot.sin();

        let transform = Transform::from_translation(Vec3::new(x, y, 1.));

        let gun = commands
            .spawn_bundle(SpriteBundle {
                transform,
                texture: textures.gun.clone(),
                sprite: Sprite {
                    color: PLAYER_COLORS[handle],
                    custom_size: Some(Vec2::new(15., 15.)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();

        commands
            .spawn_bundle(SpriteSheetBundle {
                transform,
                texture_atlas: textures.tiles_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: 3,
                    color: *color,
                    custom_size: Some(Vec2::new(
                        player_settings::DEFAULT_PLAYER_SIZE,
                        player_settings::DEFAULT_PLAYER_SIZE,
                    )),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player::new(handle))
            .insert(PlayerControls::default())
            .insert(Checksum::default())
            .insert(Rollback::new(rip.next_id()))
            .insert(RoundEntity)
            .add_child(gun);
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
