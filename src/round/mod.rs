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

pub fn setup_round(mut commands: Commands) {
    commands.init_resource::<FrameCount>();
    commands.init_resource::<CacheGrid>();
}

pub fn spawn_players(
    mut commands: Commands,
    rng: Res<RandomNumbers>,
    settings: Res<MapSettings>,
    meshes: Res<MeshAssets>,
    materials: Res<MaterialAssets>,
    mut rip: ResMut<RollbackIdProvider>,
    player_query: Query<Entity, With<Player>>,
    bullet_query: Query<Entity, With<Bullet>>,
) {
    info!("Spawning players");

    for player in player_query.iter() {
        commands.entity(player).despawn_recursive();
    }
    for bullet in bullet_query.iter() {
        commands.entity(bullet).despawn_recursive();
    }

    for (handle, color) in player_settings::PLAYER_COLORS.iter().enumerate().take(NUM_PLAYERS) {
        let transform = Transform::default().with_translation(Vec3::new(0.0, 0.0, 10.0));

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

    for _ in 0..20 {
        let map_width = settings.width / 2.;
        let map_height = settings.height / 2.;
        let x = rng.range(-map_width, map_width);
        let y = rng.range(-map_height, map_height);

        let size = creature_settings::DEFAULT_CREATURE_SIZE.0;
        spawn_zombie(&mut commands, &mut rip, &rng, x, y, size);
    }
}

pub fn respawn_players(
    time: Res<Time>,
    mut commands: Commands,
    meshes: Res<MeshAssets>,
    materials: Res<MaterialAssets>,
    mut rip: ResMut<RollbackIdProvider>,
    mut respawns: Query<(Entity, &mut Respawn)>,
) {
    for (ent, mut respawn) in &mut respawns {
        respawn.time -= time.delta_seconds();

        if respawn.time <= 0.0 {
            commands.entity(ent).despawn_recursive();

            let transform = Transform::default().with_translation(Vec3::new(0.0, 0.0, 10.0));

            spawn_player(
                &mut commands,
                &mut rip,
                transform,
                respawn.handle,
                respawn.color,
                meshes.ring.clone(),
                materials.get(respawn.color),
            );
        }
    }
}

pub fn print_p2p_events(mut session: ResMut<P2PSession<GGRSConfig>>) {
    for event in session.events() {
        info!("GGRS Event: {:?}", event);
    }
}

pub fn check_win(mut commands: Commands, player: Query<&Player>) {
    // let players = player.iter().count();

    // if players < NUM_PLAYERS {
    //     commands.insert_resource(NextState(AppState::Win));
    //     commands.insert_resource(MatchData { result: "Orange won!".to_owned() });
    // }
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
