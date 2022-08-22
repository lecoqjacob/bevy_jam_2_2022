// This is the folder to handle the `round` or `in_game` state

use crate::prelude::*;
use bevy::{math::Vec3Swizzles, sprite::MaterialMesh2dBundle};
use bevy_ggrs::{Rollback, RollbackIdProvider, SessionType};
use bytemuck::{Pod, Zeroable};
use ggrs::{InputStatus, P2PSession, PlayerHandle};

mod bullet;
mod creature;
mod input;
mod player;
mod ui;

pub use bullet::*;
pub use creature::*;
pub use input::*;
pub use player::*;
pub use ui::*;

const BLUE: Color = Color::rgb(0.8, 0.6, 0.2);
const ORANGE: Color = Color::rgb(0., 0.35, 0.8);
const MAGENTA: Color = Color::rgb(0.9, 0.2, 0.2);
const GREEN: Color = Color::rgb(0.35, 0.7, 0.35);
const PLAYER_COLORS: [Color; 4] = [BLUE, ORANGE, MAGENTA, GREEN];

#[derive(Component)]
pub struct RoundEntity;

pub fn setup_round(mut commands: Commands) {
    commands.init_resource::<FrameCount>();
    commands.init_resource::<CacheGrid>();
}

pub fn spawn_players(
    mut commands: Commands,
    rng: Res<RandomNumbers>,
    settings: Res<MapSettings>,
    textures: Res<TextureAssets>,
    mut rip: ResMut<RollbackIdProvider>,
    player_query: Query<Entity, With<Player>>,
    bullet_query: Query<Entity, With<Bullet>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Spawning players");

    for player in player_query.iter() {
        commands.entity(player).despawn_recursive();
    }
    for bullet in bullet_query.iter() {
        commands.entity(bullet).despawn_recursive();
    }

    let mut follow_player: Option<Entity> = None;
    for (handle, color) in PLAYER_COLORS.iter().enumerate().take(NUM_PLAYERS) {
        let transform = Transform::default().with_translation(Vec3::new(0.0, 0.0, 10.0));

        let player_comp = Player::new(handle);
        let player = commands
            .spawn_bundle(SpriteSheetBundle {
                transform,
                texture_atlas: textures.tiles_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    color: *color,
                    custom_size: Some(Vec2::new(player_comp.size, player_comp.size)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(player_comp)
            .insert(PlayerControls::default())
            .insert(Checksum::default())
            .insert(Rollback::new(rip.next_id()))
            .insert(BulletReady(true))
            .insert(RoundEntity)
            .id();

        let gun = commands
            .spawn_bundle(SpriteSheetBundle {
                transform: transform.with_translation(Vec3::new(0., 10., 5.)),
                texture_atlas: textures.tiles_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: 3,
                    color: *color,
                    custom_size: Some(Vec2::new(5., 15.)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RoundEntity)
            .id();

        let ring_color = Color::rgba(color.r(), color.g(), color.b(), 0.2);
        let ring = commands
            .spawn_bundle(MaterialMesh2dBundle {
                transform,
                mesh: meshes.add(shape::Circle::new(100.).into()).into(),
                material: materials.add(ColorMaterial::from(ring_color)),
                ..default()
            })
            .insert(CollectionRing)
            .insert(RoundEntity)
            .id();

        commands.entity(player).add_child(gun).add_child(ring);

        if follow_player.is_none() {
            follow_player = Some(player);
        }
    }

    for _ in 0..5 {
        let direction_vector =
            Vec2::new(rng.rand::<f32>() * 2.0 - 1.0, rng.rand::<f32>() * 2.0 - 1.0).normalize();

        let width = settings.width / 2.;
        let height = settings.height / 2.;
        let x = rng.range(-width, width);
        let y = rng.range(-height, height);

        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(5.0, 10.0)),
                    ..Sprite::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, 5.0),
                    rotation: Quat::from_rotation_z(-direction_vector.x.atan2(direction_vector.y)),
                    ..Transform::default()
                },
                ..SpriteBundle::default()
            })
            .insert(crate::components::Direction(direction_vector))
            .insert(Creature(1))
            // .insert(CreatureTarget(0, follow_player.unwrap()))
            .insert(RoundEntity);
    }
}

pub fn print_p2p_events(mut session: ResMut<P2PSession<GGRSConfig>>) {
    for event in session.events() {
        info!("GGRS Event: {:?}", event);
    }
}

pub fn check_win(mut commands: Commands, player: Query<&Player>) {
    let players = player.iter().count();

    if players < NUM_PLAYERS {
        commands.insert_resource(NextState(AppState::Win));
        commands.insert_resource(MatchData { result: "Orange won!".to_owned() });
    }
}

pub fn cleanup_round(mut commands: Commands) {
    commands.remove_resource::<CacheGrid>();
    commands.remove_resource::<FrameCount>();
    commands.remove_resource::<SessionType>();
    commands.remove_resource::<LocalHandles>();
    commands.remove_resource::<P2PSession<GGRSConfig>>();
}

pub struct RoundPlugin;
impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RoundUIPlugin);
        app.add_event::<ApplyForceEvent>();

        // Local
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
