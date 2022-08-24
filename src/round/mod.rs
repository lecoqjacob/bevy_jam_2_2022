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

#[derive(Component)]
pub struct RoundEntity;

pub fn setup_round(
    mut commands: Commands,
    rng: Res<RandomNumbers>,
    meshes: Res<MeshAssets>,
    settings: Res<MapSettings>,
    materials: Res<MaterialAssets>,
    mut rip: ResMut<RollbackIdProvider>,
    player_query: Query<Entity, With<Player>>,
    bullet_query: Query<Entity, With<Bullet>>,
) {
    commands.init_resource::<FrameCount>();
    commands.init_resource::<CacheGrid>();

    for player in player_query.iter() {
        commands.entity(player).despawn_recursive();
    }
    for bullet in bullet_query.iter() {
        commands.entity(bullet).despawn_recursive();
    }

    let transform = Transform::default().with_translation(Vec3::new(0.0, 0.0, 10.0));
    for (handle, color) in player_settings::PLAYER_COLORS.iter().enumerate().take(NUM_PLAYERS) {
        spawn_player(
            &mut commands,
            &mut rip,
            transform,
            handle,
            *color,
            meshes.ring.clone(),
            materials.get(*color),
        );
    }

    for _ in 0..50 {
        let (x, y) = random_map_point(settings.width, settings.height, &rng);
        let direction_vector =
            Vec2::new(rng.rand::<f32>() * 2.0 - 1.0, rng.rand::<f32>() * 2.0 - 1.0).normalize();
        let transform = Transform::default()
            .with_translation(Vec3::new(x, y, 10.0))
            .with_rotation(Quat::from_rotation_z(-direction_vector.x.atan2(direction_vector.y)));

        let size = creature_settings::DEFAULT_CREATURE_SIZE.0;
        spawn_zombie(&mut commands, &mut rip, transform, direction_vector, size);
    }
}

pub fn random_map_point(width: f32, height: f32, rng: &RandomNumbers) -> (f32, f32) {
    let map_width = width / 2.;
    let map_height = height / 2.;
    let x = rng.range(-map_width, map_width);
    let y = rng.range(-map_height, map_height);

    (x, y)
}

pub fn spawning(
    mut commands: Commands,
    rng: Res<RandomNumbers>,
    mut evs: EventReader<SpawnEvent>,
    mut rip: ResMut<RollbackIdProvider>,
) {
    for SpawnEvent { point, handle: _, color: _, spawn_type } in evs.iter() {
        match spawn_type {
            RespawnType::Zombie => {
                let direction_vector =
                    Vec2::new(rng.rand::<f32>() * 2.0 - 1.0, rng.rand::<f32>() * 2.0 - 1.0)
                        .normalize();

                let point = point.unwrap();
                let transform = Transform::default()
                    .with_translation(Vec3::new(point.0, point.1, 10.0))
                    .with_rotation(Quat::from_rotation_z(
                        -direction_vector.x.atan2(direction_vector.y),
                    ));

                let size = creature_settings::DEFAULT_CREATURE_SIZE.0;
                spawn_zombie(&mut commands, &mut rip, transform, direction_vector, size);
            }
            // RespawnType::Player => {
            //     let transform = Transform::default().with_translation(Vec3::new(0.0, 0.0, 10.0));
            //     let (handle, color) = (handle.unwrap(), color.unwrap());
            //     spawn_player(
            //         &mut commands,
            //         &mut rip,
            //         transform,
            //         handle,
            //         color,
            //         meshes.ring.clone(),
            //         materials.get(color),
            //     );
            // }
            _ => info!("Player Spawning not supported: {:?}", spawn_type),
        }
    }
}

pub fn spawn_creatures(
    mut timer: Local<f32>,
    rng: Res<RandomNumbers>,
    settings: Res<MapSettings>,
    mut spawn_event: EventWriter<SpawnEvent>,
    total_creature_follow: Query<&CreatureFollow>,
) {
    let count = total_creature_follow.iter().count();
    if count < 100 {
        *timer += TIME_STEP;

        if *timer >= 15. {
            *timer = 0.0;

            let point = random_map_point(settings.width, settings.height, &rng);
            spawn_event.send(SpawnEvent {
                color: None,
                handle: None,
                point: Some(point),
                spawn_type: RespawnType::Zombie,
            });
        }
    }
}

pub fn print_p2p_events(mut session: ResMut<P2PSession<GGRSConfig>>) {
    for event in session.events() {
        info!("GGRS Event: {:?}", event);
    }
}

pub fn check_win(mut commands: Commands, player: Query<&Player, Changed<Player>>) {
    for p in player.iter() {
        let count = p.active_zombies.len();
        if count >= 25 {
            commands.insert_resource(NextState(AppState::Win));
            commands.insert_resource(MatchData { result: format!("Player {:?} won!", p.handle) });
        }
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
        app.add_event::<ApplyForceEvent>();
        app.add_event::<SpawnEvent>();

        app.add_plugin(RoundUIPlugin);

        // Local
        app.add_enter_system_set(
            AppState::RoundLocal,
            ConditionSet::new().with_system(setup_round).into(),
        )
        .add_system_set(
            ConditionSet::new().run_in_state(AppState::RoundLocal).with_system(check_win).into(),
        )
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
            ConditionSet::new().with_system(setup_round).into(),
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
